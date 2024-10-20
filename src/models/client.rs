use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct Client {
    pub id: i32,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub created_at: NaiveDateTime,
}


use crate::schema::clients::dsl::clients;

#[derive(Insertable)]
#[table_name="clients"]
pub struct NewClient<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub redirect_uri: &'a str,
}
