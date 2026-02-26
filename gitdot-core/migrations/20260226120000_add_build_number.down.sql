ALTER TABLE builds DROP CONSTRAINT IF EXISTS builds_repo_number_unique;
ALTER TABLE builds DROP COLUMN IF EXISTS number;
