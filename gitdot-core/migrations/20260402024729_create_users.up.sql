CREATE TABLE core.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    is_email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    provider core.auth_provider NOT NULL DEFAULT 'email',
    settings JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_name ON core.users (name);
