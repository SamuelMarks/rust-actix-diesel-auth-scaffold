/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::pg::PgConnection;

/// Struct representing a row in table `users`
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::QueryableByName,
    PartialEq,
    diesel::Identifiable,
)]
#[diesel(table_name=users, primary_key(username))]
pub struct Users {
    /// Field representing column `username`
    pub username: String,
    /// Field representing column `password_hash`
    pub password_hash: String,
    /// Field representing column `role`
    pub role: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `users` for [`Users`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=users)]
pub struct CreateUsers {
    /// Field representing column `username`
    pub username: String,
    /// Field representing column `password_hash`
    pub password_hash: String,
    /// Field representing column `role`
    pub role: String,
}

impl Default for CreateUsers {
    fn default() -> Self {
        Self {
            username: String::new(),
            password_hash: String::new(),
            role: String::new(),
        }
    }
}

/// Update Struct for a row in table `users` for [`Users`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default,
)]
#[diesel(table_name=users)]
pub struct UpdateUsers {
    /// Field representing column `password_hash`
    pub password_hash: Option<String>,
    /// Field representing column `role`
    pub role: Option<String>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Result of a `.paginate` function
#[derive(Debug, serde::Serialize)]
pub struct PaginationResult<T> {
    /// Resulting items that are from the current page
    pub items: Vec<T>,
    /// The count of total items there are
    pub total_items: i64,
    /// Current page, 0-based index
    pub page: i64,
    /// Size of a page
    pub page_size: i64,
    /// Number of total possible pages, given the `page_size` and `total_items`
    pub num_pages: i64,
}

impl Users {
    /// Insert a new row into `users` with a given [`CreateUsers`]
    pub fn create(db: &mut ConnectionType, item: &CreateUsers) -> diesel::QueryResult<Self> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `users`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_username: String) -> diesel::QueryResult<Self> {
        use crate::schema::users::dsl::*;

        users.filter(username.eq(param_username)).first::<Self>(db)
    }

    /// Update a row in `users`, identified by the primary key with [`UpdateUsers`]
    pub fn update(
        db: &mut ConnectionType,
        param_username: String,
        item: &UpdateUsers,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::users::dsl::*;

        diesel::update(users.filter(username.eq(param_username)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `users`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_username: String) -> diesel::QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(username.eq(param_username))).execute(db)
    }
}

impl Default for Users {
    fn default() -> Self {
        Self {
            username: String::new(),
            password_hash: String::new(),
            role: String::new(),
            created_at: Default::default(),
        }
    }
}
