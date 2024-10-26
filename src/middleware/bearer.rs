/// Construct `HttpAuthentication` middleware for the HTTP "Bearer" authentication scheme.
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};
use redis::Commands;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // req isn't cloneable so let's write bad code that doesn't inform on err:
    if let Ok(client) = redis::Client::open(
        std::env::var("REDIS_URL").unwrap_or(String::from("redis://127.0.0.1/")),
    ) {
        if let Ok(mut conn) = client.get_connection() {
            if let Ok(true) = conn.exists(&credentials.token()) {
                return Ok(req);
            }
        }
    }

    let config = req
        .app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default();

    Err((AuthenticationError::from(config).into(), req))
}
