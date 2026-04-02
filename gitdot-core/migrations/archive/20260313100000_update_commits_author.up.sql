ALTER TABLE commits
    ALTER COLUMN author_id DROP NOT NULL,
    ADD COLUMN git_author_name TEXT NOT NULL DEFAULT '',
    ADD COLUMN git_author_email TEXT NOT NULL DEFAULT '';
