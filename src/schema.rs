// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Int4,
        client_id -> Varchar,
        client_secret -> Varchar,
        redirect_uri -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (username) {
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 50]
        password_hash -> Varchar,
        role -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    users,
);
