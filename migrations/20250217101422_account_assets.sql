-- Add migration script here
CREATE TABLE IF NOT EXISTS account_assets (
    id Int NOT NULL AUTO_INCREMENT PRIMARY KEY,
    file_name VARCHAR(255) NOT NULL,
    file_ext VARCHAR(255) NOT NULL,
    file_url VARCHAR(255) NOT NULL,
    file_size VARCHAR(255) NOT NULL,
    file_hash VARCHAR(255) NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)