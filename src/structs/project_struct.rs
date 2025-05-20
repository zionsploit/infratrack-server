use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Project.ts")]
pub struct Project {
    pub project_details_id: i32,
    pub project_name: String,
    pub project_year: i32,
    pub project_funded: String,
    pub project_code: String,
    pub project_status_id: i32,
    pub project_barangay_id: i32,
    pub appropriation: i32,
    pub approved_budget_contract: i32,
    pub contract_detail_id: i32,
    pub project_type_id: i32,
    pub project_category_id: i32,
    pub project_source_of_fund_id: i32,
    pub project_mode_of_implementation_id: i32,
    pub project_sustainable_developement_id: i32,
    pub project_sector_id: i32,
    pub project_taker_id: i32,
    pub accomplished: u8,
    pub remarks: String,
    pub prepared_by: String,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Project.ts")]
pub struct UpdateProjectById {
    pub id: i32,
    pub project_name: String,
    pub project_funded: String,
    pub project_code: String,
    pub project_status_id: i32,
    pub project_barangay_id: i32,
    pub appropriation: i32,
    pub approved_budget_contract: i32,
    pub contract_detail_id: i32,
    pub project_type_id: i32,
    pub project_category_id: i32,
    pub project_source_of_fund_id: i32,
    pub project_mode_of_implementation_id: i32,
    pub project_sustainable_developement_id: i32,
    pub project_sector_id: i32,
    pub project_taker_id: i32,
    pub accomplished: u8,
    pub remarks: String,
    pub prepared_by: String,
}

#[derive(Serialize, FromRow, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Project.ts")]
pub struct ReturnProject {
    pub id: i32,
    pub project_name: String,
    pub project_funded: String,
    pub project_year: u32,
    pub project_code: String,
    pub project_status_id: i32,
    pub project_barangay_id: i32,
    pub appropriation: i32,
    pub approved_budget_contract: i32,
    pub contract_detail_id: i32,
    pub project_type_id: i32,
    pub project_category_id: i32,
    pub project_source_of_fund_id: i32,
    pub project_mode_of_implementation_id: i32,
    pub project_sustainable_developement_id: i32,
    pub project_sector_id: i32,
    pub project_taker_id: i32,
    pub accomplished: u8,
    pub remarks: String,
    pub prepared_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Project.ts")]
pub struct ProjectDetails {
    pub contractor: i32,
    pub contract_cost: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub day_extension: i32,
}