DROP INDEX IF EXISTS auth.idx_auth_codes_user_code;
CREATE INDEX idx_auth_codes_code_hash ON auth.auth_codes (user_code);

ALTER TABLE auth.auth_codes ALTER COLUMN user_code TYPE VARCHAR(128);
ALTER TABLE auth.auth_codes RENAME COLUMN user_code TO code_hash;
