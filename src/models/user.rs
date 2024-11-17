use diesel::prelude::{Insertable, Queryable};
use diesel::Selectable;

pub const DEFAULT_USERNAME: &'static str = "username0";
pub const DEFAULT_PASSWORD_HASH: &'static str = "$argon2id$v=19$m=19456,t=2,p=1$zXneEzRIiMGo/aUvGv17Cg$4wW6+ICmS4W5+xNWO1wYLdYy+oJUZdtfLwjIioZJGhQ";

#[derive(Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: std::time::SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}
