DROP INDEX IF EXISTS idx_commits_repo_id_sha_short;

ALTER TABLE commits
DROP COLUMN IF EXISTS sha_short;

