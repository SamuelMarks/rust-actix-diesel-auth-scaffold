use diesel::prelude::{Queryable, Insertable};
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

use crate::schema::user::dsl::user;

#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}
