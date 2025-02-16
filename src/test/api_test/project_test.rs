
use axum::Extension;
use axum_test::TestServer;
use chrono::DateTime;
use serde_json::json;

use crate::{routes::project_route::project_route, structs::project_struct::{Project, ProjectDetails, UpdateProjectById}, utils::db_connection::initialized_db_connection};

#[tokio::test]
async fn add_project_details () {

    let app = project_route().layer(Extension(initialized_db_connection().await));

    let server = TestServer::new(app).unwrap();

    let request = ProjectDetails {
        contractor: 1,
        contract_cost: 100_000_000,
        start_date: DateTime::default(),
        end_date: DateTime::default(),
        day_extension: 0
    };

    let response = server.post("/project-details-add").json(&request).await;

    response.assert_status_ok();

}

#[tokio::test]
async fn add_project () {

    let app = project_route().layer(Extension(initialized_db_connection().await));

    let server = TestServer::new(app).unwrap();

    let request = Project { 
        project_funded: "City Funded".to_string(), 
        project_code: "PRC-CODE".to_string(), 
        project_status_id: 1, 
        project_barangay_id: 2, 
        appropriation: 0, 
        approved_budget_contract: 0, 
        contract_detail_id: 1, 
        project_type_id: 1, 
        project_category_id: 1, 
        project_source_of_fund_id: 1, 
        project_mode_of_implementation_id: 1, 
        project_sustainable_developement_id: 1, 
        project_sector_id: 1, 
        project_taker_id: 1, 
        accomplished: 0, 
        remarks: "Remarks".to_string(), 
        prepared_by: "Developer".to_string() 
    } ;

    let respose = server.post("/add").json(&request).await;

    respose.assert_status_ok();

}

#[tokio::test]
async fn update_project_by_id () {

    let app = project_route().layer(Extension(initialized_db_connection().await));

    let server = TestServer::new(app).unwrap();

    let request = UpdateProjectById { 
        id: 11,
        project_funded: "City Updated Funded".to_string(), 
        project_code: "PRC-CODE".to_string(), 
        project_status_id: 1, 
        project_barangay_id: 2, 
        appropriation: 0, 
        approved_budget_contract: 0, 
        contract_detail_id: 1, 
        project_type_id: 1, 
        project_category_id: 1, 
        project_source_of_fund_id: 1, 
        project_mode_of_implementation_id: 1, 
        project_sustainable_developement_id: 1, 
        project_sector_id: 1, 
        project_taker_id: 1, 
        accomplished: 0, 
        remarks: "Remarks".to_string(), 
        prepared_by: "Developer".to_string() 
    } ;

    let respose = server.post("/update-by-id").json(&request).await;

    respose.assert_status_ok();
}

#[tokio::test]
async fn delete_project_by_id () {
    let app = project_route().layer(Extension(initialized_db_connection().await));

    let server = TestServer::new(app).unwrap();

    let respose = server.post("/delete-by-id").json(&json!({"id": 11})).await;

    respose.assert_status_ok();
}

#[tokio::test]
async fn get_all_projects () {
    let app = project_route().layer(Extension(initialized_db_connection().await));

    let server = TestServer::new(app).unwrap();

    let respose = server.get("/get-all").await;

    respose.assert_status_ok();
}