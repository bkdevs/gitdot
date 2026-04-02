ALTER TABLE tokens ADD CONSTRAINT access_tokens_user_id_fkey FOREIGN KEY (principal_id) REFERENCES users(id);
