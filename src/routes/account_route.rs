use axum::{routing::post, Router};

use crate::services::account_service::{add_account_credentials, login_account, verify_token};

pub fn account_routes () -> Router {
    Router::new()
        .route("/login", post(login_account ))
        .route("/verify-token", post(verify_token ))
        // NEW API IMPLEMENTATIONS
        .route("/add-credentials", post(add_account_credentials))
}