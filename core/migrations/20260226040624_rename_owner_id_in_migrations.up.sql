ALTER TABLE migrations RENAME COLUMN owner_id TO author_id;
ALTER INDEX idx_migrations_owner_id RENAME TO idx_migrations_author_id;
