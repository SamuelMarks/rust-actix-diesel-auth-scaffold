use actix_web::mime;
use argon2::{PasswordHasher, PasswordVerifier};
use base64::{prelude::BASE64_STANDARD, Engine};
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use redis::Commands;

use crate::errors::AuthError;
use crate::models::token::Token;
use crate::models::users::{CreateUsers, Users};
use crate::routes::token::types::{TokenRequest, NO_PUBLIC_REGISTRATION};
use crate::DbConnectionManager;

fn generate_tokens(username_s: &str, role: &str) -> Result<actix_web::web::Json<Token>, AuthError> {
    // Generate and return an access token
    let access_token = uuid::Uuid::new_v4().to_string();
    let refresh_token = uuid::Uuid::new_v4().to_string();
    let expires_in: u64 = std::env::var("RADAS_EXPIRES_IN")
        .unwrap_or(String::from("2628000")) // token expiry in seconds [1 month]
        .parse()
        .expect("RADAS_EXPIRES_IN that parses into u64");
    let rt_expires_in: u64 = std::env::var("RADAS_RT_EXPIRES_IN")
        .unwrap_or(String::from("15768000")) // token expiry in seconds [6 months]
        .parse()
        .expect("RADAS_RT_EXPIRES_IN that parses into u64");

    // TODO: Connection pool for redis instantiated same time as PostgreSQL
    let client = redis::Client::open(
        std::env::var("REDIS_URL").unwrap_or(String::from("redis://127.0.0.1/")),
    )?;
    let mut con = client.get_connection()?;
    let fully_qualified_at = format!("{username_s}::{role}::access_token::{access_token}");
    let fully_qualified_rt = format!("{username_s}::{role}::refresh_token::{refresh_token}");

    let _: () = con.set_ex(&fully_qualified_at, expires_in, expires_in)?;
    let _: () = con.set_ex(&fully_qualified_rt, rt_expires_in, rt_expires_in)?;

    Ok(actix_web::web::Json(Token {
        access_token: fully_qualified_at,
        refresh_token: fully_qualified_rt,
        expires_in,
        token_type: String::from("Bearer"),
    }))
}

fn verify_or_insert_creds_and_get_role(
    conn: &mut diesel::r2d2::PooledConnection<DbConnectionManager>,
    username_s: &str,
    password: &str,
) -> Result<String, AuthError> {
    use crate::schema::users::dsl::*;

    // Verify user credentials
    let maybe_user: Option<Users> = users.find(username_s).first(conn).optional()?;

    /*
    hmm, this doesn't seem to have a `RETURNING` syntax:

    diesel::insert_into(users)
        .values(NewUser{username: username_s, password_hash: password})
        .on_conflict(username)
        .do_nothing()
        .execute(&mut conn)?;
    */

    match maybe_user {
        Some(user) => {
            if argon2::Argon2::default()
                .verify_password(
                    password.as_ref(),
                    &argon2::PasswordHash::parse(
                        user.password_hash.as_str(),
                        argon2::password_hash::Encoding::default(),
                    )?,
                )
                .is_ok()
            {
                Ok(user.role)
            } else {
                Err(AuthError::Unauthorised("User"))
            }
        }
        None => {
            if NO_PUBLIC_REGISTRATION {
                Err(AuthError::NotFound("User"))
            } else {
                let salt = argon2::password_hash::SaltString::try_from_rng(
                    &mut argon2::password_hash::rand_core::OsRng,
                )?;
                let gen_password_hash = argon2::Argon2::default()
                    .hash_password(password.as_ref(), &salt)?
                    .to_string();

                let user = diesel::insert_into(users)
                    .values(&CreateUsers {
                        username: String::from(username_s),
                        password_hash: gen_password_hash,
                        ..CreateUsers::default()
                    })
                    .returning(Users::as_returning())
                    .get_result(conn)?;
                Ok(user.role)
            }
        }
    }
}

pub(crate) fn handle_grant_flow_for_password(
    conn: &mut diesel::r2d2::PooledConnection<DbConnectionManager>,
    token_request: &TokenRequest,
) -> Result<actix_web::web::Json<Token>, AuthError> {
    if let (Some(username_s), Some(password)) = (&token_request.username, &token_request.password) {
        let role = verify_or_insert_creds_and_get_role(conn, username_s, password)?;
        generate_tokens(username_s, role.as_str())
    } else {
        Err(AuthError::BadRequest {
            mime: mime::APPLICATION_JSON,
            body: serde_json::json!({
                "error": "invalid_grant",
                "error_message": "username and/or password missing"
            })
            .to_string(),
        })
    }
}

