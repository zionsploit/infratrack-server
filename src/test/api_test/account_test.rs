use axum::Extension;
use axum_test::TestServer;
use serde_json::json;

use crate::{routes::account_route::account_routes, utils::db_connection::initialized_db_connection};

#[tokio::test]
async fn add_account_credentials () {
    let app = account_routes().layer(Extension(initialized_db_connection().await));

    let server = TestServer::new(app).unwrap();

    let response = server.post("/add-credentials").json(&json!({
        "username": "test_username_3",
        "recover_email": "test_recovery_123@gmail.com",
        "confirm_password": "password",
        "password": "password",
    })).await;

    response.assert_status_ok();
}