#![feature(try_trait_v2)]

use diesel::Connection;

use crate::errors::AuthError;

pub mod errors;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;

#[cfg(test)]
mod tests;

pub fn establish_connection() -> Result<diesel::PgConnection, AuthError> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::PgConnection::establish(&database_url).map_err(From::from)
}
