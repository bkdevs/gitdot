ALTER TYPE core.review_status RENAME VALUE 'in_progress' TO 'open';
ALTER TYPE core.diff_status RENAME VALUE 'pending' TO 'draft';
ALTER TABLE core.diffs ALTER COLUMN status SET DEFAULT 'draft'::core.diff_status;
