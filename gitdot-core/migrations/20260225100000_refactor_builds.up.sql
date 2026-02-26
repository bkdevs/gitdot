-- Add 'blocked' to task_status enum
ALTER TYPE task_status ADD VALUE IF NOT EXISTS 'blocked';

-- Add trigger and commit_sha to builds
ALTER TABLE builds ADD COLUMN IF NOT EXISTS trigger TEXT NOT NULL DEFAULT '';
ALTER TABLE builds ADD COLUMN IF NOT EXISTS commit_sha TEXT NOT NULL DEFAULT '';
ALTER TABLE builds ALTER COLUMN trigger DROP DEFAULT;
ALTER TABLE builds ALTER COLUMN commit_sha DROP DEFAULT;
