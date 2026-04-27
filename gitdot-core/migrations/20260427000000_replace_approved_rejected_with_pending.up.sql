CREATE TYPE core.diff_status_new AS ENUM ('pending', 'open', 'merged');

ALTER TABLE core.diffs ALTER COLUMN status DROP DEFAULT;

ALTER TABLE core.diffs
    ALTER COLUMN status TYPE core.diff_status_new
    USING (
        CASE status::text
            WHEN 'approved' THEN 'open'::core.diff_status_new
            WHEN 'rejected' THEN 'open'::core.diff_status_new
            ELSE status::text::core.diff_status_new
        END
    );

DROP TYPE core.diff_status;
ALTER TYPE core.diff_status_new RENAME TO diff_status;

ALTER TABLE core.diffs ALTER COLUMN status SET DEFAULT 'pending'::core.diff_status;
