use std::sync::Arc;

use sqlx::mysql::MySqlPoolOptions;
use tracing::{info, error};

use crate::structs::pool_conn_struct::PoolConnectionState;

pub async fn initialized_db_connection () -> Arc<PoolConnectionState> {
    let database_url = match dotenvy::var("DATABASE_URL") {
        Ok(result) => {
            info!("Initialized DB ENV");
            result
        },
        Err(err) => {
            error!("DB ERROR {err}");
            String::default()
        }
    };

    let mysql_pool =  MySqlPoolOptions::new().connect(&database_url).await;

    if let Err(ref err) = mysql_pool {
        error!("DB Connection ERROR {err}");
    } else {
        info!("Initialized DB Connection");
    }

    let conn_state = Arc::new(PoolConnectionState {
        connection: mysql_pool.unwrap()
    });

    conn_state
}