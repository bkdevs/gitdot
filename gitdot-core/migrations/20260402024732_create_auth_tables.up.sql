CREATE TABLE auth.auth_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    code_hash VARCHAR(128) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ
);

CREATE INDEX idx_auth_codes_user_id ON auth.auth_codes (user_id);
CREATE INDEX idx_auth_codes_code_hash ON auth.auth_codes (code_hash);

CREATE TABLE auth.sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    refresh_token_hash VARCHAR(128) NOT NULL UNIQUE,
    refresh_token_family UUID NOT NULL,
    user_agent TEXT,
    ip_address INET,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ
);

CREATE INDEX idx_sessions_user_id ON auth.sessions (user_id);
CREATE INDEX idx_sessions_refresh_token_hash ON auth.sessions (refresh_token_hash);
CREATE INDEX idx_sessions_family ON auth.sessions (refresh_token_family);

CREATE TABLE auth.device_authorizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_code_hash VARCHAR(128) NOT NULL UNIQUE,
    user_code VARCHAR(16) NOT NULL UNIQUE,
    client_id VARCHAR(64) NOT NULL,
    user_id UUID REFERENCES core.users(id) ON DELETE CASCADE,
    status auth.device_authorization_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_device_authorizations_device_code ON auth.device_authorizations (device_code_hash);
CREATE INDEX idx_device_authorizations_user_code ON auth.device_authorizations (user_code);

CREATE TABLE auth.tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    principal_id UUID NOT NULL,
    client_id VARCHAR(64) NOT NULL,
    token_hash VARCHAR(64) NOT NULL UNIQUE,
    token_type auth.token_type NOT NULL DEFAULT 'personal',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ
);

CREATE INDEX idx_tokens_principal_id ON auth.tokens (principal_id);
CREATE INDEX idx_tokens_token_hash ON auth.tokens (token_hash);
