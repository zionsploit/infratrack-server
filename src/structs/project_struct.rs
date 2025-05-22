use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;

use super::{contractors_struct::ReturnContractors, project_interface::{ReturnBarangays, ReturnCategories, ReturnIncharge, ReturnProjectTypes, ReturnSectors, ReturnSourceOfFunds, ReturnSustainableDevelopmentGoals}, project_status_structs::ResponseProjectStatus, project_takers::ReturnProjectTakers};

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

#[derive(Serialize, FromRow, Debug, TS, Default)]
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

#[derive(Serialize, Deserialize, Debug, TS, FromRow, Default)]
#[ts(export, export_to = "../../src/ServerTypes/Project.ts")]
pub struct ReturnProjectDetails {
    pub id: i32,
    pub contractor: i32,
    pub contract_cost: i32,
    pub start_date: NaiveDate,
    pub target_date: NaiveDate,
    pub day_extension: i32,
}

// TO VIEW
#[derive(Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Project.ts")]
pub struct ProjectFullDetails {
    pub project_details: ReturnProjectDetails,
    pub contractors: ReturnContractors,
    pub project_status: ResponseProjectStatus
}

#[derive(Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Project.ts")]
pub struct ProjectsFunded {
    pub projects: ReturnProject,
    pub project_full_details: ProjectFullDetails
}

#[derive(Serialize, Debug, TS, Default)]
#[ts(export, export_to = "../../src/ServerTypes/Project.ts")]
pub struct ProjectFullData {
    pub projects: ReturnProject,
    pub project_details: ReturnProjectDetails,
    pub project_brgy: ReturnBarangays,
    pub project_category: ReturnCategories,
    pub project_mode_of_implementation: ReturnIncharge,
    pub project_sector: ReturnSectors,
    pub project_source_of_funds: ReturnSourceOfFunds,
    pub project_status: ResponseProjectStatus,
    pub project_sustainable_development_goals: ReturnSustainableDevelopmentGoals,
    pub project_takers: ReturnProjectTakers,
    pub project_type: ReturnProjectTypes
}