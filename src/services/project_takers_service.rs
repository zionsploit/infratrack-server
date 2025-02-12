use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use http::StatusCode;

use crate::{enums::response_enum::{ResponseErrorMessage, ResponseOkMessage}, structs::{basic_struct::RequestById, pool_conn_struct::PoolConnectionState, project_takers::{ProjectTakers, ReturnProjectTakers, UpdateProjectTakersById}}};

pub async fn add_project_takers (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<ProjectTakers>
) -> impl IntoResponse {

    let find_same_name: Result<ReturnProjectTakers, sqlx::Error> = sqlx::query_as("SELECT * FROM project_takers WHERE first_name LIKE ? AND middle_name LIKE ? AND last_name LIKE ?")
        .bind(format!("%{}%", &request.first_name))
        .bind(format!("%{}%", &request.middle_name))
        .bind(format!("%{}%", &request.last_name))
        .fetch_one(&sql_pool.connection).await;



    if let Ok(_) = find_same_name {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("INSERT INTO project_takers (first_name, middle_name, last_name, profession, phone_number) VALUES (?, ?, ?, ?, ?)")
        .bind(request.first_name)
        .bind(request.middle_name)
        .bind(request.last_name)
        .bind(request.profession)
        .bind(request.phone_number)
        .execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
        }
}

pub async fn update_project_takers_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UpdateProjectTakersById>
) -> impl IntoResponse {

    let check_if_exists: Result<ReturnProjectTakers, sqlx::Error> = sqlx::query_as("SELECT * FROM project_takers WHERE id = ?")
        .bind(&request.id).fetch_one(&sql_pool.connection).await;

    if let Err(_) = check_if_exists {
        return (StatusCode::NOT_FOUND, format!("{:?}", ResponseErrorMessage::DataNotFound));
    } else {
        let return_data = check_if_exists.unwrap();

        if return_data.first_name.eq(&request.first_name)
            && return_data.middle_name.eq(&request.middle_name)
            && return_data.last_name.eq(&request.last_name) {
            return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
        }
    }

    match sqlx::query("UPDATE project_takers SET first_name = ?, middle_name = ?, last_name = ?, profession = ?, phone_number = ? WHERE id = ?")
        .bind(request.first_name)
        .bind(request.middle_name)
        .bind(request.last_name)
        .bind(request.profession)
        .bind(request.phone_number)
        .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataUpdate)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotModified))
    }

}

pub async fn delete_project_takers_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestById>
) -> impl IntoResponse {

    match sqlx::query("DELETE FROM project_takers WHERE id = ?")
        .bind(request.id)
        .execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::DataDeleteSuccess)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotDelete))
    }

}

pub async fn get_all_project_takers (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>
) -> impl IntoResponse {

    match sqlx::query_as("SELECT * FROM project_takers ORDER BY id DESC").fetch_all(&sql_pool.connection.to_owned()).await {
        Ok(result) => {
            let result: Vec<ReturnProjectTakers> = result;
            (StatusCode::OK, Json(result))
        },
        Err(_) => (StatusCode::OK, Json(Vec::default()))
    }

}