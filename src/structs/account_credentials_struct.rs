use chrono::{DateTime, Utc};
use ::serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};
use ts_rs::TS;


#[derive(FromRow, Decode, Deserialize, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Account.ts")]
pub struct AccountCredentialsBase {
    pub username: String,
    pub recover_email: String,
    pub password: String,
}

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
pub struct UpdateAccountCredentials {
    pub id: String,
    
    #[serde(flatten)]
    pub account_base: AccountCredentialsBase
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