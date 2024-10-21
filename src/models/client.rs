use diesel::prelude::*;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::clients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Client {
    pub id: i32,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub created_at: std::time::SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::clients)]
pub struct NewClient<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub redirect_uri: &'a str,
}
