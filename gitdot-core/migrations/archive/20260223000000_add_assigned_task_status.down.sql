-- PostgreSQL does not support removing enum values directly.
-- To roll back, recreate the enum without 'assigned' and update the column.
ALTER TABLE tasks ALTER COLUMN status TYPE VARCHAR(50);
DROP TYPE task_status;
CREATE TYPE task_status AS ENUM ('pending', 'running', 'success', 'failure');
ALTER TABLE tasks ALTER COLUMN status TYPE task_status USING status::task_status;
