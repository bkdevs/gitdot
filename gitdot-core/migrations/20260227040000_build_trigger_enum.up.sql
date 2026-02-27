CREATE TYPE build_trigger AS ENUM ('pull_request', 'push_to_main');
ALTER TABLE builds ALTER COLUMN trigger TYPE build_trigger USING trigger::build_trigger;
