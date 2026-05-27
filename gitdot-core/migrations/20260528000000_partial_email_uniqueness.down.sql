DROP INDEX core.idx_user_emails_user_email;
DROP INDEX core.idx_user_emails_email_verified;

CREATE UNIQUE INDEX idx_user_emails_email ON core.user_emails (email);
CREATE INDEX idx_user_emails_verified_lookup
    ON core.user_emails (email) WHERE is_verified;
