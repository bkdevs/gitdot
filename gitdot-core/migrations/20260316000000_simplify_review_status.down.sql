-- Restore reviewer_status type
CREATE TYPE reviewer_status AS ENUM ('pending', 'changesrequested', 'approved');

-- Restore review_status enum
ALTER TYPE review_status RENAME TO review_status_new;
CREATE TYPE review_status AS ENUM ('draft', 'open', 'changesrequested', 'approved', 'merged');
ALTER TABLE reviews
    ALTER COLUMN status DROP DEFAULT,
    ALTER COLUMN status TYPE review_status USING
        CASE status::text
            WHEN 'draft' THEN 'draft'::review_status
            WHEN 'inprogress' THEN 'open'::review_status
            WHEN 'closed' THEN 'merged'::review_status
        END,
    ALTER COLUMN status SET DEFAULT 'draft'::review_status;
DROP TYPE review_status_new;

-- Restore status column on reviewers
ALTER TABLE reviewers ADD COLUMN status reviewer_status NOT NULL DEFAULT 'pending';
