use chrono::{DateTime, Utc};
use ::serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};
use ts_rs::TS;

#[derive(FromRow, Decode, Deserialize, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Account.ts")]
pub struct RequestAccountCredentials {
    pub username: String,
    pub recover_email: String,
    pub password: String,
    pub confirm_password: String
}

#[derive(FromRow, Deserialize, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Account.ts")]
pub struct ResponseAccountCredentials {
    pub id: String,
    pub username: String,
    pub recovery_email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug, Serialize)]
pub struct JwtStructure {
    pub id: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}