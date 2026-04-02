-- Restore updated_at column on builds; backfill from created_at
ALTER TABLE builds ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ;

UPDATE builds
SET updated_at = COALESCE(updated_at, created_at);

ALTER TABLE builds ALTER COLUMN updated_at SET NOT NULL;

