CREATE TYPE task_status AS ENUM ('pending', 'running', 'success', 'failure');

CREATE TABLE IF NOT EXISTS tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repo_owner VARCHAR(100) NOT NULL,
    repo_name VARCHAR(100) NOT NULL,
    script TEXT NOT NULL,
    status task_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tasks_repo_owner ON tasks(repo_owner);
CREATE INDEX idx_tasks_status ON tasks(status);
