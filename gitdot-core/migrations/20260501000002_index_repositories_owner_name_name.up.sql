DROP INDEX IF EXISTS core.idx_repositories_owner_name;
CREATE INDEX idx_repositories_owner_name_name ON core.repositories (owner_name, name);
