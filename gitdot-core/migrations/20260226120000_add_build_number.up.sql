-- Add number column (repo-scoped, starts at 1)
ALTER TABLE builds ADD COLUMN number INTEGER;

-- Backfill per repo by created_at order
UPDATE builds b
SET number = sub.rn
FROM (
    SELECT id, ROW_NUMBER() OVER (PARTITION BY repo_owner, repo_name ORDER BY created_at) AS rn
    FROM builds
) sub
WHERE b.id = sub.id;

ALTER TABLE builds ALTER COLUMN number SET NOT NULL;
ALTER TABLE builds ADD CONSTRAINT builds_repo_number_unique UNIQUE (repo_owner, repo_name, number);
