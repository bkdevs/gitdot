ALTER TABLE core.commits
  ADD COLUMN owner_name TEXT NOT NULL DEFAULT 'pybbae',
  ADD COLUMN repo_name TEXT NOT NULL DEFAULT 'gitdot-desktop';
