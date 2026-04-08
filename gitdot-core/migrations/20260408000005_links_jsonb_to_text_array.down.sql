ALTER TABLE core.users ALTER COLUMN links TYPE JSONB USING to_jsonb(links);
ALTER TABLE core.users ALTER COLUMN links SET DEFAULT '[]'::jsonb;
