TRUNCATE TABLE auth.auth_codes;

ALTER TABLE auth.auth_codes RENAME COLUMN code_hash TO user_code;
ALTER TABLE auth.auth_codes ALTER COLUMN user_code TYPE VARCHAR(16);

DROP INDEX IF EXISTS auth.idx_auth_codes_code_hash;
CREATE INDEX idx_auth_codes_user_code ON auth.auth_codes (user_code);
