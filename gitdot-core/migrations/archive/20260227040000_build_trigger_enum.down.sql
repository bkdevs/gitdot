ALTER TABLE builds ALTER COLUMN trigger TYPE TEXT USING trigger::TEXT;
DROP TYPE build_trigger;
