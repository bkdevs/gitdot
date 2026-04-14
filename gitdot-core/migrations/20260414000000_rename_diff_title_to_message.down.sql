ALTER TABLE core.diffs ADD COLUMN description TEXT NOT NULL DEFAULT '';
ALTER TABLE core.diffs RENAME COLUMN message TO title;
