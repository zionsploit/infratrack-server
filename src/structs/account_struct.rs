
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;

// REQUEST STRUCTS
#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Account.ts")]
pub struct AccountRegistration {
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
    pub username: String,
    pub password: String,
    pub confirmpassword: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Account.ts")]
pub struct AccountLogin {
    pub username: String,
    pub password: String
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Account.ts")]
pub struct TokenVerification {
    pub token: String
}

// RESPONSE STRUCTS
#[derive(FromRow, Default, Debug, Serialize, Deserialize)]
pub struct ReturnAccountInformation {
    pub id: i64,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub user_name: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}