pub(crate) fn handle_grant_flow_for_refresh_token(
    token_request: TokenRequest,
) -> Result<actix_web::web::Json<Token>, AuthError> {
    if let Some(refresh_token) = token_request.refresh_token {
        if let Ok(client) = redis::Client::open(
            std::env::var("REDIS_URL").unwrap_or(String::from("redis://127.0.0.1/")),
        ) {
            if let Ok(mut conn) = client.get_connection() {
                if let Ok(false) = conn.exists(&refresh_token) {
                    return Err(AuthError::Unauthorised("refresh_token"));
                }
            }
        }

        let mut split = refresh_token.split("::");
        let username = split.next().unwrap();
        let role = split.next().unwrap();
        let token_type = split.next().unwrap();
        // token uuid is still in `split` but not needed
        if token_type != "refresh_token" {
            Err(AuthError::BadRequest {
                mime: mime::APPLICATION_JSON,
                body: serde_json::json!({
                    "error": "invalid_token_type",
                    "error_message": format!("token_type is '{token_type}' expected 'refresh_token'")
                })
                .to_string(),
            })
        } else {
            generate_tokens(username, role)
        }
    } else {
        Err(AuthError::BadRequest {
            mime: mime::APPLICATION_JSON,
            body: serde_json::json!({
                "error": "invalid_grant",
                "error_message": "refresh_token missing"
            })
            .to_string(),
        })
    }
}

// actix_web_httpauth::headers::authorization::Basic
#[derive(derive_more::Debug)]
pub struct Basic {
    user_id: std::borrow::Cow<'static, str>,
    password: Option<std::borrow::Cow<'static, str>>,
}

fn parse_authorization_basic(
    header: &actix_web::http::header::HeaderValue,
) -> Result<Basic, actix_web_httpauth::headers::authorization::ParseError> {
    // "Basic *" length
    if header.len() < 7 {
        return Err(actix_web_httpauth::headers::authorization::ParseError::Invalid);
    }

    let mut parts = header.to_str()?.splitn(2, ' ');
    match parts.next() {
        Some("Basic") => (),
        _ => return Err(actix_web_httpauth::headers::authorization::ParseError::MissingScheme),
    }

    let decoded = BASE64_STANDARD.decode(
        parts
            .next()
            .ok_or(actix_web_httpauth::headers::authorization::ParseError::Invalid)?,
    )?;
    let mut credentials = std::str::from_utf8(&decoded)?.splitn(2, ':');

    let user_id = credentials
        .next()
        .ok_or(actix_web_httpauth::headers::authorization::ParseError::MissingField("user_id"))
        .map(|user_id| user_id.to_string().into())?;

    let password = credentials
        .next()
        .ok_or(actix_web_httpauth::headers::authorization::ParseError::MissingField("password"))
        .map(|password| {
            if password.is_empty() {
                None
            } else {
                Some(password.to_string().into())
            }
        })?;

    Ok(Basic { user_id, password })
}

pub(crate) fn handle_grant_flow_for_authorization_code(
    conn: &mut diesel::r2d2::PooledConnection<DbConnectionManager>,
    headers: &actix_http::header::HeaderMap,
    token_request: TokenRequest,
) -> Result<actix_web::web::Json<Token>, AuthError> {
    if let TokenRequest {
        client_id: Some(client_id),
        redirect_uri: Some(redirect_uri),
        code: Some(code),
        ..
    } = token_request
    {
        println!(
            "TODO: check `{client_id}` is same that issued `{code}` and that `code` is valid \
        and also that {redirect_uri} is valid and matches that that issued code"
        );
        // TODO: check `client_id` is same that issued `code` and that `code` is valid
        if let Some(authorization) = headers.get("Authorization") {
            let basic = parse_authorization_basic(authorization)?;
            if let Basic {
                user_id: userid,
                password: Some(password),
            } = basic
            {
                let role = verify_or_insert_creds_and_get_role(conn, &userid, &password)?;
                return generate_tokens(&userid, role.as_str());
            }
        }
    }
    Err(AuthError::Unauthorised(
        "client_id, redirect_uri, code, basic authorization must be set",
    ))
}
