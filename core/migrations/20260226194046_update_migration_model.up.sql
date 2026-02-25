-- Replace migration_origin enum with migration_origin_service
CREATE TYPE migration_origin_service AS ENUM ('github');

ALTER TABLE migrations
    ADD COLUMN origin_service migration_origin_service,
    ADD COLUMN origin_name VARCHAR(255),
    ADD COLUMN origin_type repository_owner_type,
    ADD COLUMN destination VARCHAR(255),
    ADD COLUMN destination_type repository_owner_type;

-- Backfill from the old origin column
UPDATE migrations SET
    origin_service = 'github',
    origin_name = '',
    origin_type = 'user',
    destination = '',
    destination_type = 'user';

ALTER TABLE migrations
    ALTER COLUMN origin_service SET NOT NULL,
    ALTER COLUMN origin_name SET NOT NULL,
    ALTER COLUMN origin_type SET NOT NULL,
    ALTER COLUMN destination SET NOT NULL,
    ALTER COLUMN destination_type SET NOT NULL;

ALTER TABLE migrations DROP COLUMN origin;
ALTER TABLE migrations RENAME COLUMN origin_name TO origin;

DROP TYPE migration_origin;

-- Update migration_repositories: rename full_name, add destination_full_name, drop repository_id
ALTER TABLE migration_repositories RENAME COLUMN full_name TO origin_full_name;
ALTER TABLE migration_repositories ADD COLUMN destination_full_name VARCHAR(255);
UPDATE migration_repositories SET destination_full_name = origin_full_name;
ALTER TABLE migration_repositories ALTER COLUMN destination_full_name SET NOT NULL;
ALTER TABLE migration_repositories DROP COLUMN repository_id;
