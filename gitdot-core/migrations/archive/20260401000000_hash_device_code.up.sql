ALTER TABLE device_authorizations
    RENAME COLUMN device_code TO device_code_hash;

ALTER TABLE device_authorizations
    ALTER COLUMN device_code_hash TYPE VARCHAR(128);
