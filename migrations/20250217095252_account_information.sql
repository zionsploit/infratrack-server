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
)