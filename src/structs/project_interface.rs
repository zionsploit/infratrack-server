use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct SustainableDevelopmentGoals {
    pub sdg_title: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct UpdateSustainableDevelopmentGoalsById {
    pub id: u32,
    pub sdg_title: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct SourceOfFunds {
    pub sof_title: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct UpdateSourceOfFundsById {
    pub id: u32,
    pub sof_title: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct ProjectTypes {
    pub pt_title: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct UpdateProjectTypesById {
    pub id: u32,
    pub pt_title: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct Incharge {
    pub incharge: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct UpdateInchargeById {
    pub id: u32,
    pub incharge: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct Categories {
    pub categories: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct UpdateCategoriesById {
    pub id: u32,
    pub categories: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct Sectors {
    pub sectors: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct UpdateSectorsById {
    pub id: u32,
    pub sectors: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct Barangays {
    pub barangays: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct UpdateBarangaysById {
    pub id: u32,
    pub barangays: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct UnitOfMeasurements {
    pub um_title: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct UpdateUnitOfMeasurementsById {
    pub id: u32,
    pub um_title: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct ProjectScope {
    pub ps_title: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct UpdateProjectScopeById {
    pub id: u32,
    pub ps_title: String
}

// RETURN
#[derive(FromRow, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct ReturnSustainableDevelopmentGoals {
    pub id: i32,
    pub sdg_title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct ReturnSourceOfFunds {
    pub id: i32,
    pub sof_title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct ReturnProjectTypes {
    pub id: i32,
    pub pt_title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct ReturnIncharge {
    pub id: i32,
    pub incharge: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct ReturnCategories {
    pub id: i32,
    pub categories: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(FromRow, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct ReturnSectors {
    pub id: i32,
    pub sectors: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(FromRow, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct ReturnBarangays {
    pub id: i32,
    pub barangays: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(FromRow, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct ReturnUnitOfMeasurements {
    pub id: i32,
    pub um_title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(FromRow, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectInterface.ts")]
pub struct ReturnProjectScope {
    pub id: i32,
    pub ps_title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}