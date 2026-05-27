CREATE TABLE auth.email_verification_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_email_id UUID NOT NULL REFERENCES core.user_emails(id) ON DELETE CASCADE,
    code_hash VARCHAR(128) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ
);

CREATE INDEX idx_email_verification_codes_user_email_id
    ON auth.email_verification_codes (user_email_id);
