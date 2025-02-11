-- Add migration script here

CREATE TABLE IF NOT EXISTS pi_sdg (
    id Int NOT NULL AUTO_INCREMENT PRIMARY KEY,
    sdg_title VARCHAR(255) NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)