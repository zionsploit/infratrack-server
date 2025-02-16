
use axum::Extension;
use axum_test::TestServer;

use crate::{routes::project_takers_route::project_takers_route, utils::db_connection::initialized_db_connection};

#[tokio::test]
async fn get_all_project_takers () {
    

    let app = project_takers_route()
        .layer(Extension(initialized_db_connection().await));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/get-all").await;

    response.assert_status_ok();
}