use actix_web::{post, web};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use diesel::r2d2::{self, ConnectionManager};
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use redis::Commands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AuthError;
use crate::models::token::Token;
use crate::models::user::{NewUser, User};

type DbPool = r2d2::Pool<ConnectionManager<diesel::PgConnection>>;
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

            match maybe_user {
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
                        return generate_tokens(username_s);
                    } else {
                        return Err(AuthError::Unauthorised("User"));
                    }
                }
                None => {
                    return if NO_PUBLIC_REGISTRATION {
                        Err(AuthError::NotFound("User"))
                    } else {
                        let salt = argon2::password_hash::SaltString::generate(
                            &mut argon2::password_hash::rand_core::OsRng,
                        );
                        let gen_password_hash = Argon2::default()
                            .hash_password(password.as_ref(), &salt)?
                            .to_string();

                        let rows = diesel::insert_into(users)
                            .values(&NewUser {
                                username: username_s,
                                password_hash: gen_password_hash.as_str(),
                            })
                            .execute(&mut conn)?;
                        if rows != 1 {
                            Err(AuthError::BadRequest {
                                mime: mime::APPLICATION_JSON,
                                body: String::from("User insert affected rows not equal to 1"),
                            })
                        } else {
                            generate_tokens(username_s)
                        }
                    }
                }
            }
        }
    }

    Err(AuthError::BadRequest {
        mime: mime::APPLICATION_JSON,
        body: serde_json::json!({"error": "invalid_grant"}).to_string(),
    })
}

fn generate_tokens(username_s: &String) -> Result<web::Json<Token>, AuthError> {
    // Generate and return an access token
    let access_token = Uuid::new_v4().to_string();
    let expires_in = 3600; // token expiry in seconds

    let client = redis::Client::open(
        std::env::var("REDIS_URL").unwrap_or(String::from("redis://127.0.0.1/")),
    )?;
    let mut con = client.get_connection()?;

    let _: () = con.lpush(format!("{}::access_tokens", username_s), &access_token)?;

    /**/
    Ok(web::Json(Token {
        access_token,
        expires_in,
        token_type: String::from("Bearer"),
    }))
}
