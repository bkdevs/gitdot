-- Soft-delete marker for accounts. When set, login and session refresh are
-- blocked for the user; the row and its data are left intact for a later full
-- teardown (repositories on disk, sole-owner orgs, related rows).
ALTER TABLE core.users ADD COLUMN deleted_at TIMESTAMPTZ;
