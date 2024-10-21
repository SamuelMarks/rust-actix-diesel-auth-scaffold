use actix_web::{post, web};
use argon2::{Argon2, PasswordVerifier};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use redis::Commands;
use serde::Deserialize;
use uuid::Uuid;

use crate::errors::AuthError;
use crate::models::token::Token;
use crate::models::user::User;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct TokenRequest {
    grant_type: String,
    username: Option<String>,
    password: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
}

#[post("/token")]
async fn token(
    pool: web::Data<DbPool>,
    form: web::Form<TokenRequest>,
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
                        // Generate and return an access token
                        let access_token = Uuid::new_v4().to_string();
                        let expires_in = 3600; // token expiry in seconds

                        let client = redis::Client::open(
                            std::env::var("REDIS_URL")
                                .unwrap_or(String::from("redis://127.0.0.1/")),
                        )?;
                        let mut con = client.get_connection()?;

                        let _: () =
                            con.lpush(format!("{}::access_tokens", username_s), &access_token)?;

                        return Ok(web::Json(Token {
                            access_token,
                            expires_in,
                            token_type: String::from("Bearer"),
                        }));
                    }
                }
                None => return Err(AuthError::NotFound("User")),
            }
        }
    }

    Err(AuthError::BadRequest {
        mime: mime::APPLICATION_JSON,
        body: serde_json::json!({"error": "invalid_grant"}).to_string(),
    })
}
