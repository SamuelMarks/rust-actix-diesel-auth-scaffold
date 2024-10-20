CREATE TABLE clients (
    id SERIAL PRIMARY KEY,
    client_id VARCHAR NOT NULL,
    client_secret VARCHAR NOT NULL,
    redirect_uri TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
