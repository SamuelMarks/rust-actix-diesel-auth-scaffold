CREATE TABLE "user" (
    id integer primary key generated always as identity,
    username text,
    password_hash text,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
