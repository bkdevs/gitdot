-- Remove trigger and commit_sha from builds
ALTER TABLE builds DROP COLUMN IF EXISTS trigger;
ALTER TABLE builds DROP COLUMN IF EXISTS commit_sha;

-- Note: PostgreSQL does not support removing enum values directly.
-- The 'blocked' value added to task_status cannot be removed without recreating the type.
