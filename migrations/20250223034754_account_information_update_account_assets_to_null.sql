-- Add migration script here

-- USED TO UPDATE THE account_assets_into_null since it's optional to add account assets like images as profile picture

ALTER TABLE account_information
CHANGE COLUMN account_assets_id account_assets_id INT NULL UNIQUE KEY;