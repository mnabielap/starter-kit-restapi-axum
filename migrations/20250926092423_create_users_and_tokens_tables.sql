-- Add migration script here
-- migrations/YYYYMMDDHHMMSS_create_users_and_tokens_tables.sql

-- Membuat tipe enum untuk peran pengguna (user role)
CREATE TYPE user_role AS ENUM ('user', 'admin');

-- Tabel Pengguna (Users)
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

-- Indeks untuk pencarian email yang lebih cepat
CREATE INDEX users_email_idx ON users (email);

-- Tabel Refresh Tokens
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indeks untuk foreign key user_id
CREATE INDEX refresh_tokens_user_id_idx ON refresh_tokens (user_id);