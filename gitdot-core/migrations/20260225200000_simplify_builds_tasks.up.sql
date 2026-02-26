-- builds: swap task_dependencies for build_config
ALTER TABLE builds ADD COLUMN build_config TEXT NOT NULL DEFAULT '';
ALTER TABLE builds ALTER COLUMN build_config DROP DEFAULT;
ALTER TABLE builds DROP COLUMN task_dependencies;

-- tasks: add name column
ALTER TABLE tasks ADD COLUMN name TEXT NOT NULL DEFAULT '';
ALTER TABLE tasks ALTER COLUMN name DROP DEFAULT;

-- migrate blocked tasks to pending before dropping the variant
UPDATE tasks SET status = 'pending' WHERE status = 'blocked';

-- drop 'blocked' from task_status enum
ALTER TYPE task_status RENAME TO task_status_old;
CREATE TYPE task_status AS ENUM ('pending', 'assigned', 'running', 'success', 'failure');
ALTER TABLE tasks ALTER COLUMN status DROP DEFAULT;
ALTER TABLE tasks ALTER COLUMN status TYPE task_status USING status::text::task_status;
ALTER TABLE tasks ALTER COLUMN status SET DEFAULT 'pending';
DROP TYPE task_status_old;
