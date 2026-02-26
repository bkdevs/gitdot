ALTER TABLE commits ADD COLUMN ref_name VARCHAR(256) NOT NULL;

CREATE INDEX idx_commits_ref_name ON commits(ref_name);
