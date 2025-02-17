-- Add migration script here
CREATE TABLE IF NOT EXISTS account_credentials (
    id VARCHAR(255) NOT NULL UNIQUE KEY PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    recovery_email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)