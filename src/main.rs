mod routes;
mod structs;
mod services;
mod enums;
mod utils;
mod test;

use std::{net::SocketAddr, sync::Arc};
use axum::{http::Method, Extension, Router};
use axum_analytics::Analytics;
use routes::route::api_routes;
use sqlx::mysql::MySqlPoolOptions;
use structs::pool_conn_struct::PoolConnectionState;
use tower_http::cors::{AllowHeaders, CorsLayer};
use tower_http::cors::AllowOrigin;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_url = match dotenvy::var("DATABASE_URL") {
        Ok(value) => {
            info!("DATABASE URL ESTABLISHED: {value}");
            value
        },
        Err(error) => {
            error!("ERROR: {}", error);
            return
        }
    };

    let analytics_api_key = dotenvy::var("API_KEY");

    if let Ok(_) = analytics_api_key {
        info!("Initialized Api Analytics");
    };


    let mysql_pool = match MySqlPoolOptions::new().connect(&database_url).await {
        Ok(connection) => {
            info!("DB Connection Established");
            connection
        },
        Err(err) => {
            info!("DB Conn Error: {} ", err);
            return
        }
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(AllowOrigin::any())
        .allow_headers(AllowHeaders::any());

    let conn_state = Arc::new(PoolConnectionState {
        connection: mysql_pool
    });

    let app = Router::new()
        .nest("/api", api_routes())
        .layer(Extension(conn_state))
        .layer(Analytics::new(analytics_api_key.unwrap().to_owned()))
        .layer(cors);


    let addr = SocketAddr::from(([0,0,0,0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service()).await.unwrap()
}
