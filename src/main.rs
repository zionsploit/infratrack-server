mod routes;
mod structs;
mod services;
mod enums;
mod utils;
mod test;

use std::net::SocketAddr;
use std::str::FromStr;
use axum::{http::Method, Extension, Router};
use axum_analytics::Analytics;
use routes::route::api_routes;
use tower_http::cors::{AllowHeaders, CorsLayer};
use tower_http::cors::AllowOrigin;
use tracing::info;
use tracing_subscriber;
use utils::db_connection::initialized_db_connection;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let analytics_api_key = dotenvy::var("API_KEY");

    let host_server = dotenvy::var("HOST_SERVER").expect("Missing Host Server ENV");

    if let Ok(_) = analytics_api_key {
        info!("Initialized Api Analytics");
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(AllowOrigin::any())
        .allow_headers(AllowHeaders::any());

    let app = Router::new()
        .nest("/api", api_routes())
        .layer(Extension(initialized_db_connection().await))
        .layer(Analytics::new(analytics_api_key.unwrap().to_owned()))
        .layer(cors);


    let addr = SocketAddr::from_str(&host_server).unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service()).await.unwrap()
}
