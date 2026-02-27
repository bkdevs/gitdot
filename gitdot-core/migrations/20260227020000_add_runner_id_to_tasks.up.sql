ALTER TABLE tasks ADD COLUMN runner_id UUID REFERENCES runners(id);
