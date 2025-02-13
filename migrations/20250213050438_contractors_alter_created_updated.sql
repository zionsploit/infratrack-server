-- Add migration script here
-- ALTER TABLE TO ADD FORGOT COLUMN
ALTER TABLE contractors ADD COLUMN (
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)