ALTER TABLE access_tokens RENAME TO tokens;
ALTER INDEX idx_access_tokens_user_id RENAME TO idx_tokens_user_id;
ALTER INDEX idx_access_tokens_token_hash RENAME TO idx_tokens_token_hash;
