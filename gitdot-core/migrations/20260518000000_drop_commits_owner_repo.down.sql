ALTER TABLE core.commits ADD COLUMN owner_name TEXT NOT NULL DEFAULT '';
ALTER TABLE core.commits ADD COLUMN repo_name TEXT NOT NULL DEFAULT '';

UPDATE core.commits c
SET owner_name = COALESCE(u.name, o.name),
    repo_name  = r.name
FROM core.repositories r
LEFT JOIN core.users         u ON r.owner_id = u.id AND r.owner_type = 'user'
LEFT JOIN core.organizations o ON r.owner_id = o.id AND r.owner_type = 'organization'
WHERE c.repo_id = r.id;
