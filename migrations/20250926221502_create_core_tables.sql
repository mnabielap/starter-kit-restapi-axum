-- migrations/20250926221502_create_core_tables.sql

-- Users Table
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'user',
    is_email_verified BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Token Table
CREATE TABLE tokens (
    id TEXT PRIMARY KEY,
    token TEXT NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_type TEXT NOT NULL,
    expires_at DATETIME NOT NULL,
    blacklisted BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Index
CREATE INDEX users_email_idx ON users (email);
CREATE INDEX tokens_user_id_type_idx ON tokens (user_id, token_type);
CREATE UNIQUE INDEX tokens_token_unique_idx ON tokens (token);