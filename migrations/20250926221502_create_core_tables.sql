-- migrations/YYYYMMDDHHMMSS_create_core_tables.sql

-- Enum Type for User Roles
CREATE TYPE user_role AS ENUM ('user', 'admin');

-- Users Table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'user',
    is_email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enum Type for Token Type
CREATE TYPE token_type AS ENUM ('refresh', 'resetPassword', 'verifyEmail');

-- Token Table (for refresh, reset password, etc.)
CREATE TABLE tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_type token_type NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    blacklisted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for quick search
CREATE INDEX users_email_idx ON users (email);
CREATE INDEX tokens_user_id_type_idx ON tokens (user_id, token_type);
CREATE UNIQUE INDEX tokens_token_unique_idx ON tokens (token);