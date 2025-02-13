use axum::{routing::{get, post}, Router};

use crate::services::contractors_service::{add_contractors, delete_contractors_by_id, get_all_contractors, update_contractors_by_id};

pub fn contractors_route () -> Router {
    Router::new()
        .route("/add", post(add_contractors))
        .route("/update-by-id", post(update_contractors_by_id))
        .route("/delete-by-id", post(delete_contractors_by_id))
        .route("/get-all", get(get_all_contractors))
}