CREATE TYPE migration_status AS ENUM ('pending', 'running', 'completed', 'failed');
CREATE TYPE migration_origin AS ENUM ('github');

CREATE TABLE migrations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    origin migration_origin NOT NULL,
    status migration_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_migrations_owner_id ON migrations(owner_id);

CREATE TYPE migration_repository_status AS ENUM ('pending', 'running', 'completed', 'failed');

CREATE TABLE migration_repositories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    migration_id UUID NOT NULL REFERENCES migrations(id) ON DELETE CASCADE,
    repository_id UUID REFERENCES repositories(id) ON DELETE SET NULL,
    full_name VARCHAR(255) NOT NULL,
    status migration_repository_status NOT NULL DEFAULT 'pending',
    error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_migration_repositories_migration_id ON migration_repositories(migration_id);
