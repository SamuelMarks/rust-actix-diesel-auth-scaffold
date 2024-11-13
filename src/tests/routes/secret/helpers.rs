pub(crate) const USERNAMES: [&'static str; 2] = ["username0", "username1"];
pub(crate) const PASSWORD: &'static str = "password";

#[macro_export]
macro_rules! get_secret_app {
    () => {
        actix_web::test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(crate::POOL.clone()))
                .service(
                    actix_web::web::scope("/api/v0")
                        .wrap(actix_web::middleware::Compat::new(
                            actix_web_httpauth::middleware::HttpAuthentication::bearer(
                                crate::middleware::bearer::validator,
                            ),
                        ))
                        .service(crate::routes::secret::secret),
                ),
        )
    };
}

static INIT: std::sync::Once = std::sync::Once::new();

pub(crate) async fn prepare_secret_test(username: &str, password: &str) -> String {
    crate::establish_connection().unwrap();
    INIT.call_once(|| {
        crate::db_init();
    });

    let token = crate::get_token(String::from(username), String::from(password)).await;
    token
}

pub(crate) mod test_secret_api {
    pub(crate) fn post(token: &str) -> actix_http::Request {
        actix_web::test::TestRequest::get()
            .uri("/api/v0/secret")
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }
}
