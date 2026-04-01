CREATE TYPE auth_provider AS ENUM ('email', 'github');
ALTER TABLE users ADD COLUMN provider auth_provider NOT NULL DEFAULT 'email';
