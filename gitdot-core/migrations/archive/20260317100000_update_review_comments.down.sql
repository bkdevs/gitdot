ALTER TABLE review_comments DROP COLUMN IF EXISTS line_number_end;
ALTER TABLE review_comments RENAME COLUMN line_number_start TO line_number;
ALTER TABLE review_comments ALTER COLUMN revision_id DROP NOT NULL;
ALTER TABLE review_comments ALTER COLUMN diff_id DROP NOT NULL;
