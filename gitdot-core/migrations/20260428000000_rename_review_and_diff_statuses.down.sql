ALTER TYPE core.review_status RENAME VALUE 'open' TO 'in_progress';
ALTER TYPE core.diff_status RENAME VALUE 'draft' TO 'pending';
ALTER TABLE core.diffs ALTER COLUMN status SET DEFAULT 'pending'::core.diff_status;
