use chrono::{DateTime, Utc};
use ::serde::{Deserialize, Serialize};
use ts_rs::TS;


#[derive(Deserialize, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Account.ts")]
pub struct AccountCredentialsBase {
    pub username: String,
    pub recover_email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Account.ts")]
pub struct UpdateAccountCredentials {
    pub id: String,
    
    #[serde(flatten)]
    pub account_base: AccountCredentialsBase
}

#[derive(Deserialize, Serialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Account.ts")]
pub struct ReturnAccountCredentials {
    pub id: String,

    #[serde(flatten)]
    pub account_base: AccountCredentialsBase,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}