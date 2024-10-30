use actix_web::{post, web};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use redis::Commands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AuthError;
use crate::models::token::Token;
use crate::models::user::{NewUser, User};
use crate::DbPool;

const NO_PUBLIC_REGISTRATION: bool = match option_env!("NO_PUBLIC_REGISTRATION") {
    Some(_) => true, // s == "" || s == "true" || s == "True"|| s == "t" || s == "T" || s == "1",
    None => false,
};

#[derive(Deserialize, Serialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[post("/token")]
async fn token(
    pool: web::Data<DbPool>,
    form: web::Json<TokenRequest>,
) -> Result<web::Json<Token>, AuthError> {
    let mut conn = pool.get()?;

    if form.grant_type == "password" {
        if let (Some(username_s), Some(password)) = (&form.username, &form.password) {
            use crate::schema::users::dsl::*;

            // Verify user credentials
            let maybe_user: Option<User> = users
                .find(username_s)
                .select(User::as_select())
                .first(&mut conn)
                .optional()?;

            /*
            hmm, this doesn't seem to have a `RETURNING` syntax:

            diesel::insert_into(users)
                .values(NewUser{username: username_s, password_hash: password})
                .on_conflict(username)
                .do_nothing()
                .execute(&mut conn)?;
            */

            return match maybe_user {
                Some(user) => {
                    if Argon2::default()
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
                        let gen_password_hash = Argon2::default()
                            .hash_password(password.as_ref(), &salt)?
                            .to_string();

                        let user = diesel::insert_into(users)
                            .values(&NewUser {
                                username: username_s,
                                password_hash: gen_password_hash.as_str(),
                            })
                            .returning(User::as_returning())
                            .get_result(&mut conn)?;
                        generate_tokens(username_s, user.role.as_str())
                    }
                }
            };
        }
    }

    Err(AuthError::BadRequest {
        mime: mime::APPLICATION_JSON,
        body: serde_json::json!({"error": "invalid_grant"}).to_string(),
    })
}

fn generate_tokens(username_s: &str, role: &str) -> Result<web::Json<Token>, AuthError> {
    // Generate and return an access token
    let access_token = Uuid::new_v4().to_string();
    let expires_in = 3600; // token expiry in seconds

    // TODO: Connection pool for redis instantiated same time as PostgreSQL
    let client = redis::Client::open(
        std::env::var("REDIS_URL").unwrap_or(String::from("redis://127.0.0.1/")),
    )?;
    let mut con = client.get_connection()?;
    let fully_qualified_key = format!("{username_s}::{role}::access_token::{access_token}");

    let _: () = con.set_ex(&fully_qualified_key, expires_in, expires_in)?;

    /**/
    Ok(web::Json(Token {
        access_token: fully_qualified_key,
        expires_in,
        token_type: String::from("Bearer"),
    }))
}
