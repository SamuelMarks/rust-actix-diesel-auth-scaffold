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
    user (id) {
        id -> Int4,
        username -> Nullable<Text>,
        password_hash -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    user,
);
