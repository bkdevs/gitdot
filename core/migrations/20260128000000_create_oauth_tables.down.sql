DROP INDEX IF EXISTS idx_device_authorizations_user_code;
DROP INDEX IF EXISTS idx_device_authorizations_device_code;
DROP INDEX IF EXISTS idx_access_tokens_token_hash;
DROP INDEX IF EXISTS idx_access_tokens_user_id;
DROP TABLE IF EXISTS access_tokens;
DROP TABLE IF EXISTS device_authorizations;
