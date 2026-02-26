-- Add waits_for UUID array to tasks
ALTER TABLE tasks ADD COLUMN waits_for UUID[] NOT NULL DEFAULT '{}';

-- Add 'blocked' back to task_status enum
ALTER TYPE task_status RENAME TO task_status_old;
CREATE TYPE task_status AS ENUM ('blocked', 'pending', 'assigned', 'running', 'success', 'failure');
ALTER TABLE tasks ALTER COLUMN status DROP DEFAULT;
ALTER TABLE tasks ALTER COLUMN status TYPE task_status USING status::text::task_status;
ALTER TABLE tasks ALTER COLUMN status SET DEFAULT 'pending';
DROP TYPE task_status_old;

-- Drop build_config from builds
ALTER TABLE builds DROP COLUMN build_config;
