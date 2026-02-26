ALTER TABLE tokens RENAME COLUMN user_id TO principal_id;
ALTER INDEX idx_tokens_user_id RENAME TO idx_tokens_principal_id;
