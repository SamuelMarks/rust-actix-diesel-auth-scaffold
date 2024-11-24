use crate::errors::AuthError;
use crate::models::token::Token;
use crate::routes::token::helpers::{
    handle_grant_flow_for_authorization_code, handle_grant_flow_for_password,
    handle_grant_flow_for_refresh_token,
};
use crate::routes::token::types::GrantType;
use crate::routes::token::types::TokenRequest;
use actix_http::header::{Header, TryIntoHeaderPair};
use actix_web::post;
use actix_web_httpauth::headers::authorization::Basic;

/// Generate a token for a grant flow.
/// Implements https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.3
#[utoipa::path(
    responses(
        (status = 200, description = "Token created"),
        (status = 400, description = "Unauthorized User"),
        (status = 404, description = "Not Found User"),
        (status = 500, description = "Bad Request")
    )
)]
#[post("/token")]
pub async fn token(
    req: actix_web::HttpRequest,
    pool: actix_web::web::Data<crate::DbPool>,
    form: actix_web::Either<actix_web::web::Json<TokenRequest>, actix_web::web::Form<TokenRequest>>,
) -> Result<actix_web::web::Json<Token>, AuthError> {
    let mut conn = pool.get()?;
    let token_request = form.into_inner();

    match token_request.grant_type {
        GrantType::Password => handle_grant_flow_for_password(&mut conn, &token_request),
        GrantType::RefreshToken => handle_grant_flow_for_refresh_token(token_request),
        GrantType::AuthorizationCode => {
            let auth =
                actix_web_httpauth::headers::authorization::Authorization::<Basic>::parse(&req)?;
            println!("auth: {}", auth);

            handle_grant_flow_for_authorization_code(&mut conn, req.headers(), token_request)
        }
        GrantType::ClientCredentials | GrantType::Invalid => Err(AuthError::BadRequest {
            mime: mime::APPLICATION_JSON,
            body: serde_json::json!({"error": "unimplemented"}).to_string(),
        }),
    }
}
