DROP INDEX core.idx_reviews_repository_id_number;
ALTER TABLE core.reviews ADD CONSTRAINT reviews_repository_id_number_key
    UNIQUE (repository_id, number);
