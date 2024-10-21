use actix_web::{web, post};
use argon2::{Argon2, PasswordVerifier};
use serde::Deserialize;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use redis::Commands;
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
            use crate::schema::user::dsl::*;

            // Verify user credentials
            let user: QueryResult<User> = user.filter(username.eq(username_s)).first(&mut conn);

            match user {
                Ok(user) => {
                    if Argon2::default().verify_password(password.as_ref(), &user.password_hash).is_ok() {
                        // Generate and return an access token
                        let access_token = Uuid::new_v4().to_string();
                        let expires_in = 3600; // token expiry in seconds

                        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
                        let mut con = client.get_connection()?;

                        let _: () = con.lpush(format!("{}::access_tokens", username_s), &access_token).await?;

                        return Ok(web::Json(Token {
                            access_token,
                            expires_in,
                            token_type: String::from("Bearer"),
                        }));
                    }
                }
                Err(_) => {}
            }
        }
    }

    Err(AuthError::BadRequest {
        mime: mime::APPLICATION_JSON,
        body: serde_json::json!({"error": "invalid_grant"}).to_string(),
    })
}
