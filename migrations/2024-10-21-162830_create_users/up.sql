CREATE TABLE "users"
(
    username      varchar(50) primary key,
    password_hash varchar(50),
    role          text               DEFAULT 'regular',
    created_at    TIMESTAMP NOT NULL DEFAULT current_timestamp
);
