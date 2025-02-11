pub mod pool_conn_struct;
pub mod account_struct;
pub mod project_interface;

pub mod basic_struct {
    use serde::Deserialize;
    use ts_rs::TS;


    #[derive(Deserialize, Debug, TS)]
    #[ts(export, export_to = "../../src/ServerTypes/BasicStruct.ts")]
    pub struct  RequestById {
        pub id: u32
    }
}