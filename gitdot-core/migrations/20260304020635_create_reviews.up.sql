-- Enums
CREATE TYPE review_status AS ENUM ('draft', 'open', 'changesrequested', 'approved', 'merged');
CREATE TYPE diff_status AS ENUM ('open', 'changesrequested', 'approved', 'merged');
CREATE TYPE reviewer_status AS ENUM ('pending', 'changesrequested', 'approved');
CREATE TYPE comment_side AS ENUM ('old', 'new');

-- Reviews
CREATE TABLE IF NOT EXISTS reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    number INT NOT NULL,
    author_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    target_branch VARCHAR(255) NOT NULL,
    status review_status NOT NULL DEFAULT 'draft',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(repository_id, number)
);

CREATE INDEX idx_reviews_repository_id ON reviews(repository_id);
CREATE INDEX idx_reviews_author_id ON reviews(author_id);
CREATE INDEX idx_reviews_status ON reviews(status);

-- Diffs (stacked diffs within a review)
CREATE TABLE IF NOT EXISTS diffs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    review_id UUID NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    position INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    status diff_status NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(review_id, position)
);

CREATE INDEX idx_diffs_review_id ON diffs(review_id);

-- Revisions (iterations of a diff)
CREATE TABLE IF NOT EXISTS revisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    diff_id UUID NOT NULL REFERENCES diffs(id) ON DELETE CASCADE,
    number INT NOT NULL,
    commit_hash VARCHAR(40) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(diff_id, number)
);

CREATE INDEX idx_revisions_diff_id ON revisions(diff_id);

-- Reviewers
CREATE TABLE IF NOT EXISTS reviewers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    review_id UUID NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    reviewer_id UUID NOT NULL REFERENCES users(id),
    status reviewer_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(review_id, reviewer_id)
);

CREATE INDEX idx_reviewers_review_id ON reviewers(review_id);

-- Review comments
CREATE TABLE IF NOT EXISTS review_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    review_id UUID NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    diff_id UUID REFERENCES diffs(id) ON DELETE CASCADE,
    revision_id UUID REFERENCES revisions(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id),
    parent_id UUID REFERENCES review_comments(id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    file_path VARCHAR(1000),
    line_number INT,
    side comment_side,
    resolved BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_review_comments_review_id ON review_comments(review_id);
CREATE INDEX idx_review_comments_diff_id ON review_comments(diff_id);
CREATE INDEX idx_review_comments_parent_id ON review_comments(parent_id);
