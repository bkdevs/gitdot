-- Re-add build_config to builds
ALTER TABLE builds ADD COLUMN build_config TEXT NOT NULL DEFAULT '';
ALTER TABLE builds ALTER COLUMN build_config DROP DEFAULT;

-- Migrate blocked tasks to pending before dropping the variant
UPDATE tasks SET status = 'pending' WHERE status = 'blocked';

-- Drop 'blocked' from task_status enum
ALTER TYPE task_status RENAME TO task_status_old;
CREATE TYPE task_status AS ENUM ('pending', 'assigned', 'running', 'success', 'failure');
ALTER TABLE tasks ALTER COLUMN status DROP DEFAULT;
ALTER TABLE tasks ALTER COLUMN status TYPE task_status USING status::text::task_status;
ALTER TABLE tasks ALTER COLUMN status SET DEFAULT 'pending';
DROP TYPE task_status_old;

-- Drop waits_for from tasks
ALTER TABLE tasks DROP COLUMN waits_for;
