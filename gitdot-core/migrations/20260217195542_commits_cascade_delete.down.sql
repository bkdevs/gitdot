ALTER TABLE commits
    DROP CONSTRAINT commits_author_id_fkey,
    ADD CONSTRAINT commits_author_id_fkey FOREIGN KEY (author_id) REFERENCES users(id);

ALTER TABLE commits
    DROP CONSTRAINT commits_repo_id_fkey,
    ADD CONSTRAINT commits_repo_id_fkey FOREIGN KEY (repo_id) REFERENCES repositories(id);
