#[actix_web::test]
async fn test_read_token() {
    crate::tests::init_db_for_test();
    const USERNAME: &'static str = crate::tests::routes::token::helpers::USERNAMES[0];
    let app = crate::get_token_app!().await;
    let req = crate::tests::routes::token::helpers::test_token_api::post(
        crate::routes::token::types::TokenRequest {
            grant_type: crate::routes::token::types::GrantType::Password,
            username: Some(String::from(USERNAME)),
            password: Some(String::from(
                crate::tests::routes::secret::helpers::PASSWORD,
            )),
            ..crate::routes::token::types::TokenRequest::default()
        },
    );
    let resp = actix_web::test::call_service(&app, req).await;
    let status = resp.status();
    assert_eq!(status, actix_web::http::StatusCode::OK);
}

#[actix_web::test]
async fn test_refresh_token_flow() {
    use actix_web::body::MessageBody;

    crate::tests::init_db_for_test();
    const USERNAME: &'static str = crate::tests::routes::token::helpers::USERNAMES[1];
    let token0 = crate::get_token_object(
        String::from(USERNAME),
        String::from(crate::tests::routes::secret::helpers::PASSWORD),
    )
    .await;

    let app = crate::get_token_app!().await;
    let req = crate::tests::routes::token::helpers::test_token_api::post(
        crate::routes::token::types::TokenRequest {
            grant_type: crate::routes::token::types::GrantType::RefreshToken,
            refresh_token: Some(token0.refresh_token.clone()),
            ..crate::routes::token::types::TokenRequest::default()
        },
    );
    let resp = actix_web::test::call_service(&app, req).await;
    let status = resp.status();
    assert_eq!(status, actix_web::http::StatusCode::OK);
    let token: crate::models::token::Token =
        serde_json::from_slice(&resp.into_body().try_into_bytes().unwrap()).unwrap();
    assert_ne!(token, token0);
}
