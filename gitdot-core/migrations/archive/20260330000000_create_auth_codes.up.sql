CREATE TABLE auth_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    code_hash VARCHAR(128) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ
);

CREATE INDEX idx_auth_codes_user_id ON auth_codes(user_id);
CREATE INDEX idx_auth_codes_code_hash ON auth_codes(code_hash);

ALTER TABLE users ADD COLUMN is_email_verified BOOLEAN NOT NULL DEFAULT FALSE;
