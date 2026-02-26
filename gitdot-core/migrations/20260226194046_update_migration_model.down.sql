-- Restore migration_repositories
ALTER TABLE migration_repositories ADD COLUMN repository_id UUID REFERENCES repositories(id) ON DELETE SET NULL;
ALTER TABLE migration_repositories DROP COLUMN destination_full_name;
ALTER TABLE migration_repositories RENAME COLUMN origin_full_name TO full_name;

-- Restore migrations
CREATE TYPE migration_origin AS ENUM ('github');

ALTER TABLE migrations RENAME COLUMN origin TO origin_name;
ALTER TABLE migrations ADD COLUMN origin migration_origin;
UPDATE migrations SET origin = 'github';
ALTER TABLE migrations ALTER COLUMN origin SET NOT NULL;

ALTER TABLE migrations
    DROP COLUMN origin_service,
    DROP COLUMN origin_name,
    DROP COLUMN origin_type,
    DROP COLUMN destination,
    DROP COLUMN destination_type;

DROP TYPE migration_origin_service;
