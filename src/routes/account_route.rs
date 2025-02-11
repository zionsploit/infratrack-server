use axum::{routing::post, Router};

use crate::services::account_service::{login_account, register_account, verify_token};

pub fn account_routes () -> Router {
    Router::new()
        .route("/registration", post(register_account))
        .route("/login", post(login_account ))
        .route("/verify-token", post(verify_token ))
}