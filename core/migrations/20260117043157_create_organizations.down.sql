-- Drop indexes
DROP INDEX IF EXISTS idx_organization_members_org_id;
DROP INDEX IF EXISTS idx_organization_members_user_id;
DROP INDEX IF EXISTS idx_organizations_name;

-- Drop tables (order matters due to foreign key)
DROP TABLE IF EXISTS organization_members;
DROP TABLE IF EXISTS organizations;

-- Drop enum
DROP TYPE IF EXISTS organization_role;
