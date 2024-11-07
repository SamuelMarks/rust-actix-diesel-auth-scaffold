#![feature(try_trait_v2)]

use diesel::Connection;
use diesel_migrations::MigrationHarness;

pub mod errors;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;

#[cfg(test)]
mod tests;

pub const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

lazy_static::lazy_static! {
   static ref INITIATED: std::sync::Arc<std::sync::Mutex<bool>> = std::sync::Arc::new(std::sync::Mutex::new(false));

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

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
