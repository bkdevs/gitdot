CREATE TABLE migration.github_installations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    installation_id BIGINT NOT NULL UNIQUE,
    owner_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    type migration.github_installation_type NOT NULL,
    github_login TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_github_installations_owner_id ON migration.github_installations (owner_id);

CREATE TABLE migration.migrations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    number INTEGER NOT NULL,
    author_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    origin_service migration.migration_origin_service NOT NULL,
    origin VARCHAR(255) NOT NULL,
    origin_type core.repository_owner_type NOT NULL,
    destination VARCHAR(255) NOT NULL,
    destination_type core.repository_owner_type NOT NULL,
    status migration.migration_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(author_id, number)
);

CREATE INDEX idx_migrations_author_id ON migration.migrations (author_id);

CREATE TABLE migration.migration_repositories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    migration_id UUID NOT NULL REFERENCES migration.migrations(id) ON DELETE CASCADE,
    origin_full_name VARCHAR(255) NOT NULL,
    destination_full_name VARCHAR(255) NOT NULL,
    visibility core.repository_visibility NOT NULL DEFAULT 'public',
    status migration.migration_repository_status NOT NULL DEFAULT 'pending',
    error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_migration_repositories_migration_id ON migration.migration_repositories (migration_id);
