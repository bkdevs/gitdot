CREATE TYPE runner_owner_type AS ENUM ('user', 'organization');

CREATE TABLE IF NOT EXISTS runners (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    owner_id UUID NOT NULL,
    owner_type runner_owner_type NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(owner_id, name)
);

CREATE INDEX idx_runners_owner_id ON runners(owner_id);
CREATE INDEX idx_runners_name ON runners(name);
