ALTER TABLE core.reviews DROP CONSTRAINT reviews_repository_id_number_key;
CREATE UNIQUE INDEX idx_reviews_repository_id_number
    ON core.reviews (repository_id, number)
    WHERE number <> -1;
