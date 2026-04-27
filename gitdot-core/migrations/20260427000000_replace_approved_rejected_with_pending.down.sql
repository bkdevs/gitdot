CREATE TYPE core.diff_status_old AS ENUM ('open', 'rejected', 'approved', 'merged');

ALTER TABLE core.diffs
    ALTER COLUMN status TYPE core.diff_status_old
    USING (
        CASE status::text
            WHEN 'pending' THEN 'open'::core.diff_status_old
            ELSE status::text::core.diff_status_old
        END
    );

ALTER TABLE core.diffs ALTER COLUMN status SET DEFAULT 'open'::core.diff_status_old;

DROP TYPE core.diff_status;
ALTER TYPE core.diff_status_old RENAME TO diff_status;
