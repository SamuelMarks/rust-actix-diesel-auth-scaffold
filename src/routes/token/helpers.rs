use argon2::{PasswordHasher, PasswordVerifier};
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use redis::Commands;

use crate::errors::AuthError;
use crate::models::token::Token;
use crate::models::user::{NewUser, User};
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

pub(crate) fn handle_grant_flow_for_password(
    conn: &mut diesel::r2d2::PooledConnection<DbConnectionManager>,
    token_request: &TokenRequest,
) -> Result<actix_web::web::Json<Token>, AuthError> {
    if let (Some(username_s), Some(password)) = (&token_request.username, &token_request.password) {
        use crate::schema::users::dsl::*;

        // Verify user credentials
        let maybe_user: Option<User> = users
            .find(username_s)
            .select(User::as_select())
            .first(conn)
            .optional()?;

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
                    generate_tokens(username_s.as_str(), user.role.as_str())
                } else {
                    Err(AuthError::Unauthorised("User"))
                }
            }
            None => {
                if NO_PUBLIC_REGISTRATION {
                    Err(AuthError::NotFound("User"))
                } else {
                    let salt = argon2::password_hash::SaltString::generate(
                        &mut argon2::password_hash::rand_core::OsRng,
                    );
                    let gen_password_hash = argon2::Argon2::default()
                        .hash_password(password.as_ref(), &salt)?
                        .to_string();

                    let user = diesel::insert_into(users)
                        .values(&NewUser {
                            username: username_s,
                            password_hash: gen_password_hash.as_str(),
                        })
                        .returning(User::as_returning())
                        .get_result(conn)?;
                    generate_tokens(username_s, user.role.as_str())
                }
            }
        }
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
