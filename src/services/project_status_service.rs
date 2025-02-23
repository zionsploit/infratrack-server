use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use http::StatusCode;

use crate::structs::{pool_conn_struct::PoolConnectionState, project_status_structs::ResponseProjectStatus};

pub async fn get_all_project_status (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
) -> impl IntoResponse {
    match sqlx::query_as("SELECT * FROM project_status").fetch_all(&sql_pool.connection.to_owned()).await {
        Ok(result) => {
            let result: Vec<ResponseProjectStatus> = result;
            (StatusCode::OK, Json(result))
        }, 
        Err(_) => (StatusCode::OK, Json(Vec::default()))
    }
}