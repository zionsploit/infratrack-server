-- Add migration script here
CREATE TABLE IF NOT EXISTS contractors (
    id Int NOT NULL AUTO_INCREMENT PRIMARY KEY,
    contractor_name VARCHAR(255) NOT NULL,
    contractor_email VARCHAR(255),
    contractor_address_street VARCHAR(255) NOT NULL,
    contractor_address_barangay VARCHAR(255) NOT NULL,
    contractor_address_municipality VARCHAR(255) NOT NULL,
    contractor_address_province VARCHAR(255) NOT NULL,
    contractor_description TEXT,
    contractor_contact_name VARCHAR(255) NOT NULL,
    contractor_contact_position VARCHAR(255),
    contractor_contract_number VARCHAR(255)
)