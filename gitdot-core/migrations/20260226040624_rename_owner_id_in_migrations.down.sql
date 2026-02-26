ALTER TABLE migrations RENAME COLUMN author_id TO owner_id;
ALTER INDEX idx_migrations_author_id RENAME TO idx_migrations_owner_id;
