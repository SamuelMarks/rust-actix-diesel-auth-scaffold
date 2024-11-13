use crate::get_secret_app;
use crate::tests::routes::secret::helpers::{
    prepare_secret_test, test_secret_api, PASSWORD, USERNAMES,
};

#[actix_web::test]
async fn test_read_secret() {
    crate::tests::init_db_for_test();
    const USERNAME: &'static str = USERNAMES[0];
    let app = get_secret_app!().await;
    let token = prepare_secret_test(USERNAME, PASSWORD).await;
    let req = test_secret_api::post(&token);
    let resp = actix_web::test::call_service(&app, req).await;
    let status = resp.status();
    assert_eq!(status, actix_web::http::StatusCode::OK);
}
