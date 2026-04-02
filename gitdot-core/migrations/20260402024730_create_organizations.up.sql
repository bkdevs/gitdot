CREATE TABLE core.organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_organizations_name ON core.organizations (name);

CREATE TABLE core.organization_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES core.organizations(id) ON DELETE CASCADE,
    role core.organization_role NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, organization_id)
);

CREATE INDEX idx_organization_members_user_id ON core.organization_members (user_id);
CREATE INDEX idx_organization_members_org_id ON core.organization_members (organization_id);
