ALTER TABLE tasks ADD COLUMN repo_owner TEXT;
ALTER TABLE tasks ADD COLUMN repo_name TEXT;
UPDATE tasks t SET repo_owner = r.owner_name, repo_name = r.name
FROM repositories r WHERE r.id = t.repository_id;
ALTER TABLE tasks ALTER COLUMN repo_owner SET NOT NULL;
ALTER TABLE tasks ALTER COLUMN repo_name SET NOT NULL;
ALTER TABLE tasks DROP COLUMN repository_id;

ALTER TABLE builds DROP CONSTRAINT builds_repo_number_unique;
ALTER TABLE builds ADD COLUMN repo_owner TEXT;
ALTER TABLE builds ADD COLUMN repo_name TEXT;
UPDATE builds b SET repo_owner = r.owner_name, repo_name = r.name
FROM repositories r WHERE r.id = b.repository_id;
ALTER TABLE builds ALTER COLUMN repo_owner SET NOT NULL;
ALTER TABLE builds ALTER COLUMN repo_name SET NOT NULL;
ALTER TABLE builds ADD CONSTRAINT builds_repo_number_unique UNIQUE (repo_owner, repo_name, number);
ALTER TABLE builds DROP COLUMN repository_id;
