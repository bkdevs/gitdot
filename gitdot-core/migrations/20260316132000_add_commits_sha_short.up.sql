-- Add generated column for 7-character short SHA and supporting index
ALTER TABLE commits
ADD COLUMN IF NOT EXISTS sha_short VARCHAR(7)
GENERATED ALWAYS AS (substring(sha FROM 1 FOR 7)) STORED;

CREATE INDEX IF NOT EXISTS idx_commits_repo_id_sha_short
ON commits (repo_id, sha_short);

