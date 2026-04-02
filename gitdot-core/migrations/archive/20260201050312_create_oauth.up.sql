-- Device authorization status enum
CREATE TYPE device_authorization_status AS ENUM ('pending', 'authorized', 'expired');

-- Device authorization requests (short-lived, for OAuth device flow)
CREATE TABLE device_authorizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_code VARCHAR(64) NOT NULL UNIQUE,
    user_code VARCHAR(16) NOT NULL UNIQUE,
    client_id VARCHAR(64) NOT NULL,
    user_id UUID REFERENCES users(id),
    status device_authorization_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_device_authorizations_device_code ON device_authorizations(device_code);
CREATE INDEX idx_device_authorizations_user_code ON device_authorizations(user_code);

-- Long-lived access tokens for CLI/git
CREATE TABLE access_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    client_id VARCHAR(64) NOT NULL,
    token_hash VARCHAR(64) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ
);

CREATE INDEX idx_access_tokens_user_id ON access_tokens(user_id);
CREATE INDEX idx_access_tokens_token_hash ON access_tokens(token_hash);
