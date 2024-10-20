use actix_web::{web, HttpResponse, Responder, post};
use argon2::{Argon2, PasswordVerifier};
use serde::Deserialize;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use redis::Commands;
use uuid::Uuid;

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
    form: web::Form<TokenRequest>
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

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

                        let _: () = con.lpush(format!("{}::access_tokens", username_s), access_token).await?;

                        return HttpResponse::Ok().json(serde_json::json!({
                            "access_token": access_token,
                            "token_type": "Bearer",
                            "expires_in": expires_in
                        }));
                    }
                },
                Err(_) => {}
            }
        }
    }

    HttpResponse::BadRequest().json(serde_json::json!({
        "error": "invalid_grant"
    }))
}
