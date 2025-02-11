use sqlx::{MySql, Pool};

pub struct PoolConnectionState {
    pub connection: Pool<MySql>
}