CREATE TABLE IF NOT EXISTS dags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repo_owner VARCHAR(100) NOT NULL,
    repo_name VARCHAR(100) NOT NULL,
    task_ids UUID[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_dags_repo_owner ON dags(repo_owner);
CREATE INDEX idx_dags_repo_name ON dags(repo_name);
