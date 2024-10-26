use actix_web::{post, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::errors::AuthError;

#[post("/logout")]
pub async fn logout(credentials: BearerAuth) -> Result<impl Responder, AuthError> {
    if let Some((token_namespace, _)) = credentials.token().rsplit_once("::") {
        if let Ok(client) = redis::Client::open(
            std::env::var("REDIS_URL").unwrap_or(String::from("redis://127.0.0.1/")),
        ) {
            if let Ok(mut conn) = client.get_connection() {
                let script = redis::Script::new(&format!("for _,k in ipairs(redis.call('keys','{token_namespace}::*')) do redis.call('del',k) end"));
                let _: () = script /*.arg(token_namespace)*/
                    .invoke(&mut conn)?;
            }
        }
    }
    Ok(HttpResponse::Ok())
}
