CREATE TABLE clients
(
    id            INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    client_id     VARCHAR   NOT NULL,
    client_secret VARCHAR   NOT NULL,
    redirect_uri  TEXT      NOT NULL,
    created_at    TIMESTAMP NOT NULL DEFAULT current_timestamp
);
