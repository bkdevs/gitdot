DROP INDEX core.idx_user_emails_email;
DROP INDEX core.idx_user_emails_verified_lookup;

CREATE UNIQUE INDEX idx_user_emails_email_verified
    ON core.user_emails (email) WHERE is_verified;

CREATE UNIQUE INDEX idx_user_emails_user_email
    ON core.user_emails (user_id, email);
