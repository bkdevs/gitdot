ALTER TABLE device_authorizations
    ALTER COLUMN device_code_hash TYPE VARCHAR(64);

ALTER TABLE device_authorizations
    RENAME COLUMN device_code_hash TO device_code;
