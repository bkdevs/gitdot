ALTER TABLE access_tokens DROP COLUMN token_type;
DROP TYPE IF EXISTS token_type;
