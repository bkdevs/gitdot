CREATE TYPE token_type AS ENUM ('personal', 'runner');
ALTER TABLE access_tokens ADD COLUMN token_type token_type NOT NULL DEFAULT 'personal';
