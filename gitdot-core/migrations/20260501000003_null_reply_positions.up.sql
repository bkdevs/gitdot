UPDATE core.review_comments
SET
    file_path = NULL,
    line_number_start = NULL,
    line_number_end = NULL,
    start_character = NULL,
    end_character = NULL,
    side = NULL
WHERE parent_id IS NOT NULL;
