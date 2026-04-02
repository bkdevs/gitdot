ALTER TABLE device_authorizations
    DROP CONSTRAINT device_authorizations_user_id_fkey,
    ADD CONSTRAINT device_authorizations_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users(id);
