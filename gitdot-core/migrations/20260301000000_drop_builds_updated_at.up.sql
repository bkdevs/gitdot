-- Drop updated_at column from builds; updated_at will be derived from tasks
ALTER TABLE builds DROP COLUMN IF EXISTS updated_at;

