DROP INDEX IF EXISTS core.idx_users_name;
CREATE INDEX idx_users_name ON core.users (name);
