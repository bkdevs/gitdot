-- Make diff_id and revision_id required
ALTER TABLE review_comments ALTER COLUMN diff_id SET NOT NULL;
ALTER TABLE review_comments ALTER COLUMN revision_id SET NOT NULL;

-- Rename line_number to line_number_start and add line_number_end for multi-line comments
ALTER TABLE review_comments RENAME COLUMN line_number TO line_number_start;
ALTER TABLE review_comments ADD COLUMN line_number_end INT;
