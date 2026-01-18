-- Create repository owner type enum
CREATE TYPE repository_owner_type AS ENUM ('User', 'Organization');

-- Create repository visibility enum
CREATE TYPE repository_visibility AS ENUM ('Public', 'Private');

-- Create repositories table
CREATE TABLE IF NOT EXISTS repositories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    owner_id UUID NOT NULL,
    owner_name VARCHAR(100) NOT NULL,
    owner_type repository_owner_type NOT NULL,
    visibility repository_visibility NOT NULL DEFAULT 'Private',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(owner_id, name)
);

-- Create indexes for better query performance
CREATE INDEX idx_repositories_name ON repositories(name);
CREATE INDEX idx_repositories_owner_id ON repositories(owner_id);
CREATE INDEX idx_repositories_visibility ON repositories(visibility);
