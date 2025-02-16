use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use chrono::{Datelike, Local};
use http::StatusCode;

use crate::{enums::response_enum::{ResponseErrorMessage, ResponseOkMessage}, structs::{basic_struct::RequestById, pool_conn_struct::PoolConnectionState, project_struct::{Project, ProjectDetails, ReturnProject, UpdateProjectById}}};

pub async fn add_project_details (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<ProjectDetails>
) -> impl IntoResponse {

    match sqlx::query("INSERT INTO project_details (contractor, contract_cost, start_date, target_date, day_extension)
            VALUES (?, ?, ?, ?, ?)")
                .bind(request.contractor)
                .bind(request.contract_cost)
                .bind(request.start_date)
                .bind(request.end_date)
                .bind(request.day_extension).execute(&sql_pool.connection.to_owned()).await {
                    Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
                    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
    }

}

pub async fn add_project (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<Project>
) -> impl IntoResponse {

    let now = Local::now();

    let find_same_project_code: Result<ReturnProject, sqlx::Error> = sqlx::query_as("SELECT * FROM projects WHERE project_code = ?")
        .bind(&request.project_code).fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_project_code {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("INSERT INTO projects 
        (project_funded, project_year, project_code, project_status_id, project_barangay_id, appropriation, approved_budget_contract, contract_detail_id, project_type_id, project_category_id, project_source_of_fund_id, project_mode_of_implementation_id, project_sustainable_developement_id, project_sector_id, project_taker_id, accomplished, remarks, prepared_by)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(request.project_funded)
            .bind(now.year())
            .bind(request.project_code)
            .bind(request.project_status_id)
            .bind(request.project_barangay_id)
            .bind(request.appropriation)
            .bind(request.approved_budget_contract)
            .bind(request.contract_detail_id)
            .bind(request.project_type_id)
            .bind(request.project_category_id)
            .bind(request.project_source_of_fund_id)
            .bind(request.project_mode_of_implementation_id)
            .bind(request.project_sustainable_developement_id)
            .bind(request.project_sector_id)
            .bind(request.project_taker_id)
            .bind(request.accomplished)
            .bind(request.remarks)
            .bind(request.prepared_by).execute(&sql_pool.connection.to_owned()).await {
                Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
            }
}

pub async fn update_project_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UpdateProjectById>
) -> impl IntoResponse {

    match sqlx::query("UPDATE projects 
        SET project_funded = ?,  project_code = ?, project_status_id = ?, project_barangay_id = ?, appropriation = ?,
            approved_budget_contract = ?, contract_detail_id = ?, project_type_id = ?, project_category_id = ?, project_source_of_fund_id = ?,
            project_mode_of_implementation_id = ?, project_sustainable_developement_id = ?, project_sector_id = ?, project_taker_id = ?, accomplished = ?,
            remarks = ?, prepared_by = ? WHERE id = ?")
                .bind(request.project_funded)
                .bind(request.project_code)
                .bind(request.project_status_id)
                .bind(request.project_barangay_id)
                .bind(request.appropriation)
                .bind(request.approved_budget_contract)
                .bind(request.contract_detail_id)
                .bind(request.project_type_id)
                .bind(request.project_category_id)
                .bind(request.project_source_of_fund_id)
                .bind(request.project_mode_of_implementation_id)
                .bind(request.project_sustainable_developement_id)
                .bind(request.project_sector_id)
                .bind(request.project_taker_id)
                .bind(request.accomplished)
                .bind(request.remarks)
                .bind(request.prepared_by)
                .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
                    Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataUpdate)),
                    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
                }

}

pub async fn delete_project_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestById>
) -> impl IntoResponse {

    match sqlx::query("DELETE FROM projects WHERE id = ?").bind(request.id).execute(&sql_pool.connection.to_owned()).await {
        Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::DataDeleteSuccess)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
    }

}

pub async fn get_all_projects (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>
) -> impl IntoResponse {

    match sqlx::query_as("SELECT * FROM projects ORDER BY id DESC").fetch_all(&sql_pool.connection.to_owned()).await {
        Ok(result) => {
            let result: Vec<ReturnProject> = result;
            (StatusCode::OK, Json(result))
        },
        Err(_) => {
            (StatusCode::OK, Json(Vec::default()))
        }
    }

}