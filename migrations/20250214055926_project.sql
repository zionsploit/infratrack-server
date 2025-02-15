-- Add migration script here
CREATE TABLE IF NOT EXISTS projects (
    id Int NOT NULL AUTO_INCREMENT UNIQUE KEY PRIMARY KEY,
    project_funded VARCHAR(255) NOT NULL,
    project_year YEAR DEFAULT NOW(),
    project_code VARCHAR(255) NOT NULL,
    
    project_status_id Int,
    INDEX project_status_id_ind (project_status_id),
    FOREIGN KEY (project_status_id) REFERENCES project_status(id) ON DELETE CASCADE,

    project_barangay_id Int,
    INDEX project_barangay_id_ind (project_barangay_id),
    FOREIGN KEY (project_barangay_id) REFERENCES pi_barangays(id) ON DELETE CASCADE,
    
    appropriation Int NOT NULL,
    approved_budget_contract Int NOT NULL,

    contract_detail_id Int,
    INDEX contract_detail_id_ind (contract_detail_id),
    FOREIGN KEY (contract_detail_id) REFERENCES project_details(id) ON DELETE CASCADE,

    project_type_id Int,
    INDEX project_type_id_ind (project_type_id),
    FOREIGN KEY (project_type_id) REFERENCES pi_pt(id) ON DELETE CASCADE,

    project_category_id Int,
    INDEX project_category_id_ind (project_category_id),
    FOREIGN KEY (project_category_id) REFERENCES pi_categories(id) ON DELETE CASCADE,

    project_source_of_fund_id Int,
    INDEX project_source_of_fund_id_ind (project_source_of_fund_id),
    FOREIGN KEY (project_source_of_fund_id) REFERENCES pi_sof(id) ON DELETE CASCADE,

    project_mode_of_implementation_id Int,
    INDEX project_mode_of_implementation_id_ind (project_mode_of_implementation_id),
    FOREIGN KEY (project_mode_of_implementation_id) REFERENCES pi_incharge(id) ON DELETE CASCADE,

    project_sustainable_developement_id Int,
    INDEX project_sustainable_developement_id_ind (project_sustainable_developement_id),
    FOREIGN KEY (project_sustainable_developement_id) REFERENCES pi_sdg(id) ON DELETE CASCADE,

    project_sector_id Int,
    INDEX project_sector_id_ind (project_sector_id),
    FOREIGN KEY (project_sector_id) REFERENCES pi_sectors(id) ON DELETE CASCADE,

    project_taker_id Int,
    INDEX project_taker_id_ind (project_taker_id),
    FOREIGN KEY (project_taker_id) REFERENCES project_takers(id) ON DELETE CASCADE,

    accomplished TinyInt Unsigned,
    check (accomplished BETWEEN 0 AND 100),
    
    remarks TEXT,
    prepared_by VARCHAR(255),
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)