-- Add migration script here
CREATE TABLE IF NOT EXISTS account_information (
    id VARCHAR(255) NOT NULL UNIQUE KEY PRIMARY KEY,
    first_name VARCHAR(255),
    middle_name VARCHAR(255),
    last_name VARCHAR(255),
    email_address VARCHAR(255),
    contact_number VARCHAR(255),
    position VARCHAR(255),
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- ALTER TABLE ADD MISSING COLUMN REFERENCES

ALTER TABLE account_information ADD COLUMN (

    account_cred_id VARCHAR(255) NOT NULL,
    INDEX account_cred_id_ind (account_cred_id),
    FOREIGN KEY (account_cred_id) REFERENCES account_credentials(id) ON DELETE CASCADE,

    account_assets_id Int NOT NULL,
    INDEX account_assets_id_ind (account_assets_id),
    FOREIGN KEY (account_assets_id) REFERENCES account_assets(id) ON DELETE CASCADE,

    UNIQUE(account_cred_id, account_assets_id)
)