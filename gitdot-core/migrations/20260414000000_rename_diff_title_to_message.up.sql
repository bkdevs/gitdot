ALTER TABLE core.diffs RENAME COLUMN title TO message;
ALTER TABLE core.diffs DROP COLUMN description;
