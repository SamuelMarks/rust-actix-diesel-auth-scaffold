/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::pg::PgConnection;

/// Struct representing a row in table `clients`
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
#[diesel(table_name=clients, primary_key(id))]
pub struct Clients {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `client_id`
    pub client_id: String,
    /// Field representing column `client_secret`
    pub client_secret: String,
    /// Field representing column `redirect_uri`
    pub redirect_uri: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `clients` for [`Clients`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=clients)]
pub struct CreateClients {
    /// Field representing column `client_id`
    pub client_id: String,
    /// Field representing column `client_secret`
    pub client_secret: String,
    /// Field representing column `redirect_uri`
    pub redirect_uri: String,
}

impl Default for CreateClients {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            redirect_uri: String::new(),
        }
    }
}

/// Update Struct for a row in table `clients` for [`Clients`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default,
)]
#[diesel(table_name=clients)]
pub struct UpdateClients {
    /// Field representing column `client_id`
    pub client_id: Option<String>,
    /// Field representing column `client_secret`
    pub client_secret: Option<String>,
    /// Field representing column `redirect_uri`
    pub redirect_uri: Option<String>,
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

impl Clients {
    /// Insert a new row into `clients` with a given [`CreateClients`]
    pub fn create(db: &mut ConnectionType, item: &CreateClients) -> diesel::QueryResult<Self> {
        use crate::schema::clients::dsl::*;

        diesel::insert_into(clients)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `clients`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::clients::dsl::*;

        clients.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `clients`, identified by the primary key with [`UpdateClients`]
    pub fn update(
        db: &mut ConnectionType,
        param_id: i32,
        item: &UpdateClients,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::clients::dsl::*;

        diesel::update(clients.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `clients`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::clients::dsl::*;

        diesel::delete(clients.filter(id.eq(param_id))).execute(db)
    }
}

impl Default for Clients {
    fn default() -> Self {
        Self {
            id: 0,
            client_id: String::new(),
            client_secret: String::new(),
            redirect_uri: String::new(),
            created_at: Default::default(),
        }
    }
}
