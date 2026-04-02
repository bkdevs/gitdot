CREATE TABLE core.repositories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    owner_id UUID NOT NULL,
    owner_name VARCHAR(100) NOT NULL,
    owner_type core.repository_owner_type NOT NULL,
    visibility core.repository_visibility NOT NULL,
    settings JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(owner_id, name)
);

CREATE INDEX idx_repositories_name ON core.repositories (name);
CREATE INDEX idx_repositories_owner_id ON core.repositories (owner_id);
CREATE INDEX idx_repositories_visibility ON core.repositories (visibility);
