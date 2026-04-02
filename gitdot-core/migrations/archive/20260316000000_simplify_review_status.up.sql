-- Remove status column from reviewers
ALTER TABLE reviewers DROP COLUMN status;

-- Replace review_status enum: draft, inprogress, closed
-- (rename_all = "lowercase" maps InProgress → inprogress)
ALTER TYPE review_status RENAME TO review_status_old;
CREATE TYPE review_status AS ENUM ('draft', 'inprogress', 'closed');
ALTER TABLE reviews
    ALTER COLUMN status DROP DEFAULT,
    ALTER COLUMN status TYPE review_status USING
        CASE status::text
            WHEN 'draft' THEN 'draft'::review_status
            WHEN 'open' THEN 'inprogress'::review_status
            WHEN 'changesrequested' THEN 'inprogress'::review_status
            WHEN 'approved' THEN 'inprogress'::review_status
            WHEN 'merged' THEN 'closed'::review_status
        END,
    ALTER COLUMN status SET DEFAULT 'draft'::review_status;
DROP TYPE review_status_old;

-- Drop reviewer_status type
DROP TYPE reviewer_status;
