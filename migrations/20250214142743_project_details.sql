-- Add migration script here
CREATE TABLE IF NOT EXISTS project_details (
    id Int NOT NULL AUTO_INCREMENT UNIQUE KEY PRIMARY KEY,
    
    contractor Int,
    INDEX contractor_ind (contractor),
    FOREIGN KEY (contractor) REFERENCES contractors(id) ON DELETE CASCADE,

    contract_cost Int,
    start_date DATE,
    target_date DATE,
    day_extension Int,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)