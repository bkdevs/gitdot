CREATE TYPE migration.github_installation_type AS ENUM ('user', 'organization');
CREATE TYPE migration.migration_origin_service AS ENUM ('github');
CREATE TYPE migration.migration_status AS ENUM ('pending', 'running', 'completed', 'failed');
CREATE TYPE migration.migration_repository_status AS ENUM ('pending', 'running', 'completed', 'failed');
