ALTER TABLE tokens RENAME COLUMN principal_id TO user_id;
ALTER INDEX idx_tokens_principal_id RENAME TO idx_tokens_user_id;
