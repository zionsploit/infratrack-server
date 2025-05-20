pub mod pool_conn_struct;
pub mod account_struct;
pub mod project_interface;
pub mod project_takers;
pub mod contractors_struct;
pub mod project_struct;
pub mod account_credentials_struct;
pub mod account_assets;
pub mod account_information_struct;
pub mod project_status_structs;
pub mod basic_struct {
    use serde::{Deserialize, Serialize};
    use ts_rs::TS;


    #[derive(Deserialize, Debug, TS)]
    #[ts(export, export_to = "../../src/ServerTypes/BasicStruct.ts")]
    pub struct RequestById {
        pub id: u32
    }

    #[derive(Serialize, Debug, TS, Default)]
    #[ts(export, export_to = "../../src/ServerTypes/BasicStruct.ts")]
    pub struct ResponseWithId {
        pub id: u64,
        pub message: String
    } 
}