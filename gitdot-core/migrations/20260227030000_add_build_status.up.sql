CREATE TYPE build_status AS ENUM ('running', 'success', 'failure');

ALTER TABLE builds ADD COLUMN status build_status NOT NULL DEFAULT 'running';
