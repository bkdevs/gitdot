-- Backfill existing migrations with sequential numbers per author
ALTER TABLE migrations ADD COLUMN number INTEGER;

UPDATE migrations m
SET number = sub.row_num
FROM (
    SELECT id, ROW_NUMBER() OVER (PARTITION BY author_id ORDER BY created_at ASC) AS row_num
    FROM migrations
) sub
WHERE m.id = sub.id;

ALTER TABLE migrations ALTER COLUMN number SET NOT NULL;

CREATE UNIQUE INDEX idx_migrations_author_id_number ON migrations(author_id, number);
