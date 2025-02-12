use axum::{routing::{get, post}, Router};

use crate::services::project_takers_service::{add_project_takers, delete_project_takers_by_id, get_all_project_takers, update_project_takers_by_id};

pub fn project_takers_route () -> Router {
    Router::new()
        .route("/add", post(add_project_takers))
        .route("/update-by-id", post(update_project_takers_by_id))
        .route("/delete-by-id", post(delete_project_takers_by_id))
        .route("/get-all", get(get_all_project_takers))
}