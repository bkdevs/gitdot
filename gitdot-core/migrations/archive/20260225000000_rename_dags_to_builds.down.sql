ALTER TABLE tasks RENAME COLUMN build_id TO dag_id;

ALTER INDEX idx_builds_repo_owner RENAME TO idx_dags_repo_owner;
ALTER INDEX idx_builds_repo_name RENAME TO idx_dags_repo_name;

ALTER TABLE builds RENAME TO dags;
