use std::sync::Arc;

use axum::Extension;
use axum_test::TestServer;
use sqlx::mysql::MySqlPoolOptions;

use crate::{routes::project_takers_route::project_takers_route, structs::pool_conn_struct::PoolConnectionState};

async fn initialized_db_connection () -> Arc<PoolConnectionState> {
    let database_url = dotenvy::var("DATABASE_URL").unwrap() ;
    let mysql_pool = MySqlPoolOptions::new().connect(&database_url).await.unwrap();
    let conn_state = Arc::new(PoolConnectionState {
        connection: mysql_pool
    });

    conn_state
}

#[tokio::test]
async fn get_all_project_takers () {
    

    let app = project_takers_route()
        .layer(Extension(initialized_db_connection().await));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/get-all").await;

    // TEST FOR RESPONSE STATUS
    response.assert_status_ok();

    // TEST FOR RESPONSE BODY
}