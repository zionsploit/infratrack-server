use axum::{routing::{get, post}, Router};

use crate::services::{project_service::{add_project, add_project_details, delete_project_by_id, get_all_projects, get_project_by_funded, get_project_by_id, get_project_by_project_code, update_project_by_id}, project_status_service::get_all_project_status};

pub fn project_route () -> Router {
    Router::new()
        .route("/project-details-add", post(add_project_details))
        .route("/add", post(add_project))
        .route("/update-by-id", post(update_project_by_id))
        .route("/delete-by-id", post(delete_project_by_id))
        .route("/get/{project_id}", get(get_project_by_id))
        .route("/get-all", get(get_all_projects))
        .route("/get-all/{project_funded}", get(get_project_by_funded))
        .route("/get-project-by-project-code/{project_code}", get(get_project_by_project_code))
        // PROJECT STATUS
        .route("/get-all-project-status", get(get_all_project_status))
}