use std::sync::Arc;

use axum::Extension;
use axum_test::TestServer;
use http::StatusCode;
use serde_json::json;
use sqlx::mysql::MySqlPoolOptions;

use crate::{routes::contractors_route::contractors_route, structs::{contractors_struct::{Contractors, UpdateContractorsById}, pool_conn_struct::PoolConnectionState}};

async fn initialized_db_connection () -> Arc<PoolConnectionState> {
    let database_url = dotenvy::var("DATABASE_URL").unwrap() ;
    let mysql_pool = MySqlPoolOptions::new().connect(&database_url).await.unwrap();
    let conn_state = Arc::new(PoolConnectionState {
        connection: mysql_pool
    });

    conn_state
}

#[tokio::test]
async fn add_contractors () {

    let app = contractors_route().layer(Extension(initialized_db_connection().await));

    let server = TestServer::new(app).unwrap();

    let request = Contractors {
        contractor_name: "Contractors Name".to_string(),
        contractor_email: "contractors@gmail.com".to_string(),
        contractor_address_street: "contractors street".to_string(),
        contractor_address_barangay: "contractors barangay".to_string(),
        contractor_address_municipality: "contractors municipality".to_string(),
        contractor_address_province: "contractors province".to_string(),
        contractor_description: "contractors descriptions".to_string(),
        contractor_contact_name: "contractors contact name".to_string(),
        contractor_contact_position: "contractors position".to_string(),
        contractor_contract_number: "contractors active number".to_string()
    };

    let response = server.post("/add").json(&request).await;

    response.assert_status(StatusCode::OK)
}

#[tokio::test]
async fn get_all_contractors () {

    let app = contractors_route().layer(Extension(initialized_db_connection().await));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/get-all").await;

    // CHANGE THE VALUE FOR EXPECTED RETURN
    let request: Vec<Contractors> = vec![Contractors  {
        contractor_name: "Contractors Name".to_string(),
        contractor_email: "contractors@gmail.com".to_string(),
        contractor_address_street: "contractors street".to_string(),
        contractor_address_barangay: "contractors barangay".to_string(),
        contractor_address_municipality: "contractors municipality".to_string(),
        contractor_address_province: "contractors province".to_string(),
        contractor_description: "contractors descriptions".to_string(),
        contractor_contact_name: "contractors contact name".to_string(),
        contractor_contact_position: "contractors position".to_string(),
        contractor_contract_number: "contractors active number".to_string()
    }];

    response.assert_status(StatusCode::OK);
    response.assert_json_contains(&json!(request));

}

#[tokio::test]
async fn update_contractors_by_id () {
    let app = contractors_route().layer(Extension(initialized_db_connection().await));

    let server = TestServer::new(app).unwrap();

    let request = UpdateContractorsById {
        id: 1,
        contractor_name: "Contractors Name updated".to_string(),
        contractor_email: "contractors@gmail.com".to_string(),
        contractor_address_street: "contractors street".to_string(),
        contractor_address_barangay: "contractors barangay".to_string(),
        contractor_address_municipality: "contractors municipality".to_string(),
        contractor_address_province: "contractors province".to_string(),
        contractor_description: "contractors descriptions".to_string(),
        contractor_contact_name: "contractors contact name".to_string(),
        contractor_contact_position: "contractors position".to_string(),
        contractor_contract_number: "contractors active number".to_string()
    };

    let response = server.post("/update-by-id").json(&request).await;

    response.assert_status_ok();
}

#[tokio::test]
pub async fn delete_contractors_by_id () {
    let app = contractors_route().layer(Extension(initialized_db_connection().await));

    let server = TestServer::new(app).unwrap();

    let response = server.post("/delete-by-id").json(&json!({
        "id": 1
    })).await;

    response.assert_status_ok();
}