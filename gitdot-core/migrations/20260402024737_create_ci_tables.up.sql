CREATE TABLE ci.runners (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    owner_id UUID NOT NULL,
    owner_name VARCHAR(100) NOT NULL,
    owner_type ci.runner_owner_type NOT NULL,
    registered BOOLEAN NOT NULL DEFAULT FALSE,
    last_active TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(owner_id, name)
);

CREATE INDEX idx_runners_owner_id ON ci.runners (owner_id);
CREATE INDEX idx_runners_name ON ci.runners (name);

CREATE TABLE ci.builds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES core.repositories(id) ON DELETE CASCADE,
    number INTEGER NOT NULL,
    ref_name TEXT NOT NULL,
    commit_sha TEXT NOT NULL,
    trigger ci.build_trigger NOT NULL,
    status ci.build_status NOT NULL DEFAULT 'running',
    build_config TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(repository_id, number)
);

CREATE INDEX idx_builds_repository_id ON ci.builds (repository_id);

CREATE TABLE ci.tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    build_id UUID NOT NULL REFERENCES ci.builds(id) ON DELETE CASCADE,
    repository_id UUID NOT NULL REFERENCES core.repositories(id) ON DELETE CASCADE,
    runner_id UUID REFERENCES ci.runners(id),
    name TEXT NOT NULL,
    command TEXT NOT NULL,
    s2_uri TEXT NOT NULL,
    waits_for UUID[] NOT NULL DEFAULT '{}',
    status ci.task_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tasks_build_id ON ci.tasks (build_id);
CREATE INDEX idx_tasks_repository_id ON ci.tasks (repository_id);
CREATE INDEX idx_tasks_runner_id ON ci.tasks (runner_id);
CREATE INDEX idx_tasks_status ON ci.tasks (status);
