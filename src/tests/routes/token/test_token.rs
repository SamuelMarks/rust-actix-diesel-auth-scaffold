#[actix_web::test]
async fn test_read_token() {
    crate::tests::init_db_for_test();
    const USERNAME: &'static str = crate::tests::routes::token::helpers::USERNAMES[0];
    let app = crate::get_token_app!().await;
    let req = crate::tests::routes::token::helpers::test_token_api::post(
        crate::routes::token::TokenRequest {
            grant_type: String::from("password"),
            username: Some(String::from(USERNAME)),
            password: Some(String::from(
                crate::tests::routes::secret::helpers::PASSWORD,
            )),
            client_id: None,
            client_secret: None,
        },
    );
    let resp = actix_web::test::call_service(&app, req).await;
    let status = resp.status();
    assert_eq!(status, actix_web::http::StatusCode::OK);
}
