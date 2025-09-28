use actix_web::{post, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use redis::Commands;

use crate::errors::{AuthError, AuthErrorSchema};

fn t() -> bool {
    true
}

#[derive(serde::Deserialize)]
struct LogoutParams {
    #[serde(default = "t")]
    all_devices: bool,
}

/// Logout a user (uses provided Bearer token from Header)
#[utoipa::path(
    responses(
        (status = 200, description = "Empty response"),
        (status = 500, description = "Error response", body = AuthErrorSchema),
    ),
    security(("password"=[]))
)]
#[post("/logout")]
pub async fn logout(
    params: actix_web::web::Query<LogoutParams>,
    credentials: BearerAuth,
) -> Result<impl Responder, AuthError> {
    let token = credentials.token();
    let split = token.find("::").unwrap();
    let username = &token[..split];

    if let Ok(client) = redis::Client::open(
        std::env::var("REDIS_URL").unwrap_or(String::from("redis://127.0.0.1/")),
    ) {
        if let Ok(mut conn) = client.get_connection() {
            let _: () = if params.all_devices {
                let script = redis::Script::new(&format!("for _,k in ipairs(redis.call('keys','{username}::*::*_token::*')) do redis.call('del',k) end"));
                script /*.arg(token_namespace)*/
                    .invoke(&mut conn)?
            } else {
                conn.del(token)?
            };
        }
        Ok(HttpResponse::Ok())
    } else {
        Err(AuthError::RedisError {
            error: redis::RedisError::from((
                redis::ErrorKind::TryAgain,
                "Unable to get connection",
            )),
        })
    }
}
