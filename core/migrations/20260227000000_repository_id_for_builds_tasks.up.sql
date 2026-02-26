-- builds: add repository_id FK, backfill from repositories table, drop old columns
ALTER TABLE builds ADD COLUMN repository_id UUID REFERENCES repositories(id) ON DELETE CASCADE;

UPDATE builds b
SET repository_id = r.id
FROM repositories r
WHERE r.owner_name = b.repo_owner AND r.name = b.repo_name;

ALTER TABLE builds ALTER COLUMN repository_id SET NOT NULL;
ALTER TABLE builds DROP CONSTRAINT builds_repo_number_unique;
ALTER TABLE builds ADD CONSTRAINT builds_repo_number_unique UNIQUE (repository_id, number);
ALTER TABLE builds DROP COLUMN repo_owner;
ALTER TABLE builds DROP COLUMN repo_name;

-- tasks: derive repository_id from parent build
ALTER TABLE tasks ADD COLUMN repository_id UUID REFERENCES repositories(id) ON DELETE CASCADE;

UPDATE tasks t
SET repository_id = b.repository_id
FROM builds b
WHERE t.build_id = b.id;

ALTER TABLE tasks ALTER COLUMN repository_id SET NOT NULL;
ALTER TABLE tasks DROP COLUMN repo_owner;
ALTER TABLE tasks DROP COLUMN repo_name;
