ALTER TABLE core.users ADD COLUMN links JSONB NOT NULL DEFAULT '[]'::jsonb;
ALTER TABLE core.users DROP COLUMN website;
