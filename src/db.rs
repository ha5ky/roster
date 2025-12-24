use sqlx::{mysql::{MySqlPoolOptions, MySqlConnectOptions}, Pool, MySql, ConnectOptions};
use std::str::FromStr;
use crate::error::AppError;
use tracing::log::LevelFilter;

pub type DbConn = Pool<MySql>;

pub async fn init_db(database_url: &str) -> Result<DbConn, AppError> {
    let connection_options = MySqlConnectOptions::from_str(database_url)?
        .log_statements(LevelFilter::Info)
        .log_slow_statements(LevelFilter::Warn, std::time::Duration::from_secs(1));

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await?;

    // Create table if not exists
    // sqlx::query(
    //     r#"
    //     CREATE TABLE IF NOT EXISTS users (
    //         id BIGINT PRIMARY KEY AUTO_INCREMENT,
    //         username VARCHAR(255) NOT NULL,
    //         email VARCHAR(255) NOT NULL
    //     )
    //     "#,
    // )
    // .execute(&pool)
    // .await?;

    Ok(pool)
}
