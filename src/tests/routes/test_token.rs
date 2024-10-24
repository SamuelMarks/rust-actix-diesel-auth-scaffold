use crate::models::token::Token;
use crate::routes::token::TokenRequest;
use actix_web::body::MessageBody;

lazy_static::lazy_static! {
   static ref INITIATED: std::sync::Arc<std::sync::Mutex<bool>> = std::sync::Arc::new(std::sync::Mutex::new(false));

    static ref POOL: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>> = {
       let db_url = std::env::var("DATABASE_URL").expect("Database url not set");
       let manager = diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(db_url);
       let pool_size = match cfg!(test) {
           true => 1,
           false => 10,
       };
       diesel::r2d2::Builder::new().max_size(pool_size).build(manager).expect("Failed to create db pool")
   };
}

pub fn db_init() {
    log::info!("Initializing DB");
    lazy_static::initialize(&POOL);
    /*let conn = connection().expect("Failed to get db connection");
    if cfg!(test) {
        conn.begin_test_transaction().expect("Failed to start transaction");
    }
    embedded_migrations::run(&conn).unwrap();*/
}

#[cfg(test)]
pub fn init() {
    let mut initiated = INITIATED.lock().unwrap();
    if *initiated == false {
        dotenvy::from_filename(std::path::Path::new("..").join("..").join(".env")).ok();
        *initiated = true;
    }
}

#[actix_web::test]
async fn test_token_post() {
    init();
    // db_init();
    let app = actix_web::test::init_service(
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(POOL.clone()))
            .service(crate::routes::token::token),
    )
    .await;
    let req = actix_web::test::TestRequest::post()
        .uri("/token")
        .set_json(TokenRequest {
            grant_type: String::from("password"),
            username: Some(String::from("username")),
            password: Some(String::from("password")),
            client_id: None,
            client_secret: None,
        })
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;
    let status = resp.status();
    let resp_body_as_bytes = resp.into_body().try_into_bytes().unwrap();
    let resp_body_as_str = std::str::from_utf8(&resp_body_as_bytes).unwrap();
    let resp_body_as_token: Token = serde_json::from_slice(&resp_body_as_bytes).unwrap();
    println!("resp_body_as_str = {:#?}", resp_body_as_str);
    assert_eq!(status, http::StatusCode::OK);
    assert!(resp_body_as_token.access_token.len() > 0);
    assert_eq!(resp_body_as_token.token_type, "Bearer");
    assert!(resp_body_as_token.expires_in > 0)
    // assert!(status.is_success());
    //assert!(resp.status().is_client_error());
}
