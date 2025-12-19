use sqlx::{mysql::MySqlPoolOptions, Pool, MySql};
use crate::error::AppError;

pub type DbConn = Pool<MySql>;

pub async fn init_db(database_url: &str) -> Result<DbConn, AppError> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Create table if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id BIGINT PRIMARY KEY AUTO_INCREMENT,
            username VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
