ALTER TABLE core.users ADD COLUMN email VARCHAR(255);
ALTER TABLE core.users ADD COLUMN is_email_verified BOOLEAN NOT NULL DEFAULT FALSE;

UPDATE core.users u
SET email = ue.email,
    is_email_verified = ue.is_verified
FROM core.user_emails ue
WHERE ue.user_id = u.id AND ue.is_primary;

ALTER TABLE core.users ALTER COLUMN email SET NOT NULL;
ALTER TABLE core.users ADD CONSTRAINT users_email_key UNIQUE (email);
