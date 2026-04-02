ALTER TABLE tokens RENAME TO access_tokens;
ALTER INDEX idx_tokens_user_id RENAME TO idx_access_tokens_user_id;
ALTER INDEX idx_tokens_token_hash RENAME TO idx_access_tokens_token_hash;
