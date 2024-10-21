use diesel::prelude::{Insertable, Queryable};
use diesel::Selectable;

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
