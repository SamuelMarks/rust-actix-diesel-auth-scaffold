#![feature(try_trait_v2)]
extern crate alloc;

use crate::tests::routes::token::helpers::test_token_api;
use actix_web::body::MessageBody;
use diesel::Connection;
use diesel_migrations::MigrationHarness;

pub mod errors;
pub mod middleware;

pub mod diesel {
    pub use diesel::*;
}

pub mod models;
pub mod routes;
pub mod schema;

pub mod tests;

pub const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

lazy_static::lazy_static! {
   pub static ref INITIATED: std::sync::Arc<std::sync::Mutex<bool>> = std::sync::Arc::new(std::sync::Mutex::new(false));

   pub static ref POOL: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>> = {
       let db_url = std::env::var("DATABASE_URL").expect("Database url not set");
       let manager = diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(db_url);
       let pool_size = match cfg!(test) {
           true => 1,
           false => 10,
       };
       diesel::r2d2::Builder::new().max_size(pool_size).build(manager).expect("Failed to create db pool")
   };
}

pub async fn get_token(username_s: String, password_s: String) -> String {
    let token = get_token_object(username_s, password_s).await;
    token.access_token
}

pub async fn get_token_object(username_s: String, password_s: String) -> models::token::Token {
    db_init();
    let app = actix_web::test::init_service(
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(POOL.clone()))
            .service(
                actix_web::web::scope("/api")
                    .service(routes::token::token)
                    .service(routes::authorisation::authorise),
            ),
    )
    .await;
    let req = test_token_api::post_username_password(&username_s, &password_s);
    let resp = actix_web::test::call_service(&app, req).await;
    let status = resp.status();
    let resp_body_as_bytes = resp.into_body().try_into_bytes().unwrap();
    let resp_body_as_token: models::token::Token = serde_json::from_slice(&resp_body_as_bytes)
        .unwrap_or_else(|_| panic!("{}", std::str::from_utf8(&resp_body_as_bytes).unwrap()));
    assert_eq!(status, actix_web::http::StatusCode::OK);
    assert!(resp_body_as_token.access_token.len() > 0);
    assert_eq!(resp_body_as_token.token_type, "Bearer");
    assert!(resp_body_as_token.expires_in > 0);
    resp_body_as_token
}

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("./migrations");

pub fn db_init() {
    log::info!("Initializing DB");
    lazy_static::initialize(&POOL);
    let mut connection = establish_connection().expect("Failed to create connection");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}

pub fn establish_connection() -> Result<diesel::PgConnection, errors::AuthError> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::PgConnection::establish(&database_url).map_err(From::from)
}

pub type DbConnectionManager = diesel::r2d2::ConnectionManager<diesel::PgConnection>;
pub type DbPool = diesel::r2d2::Pool<DbConnectionManager>;

pub const fn option_default<T>() -> Option<T> {
    None
}
