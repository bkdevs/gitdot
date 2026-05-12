ALTER TABLE core.organizations DROP COLUMN display_name;
ALTER TABLE core.users RENAME COLUMN display_name TO company;
