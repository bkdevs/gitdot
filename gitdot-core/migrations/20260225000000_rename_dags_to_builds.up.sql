ALTER TABLE dags RENAME TO builds;

ALTER INDEX idx_dags_repo_owner RENAME TO idx_builds_repo_owner;
ALTER INDEX idx_dags_repo_name RENAME TO idx_builds_repo_name;

ALTER TABLE tasks RENAME COLUMN dag_id TO build_id;
