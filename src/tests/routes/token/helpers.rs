#[cfg(test)]
pub(crate) const USERNAMES: [&'static str; 2] = ["username0", "username1"];

#[macro_export]
macro_rules! get_token_app {
    () => {
        actix_web::test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(crate::POOL.clone()))
                .service(
                    actix_web::web::scope("/api")
                        .service(crate::routes::token::token)
                        .service(crate::routes::authorisation::authorise),
                ),
        )
    };
}

pub mod test_token_api {
    use crate::routes::token::types::{GrantType, TokenRequest};

    pub fn post(token_request: TokenRequest) -> actix_http::Request {
        actix_web::test::TestRequest::post()
            .uri("/api/token")
            .set_json(token_request)
            .to_request()
    }

    pub fn post_username_password(username: &str, password: &str) -> actix_http::Request {
        post(TokenRequest {
            grant_type: GrantType::Password,
            username: Some(String::from(username)),
            password: Some(String::from(password)),
            ..TokenRequest::default()
        })
    }
}
