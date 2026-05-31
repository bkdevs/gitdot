TRUNCATE TABLE auth.device_authorizations;

DROP INDEX IF EXISTS auth.idx_device_authorizations_user_code_hash;
CREATE INDEX idx_device_authorizations_user_code ON auth.device_authorizations (user_code_hash);

ALTER TABLE auth.device_authorizations ALTER COLUMN user_code_hash TYPE VARCHAR(16);
ALTER TABLE auth.device_authorizations RENAME COLUMN user_code_hash TO user_code;
