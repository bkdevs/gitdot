-- Device-flow user codes were stored as plaintext 6-char codes. Hash them at
-- rest so a DB read can't reveal live device codes. Existing plaintext codes
-- can't be re-hashed, so drop them (codes are short-lived and re-requestable).
TRUNCATE TABLE auth.device_authorizations;

ALTER TABLE auth.device_authorizations RENAME COLUMN user_code TO user_code_hash;
ALTER TABLE auth.device_authorizations ALTER COLUMN user_code_hash TYPE VARCHAR(128);

DROP INDEX IF EXISTS auth.idx_device_authorizations_user_code;
CREATE INDEX idx_device_authorizations_user_code_hash ON auth.device_authorizations (user_code_hash);
