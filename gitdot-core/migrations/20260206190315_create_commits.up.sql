-- Create commits table
CREATE TABLE IF NOT EXISTS commits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES users(id),
    repo_id UUID NOT NULL REFERENCES repositories(id),
    sha VARCHAR(40) NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(repo_id, sha)
);

-- Create indexes for better query performance
CREATE INDEX idx_commits_author_id ON commits(author_id);
CREATE INDEX idx_commits_repo_id ON commits(repo_id);
CREATE INDEX idx_commits_sha ON commits(sha);
CREATE INDEX idx_commits_created_at ON commits(created_at);
