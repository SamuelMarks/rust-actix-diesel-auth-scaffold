CREATE TABLE "users"
(
    username      VARCHAR(50) PRIMARY KEY,
    password_hash TEXT NOT NULL,
    role          TEXT        NOT NULL DEFAULT 'regular',
    created_at    TIMESTAMP   NOT NULL DEFAULT current_timestamp
);
