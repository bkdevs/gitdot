-- core.users.name is the user's handle and shares one slug namespace with
-- core.organizations.name (already UNIQUE). Uniqueness was previously enforced
-- only in the service layer (is_name_taken), which races on concurrent renames.
-- Promote the plain idx_users_name to a unique index so the database is the
-- source of truth. Names are stored lowercased (OwnerName sanitization), so a
-- plain UNIQUE is sufficient. Fails loudly if duplicate names already exist.
DROP INDEX IF EXISTS core.idx_users_name;
CREATE UNIQUE INDEX idx_users_name ON core.users (name);
