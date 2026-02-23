ALTER TABLE dags DROP COLUMN task_dependencies;
ALTER TABLE dags ADD COLUMN task_ids UUID[] NOT NULL DEFAULT '{}';
