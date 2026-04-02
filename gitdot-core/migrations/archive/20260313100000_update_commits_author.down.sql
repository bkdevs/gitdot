ALTER TABLE commits
    ALTER COLUMN author_id SET NOT NULL,
    DROP COLUMN git_author_name,
    DROP COLUMN git_author_email;
