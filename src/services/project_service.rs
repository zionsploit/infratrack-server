use std::sync::Arc;

use axum::{extract::Path, response::IntoResponse, Extension, Json};
use http::StatusCode;

use crate::{enums::response_enum::{ResponseErrorMessage, ResponseOkMessage}, structs::{basic_struct::{RequestById, ResponseWithId}, contractors_struct::ReturnContractors, pool_conn_struct::PoolConnectionState, project_status_structs::ResponseProjectStatus, project_struct::{Project, ProjectDetails, ProjectFullDetails, ProjectsFunded, ReturnProject, ReturnProjectDetails, UpdateProjectById}}};

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
                    Ok(results) => {
                        let response = ResponseWithId {
                            id: results.last_insert_id(),
                            message: String::from("NewAccountIsCreated")
                        };
                        
                        (StatusCode::OK, Json(response))
                    },
                    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json::default())
    }

}

pub async fn add_project (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<Project>
) -> impl IntoResponse {

    let find_same_project_code: Result<ReturnProject, sqlx::Error> = sqlx::query_as("SELECT * FROM projects WHERE project_code = ?")
        .bind(&request.project_code).fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_project_code {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("INSERT INTO projects 
        (contract_detail_id, project_name, project_funded, project_year, project_code, project_status_id, project_barangay_id, appropriation, approved_budget_contract, project_type_id, project_category_id, project_source_of_fund_id, project_mode_of_implementation_id, project_sustainable_developement_id, project_sector_id, project_taker_id, accomplished, remarks, prepared_by)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(request.project_details_id)
            .bind(request.project_name)
            .bind(request.project_funded)
            .bind(request.project_year)
            .bind(request.project_code)
            .bind(request.project_status_id)
            .bind(request.project_barangay_id)
            .bind(request.appropriation)
            .bind(request.approved_budget_contract)
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
                Err(err) => {
                    println!("{:?}", err);
                    return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError));
                }
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

pub async fn get_project_by_funded (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Path(project_funded): Path<String>
) -> impl IntoResponse {
    
    match sqlx::query_as("SELECT * FROM projects WHERE project_funded = ?")
        .bind(project_funded).fetch_all(&sql_pool.connection.to_owned()).await {
            Ok(result) => {
                let result: Vec<ReturnProject> = result;
                let mut projects_funded: Vec<ProjectsFunded> = Vec::new();

                for project in result {
                    let project_details: ReturnProjectDetails = sqlx::query_as("SELECT * FROM project_details where id = ?")
                        .bind(project.contract_detail_id).fetch_one(&sql_pool.connection.to_owned()).await.unwrap();

                    let get_project_contractors: ReturnContractors = sqlx::query_as("SELECT * FROM contractors WHERE id = ?")
                        .bind(project_details.contractor).fetch_one(&sql_pool.connection.to_owned()).await.unwrap();

                    let get_project_status: ResponseProjectStatus = sqlx::query_as("SELECT * FROM project_status WHERE id = ?")
                        .bind(project.project_status_id).fetch_one(&sql_pool.connection.to_owned()).await.unwrap();

                    projects_funded.push(ProjectsFunded { projects: project, project_full_details: ProjectFullDetails {
                        project_details: project_details,
                        contractors: get_project_contractors,
                        project_status: get_project_status
                    } });
                }

                (StatusCode::OK, Json(projects_funded))
            },
            Err(_) => (StatusCode::OK, Json::default())
    }
}