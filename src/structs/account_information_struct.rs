use serde::Deserialize;
use ts_rs::TS;


#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/ServerTypes/Account.ts")]
pub struct RequestAccountInformation {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub email_address: String,
    pub contact_number: String,
    pub position: String,
    pub account_cred_id: String,
    pub account_assets_id: i32,
}