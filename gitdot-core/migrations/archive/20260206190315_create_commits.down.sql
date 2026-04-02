-- Drop indexes
DROP INDEX IF EXISTS idx_commits_created_at;
DROP INDEX IF EXISTS idx_commits_sha;
DROP INDEX IF EXISTS idx_commits_repo_id;
DROP INDEX IF EXISTS idx_commits_author_id;

-- Drop commits table
DROP TABLE IF EXISTS commits;
