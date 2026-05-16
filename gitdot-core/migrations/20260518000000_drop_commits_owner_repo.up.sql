-- Drop denormalized owner_name / repo_name from core.commits.
-- The values are now sourced via JOINs to repositories → users/organizations
-- with COALESCE(u.name, o.name), embedded in query results as a JSON
-- object on the Commit row.
ALTER TABLE core.commits DROP COLUMN owner_name;
ALTER TABLE core.commits DROP COLUMN repo_name;
