use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Contractors.ts")]
pub struct Contractors {
    pub contractor_name: String,
    pub contractor_email: String,
    pub contractor_address_street: String,
    pub contractor_address_barangay: String,
    pub contractor_address_municipality: String,
    pub contractor_address_province: String,
    pub contractor_description: String,
    pub contractor_contact_name: String,
    pub contractor_contact_position: String,
    pub contractor_contract_number: String
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Contractors.ts")]
pub struct UpdateContractorsById {
    pub id: i32,
    pub contractor_name: String,
    pub contractor_email: String,
    pub contractor_address_street: String,
    pub contractor_address_barangay: String,
    pub contractor_address_municipality: String,
    pub contractor_address_province: String,
    pub contractor_description: String,
    pub contractor_contact_name: String,
    pub contractor_contact_position: String,
    pub contractor_contract_number: String
}

#[derive(FromRow, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Contractors.ts")]
pub struct ReturnContractors {
    pub id: i32,
    pub contractor_name: String,
    pub contractor_email: String,
    pub contractor_address_street: String,
    pub contractor_address_barangay: String,
    pub contractor_address_municipality: String,
    pub contractor_address_province: String,
    pub contractor_description: String,
    pub contractor_contact_name: String,
    pub contractor_contact_position: String,
    pub contractor_contract_number: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}