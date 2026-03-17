-- Rename existing enum values to snake_case
ALTER TYPE review_status RENAME VALUE 'inprogress' TO 'in_progress';
ALTER TYPE diff_status RENAME VALUE 'changesrequested' TO 'changes_requested';

-- Create verdict enum and table
CREATE TYPE verdict AS ENUM ('approved', 'changes_requested');

CREATE TABLE IF NOT EXISTS review_verdicts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    diff_id UUID NOT NULL REFERENCES diffs(id) ON DELETE CASCADE,
    revision_id UUID NOT NULL REFERENCES revisions(id) ON DELETE CASCADE,
    reviewer_id UUID NOT NULL REFERENCES users(id),
    verdict verdict NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_review_verdicts_diff_id ON review_verdicts(diff_id);
CREATE INDEX idx_review_verdicts_reviewer_id ON review_verdicts(reviewer_id);
