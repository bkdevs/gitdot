CREATE TABLE core.reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES core.repositories(id) ON DELETE CASCADE,
    number INTEGER NOT NULL,
    author_id UUID NOT NULL REFERENCES core.users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    target_branch VARCHAR(255) NOT NULL,
    status core.review_status NOT NULL DEFAULT 'draft',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(repository_id, number)
);

CREATE INDEX idx_reviews_repository_id ON core.reviews (repository_id);
CREATE INDEX idx_reviews_author_id ON core.reviews (author_id);
CREATE INDEX idx_reviews_status ON core.reviews (status);

CREATE TABLE core.diffs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    review_id UUID NOT NULL REFERENCES core.reviews(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    status core.diff_status NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(review_id, position)
);

CREATE INDEX idx_diffs_review_id ON core.diffs (review_id);

CREATE TABLE core.revisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    diff_id UUID NOT NULL REFERENCES core.diffs(id) ON DELETE CASCADE,
    number INTEGER NOT NULL,
    commit_hash VARCHAR(40) NOT NULL,
    parent_hash VARCHAR(40) NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(diff_id, number)
);

CREATE INDEX idx_revisions_diff_id ON core.revisions (diff_id);

CREATE TABLE core.reviewers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    review_id UUID NOT NULL REFERENCES core.reviews(id) ON DELETE CASCADE,
    reviewer_id UUID NOT NULL REFERENCES core.users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(review_id, reviewer_id)
);

CREATE INDEX idx_reviewers_review_id ON core.reviewers (review_id);

CREATE TABLE core.review_verdicts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    diff_id UUID NOT NULL REFERENCES core.diffs(id) ON DELETE CASCADE,
    revision_id UUID NOT NULL REFERENCES core.revisions(id) ON DELETE CASCADE,
    reviewer_id UUID NOT NULL REFERENCES core.users(id),
    verdict core.verdict NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_review_verdicts_diff_id ON core.review_verdicts (diff_id);
CREATE INDEX idx_review_verdicts_reviewer_id ON core.review_verdicts (reviewer_id);

CREATE TABLE core.review_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    review_id UUID NOT NULL REFERENCES core.reviews(id) ON DELETE CASCADE,
    diff_id UUID NOT NULL REFERENCES core.diffs(id) ON DELETE CASCADE,
    revision_id UUID NOT NULL REFERENCES core.revisions(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES core.users(id),
    parent_id UUID REFERENCES core.review_comments(id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    file_path VARCHAR(1000),
    line_number_start INTEGER,
    line_number_end INTEGER,
    side core.comment_side,
    resolved BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_review_comments_review_id ON core.review_comments (review_id);
CREATE INDEX idx_review_comments_diff_id ON core.review_comments (diff_id);
CREATE INDEX idx_review_comments_parent_id ON core.review_comments (parent_id);
