CREATE TABLE core.commits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repo_id UUID NOT NULL REFERENCES core.repositories(id) ON DELETE CASCADE,
    author_id UUID REFERENCES core.users(id) ON DELETE CASCADE,
    git_author_name TEXT NOT NULL DEFAULT '',
    git_author_email TEXT NOT NULL DEFAULT '',
    sha VARCHAR(40) NOT NULL,
    sha_short VARCHAR(7) GENERATED ALWAYS AS (substring(sha FROM 1 FOR 7)) STORED,
    parent_sha VARCHAR(40) NOT NULL DEFAULT '0000000000000000000000000000000000000000',
    ref_name VARCHAR(256) NOT NULL,
    message TEXT NOT NULL,
    review_number INTEGER,
    diff_position INTEGER,
    diffs JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(repo_id, sha)
);

CREATE INDEX idx_commits_repo_id ON core.commits (repo_id);
CREATE INDEX idx_commits_author_id ON core.commits (author_id);
CREATE INDEX idx_commits_sha ON core.commits (sha);
CREATE INDEX idx_commits_ref_name ON core.commits (ref_name);
CREATE INDEX idx_commits_created_at ON core.commits (created_at);
CREATE INDEX idx_commits_repo_id_sha_short ON core.commits (repo_id, sha_short);
