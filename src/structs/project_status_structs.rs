use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;
use ts_rs::TS;

#[derive(FromRow, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Project.ts")]
pub struct ResponseProjectStatus {
    pub id: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}