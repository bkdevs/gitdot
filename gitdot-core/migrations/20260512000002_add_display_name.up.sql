ALTER TABLE core.users RENAME COLUMN company TO display_name;
ALTER TABLE core.organizations ADD COLUMN display_name VARCHAR(255);
