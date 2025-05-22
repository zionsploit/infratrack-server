use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectTakers.ts")]
pub struct ProjectTakers {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub profession: String,
    pub phone_number: String,
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectTakers.ts")]
pub struct UpdateProjectTakersById {
    pub id: i32,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub profession: String,
    pub phone_number: String,
}

#[derive(FromRow, Serialize, Debug, TS, Default)]
#[ts(export, export_to = "../../src/ServerTypes/ProjectTakers.ts")]
pub struct ReturnProjectTakers {
    pub id: i32,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub profession: String,
    pub phone_number: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}