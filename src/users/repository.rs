use sqlx::MySqlPool;
use crate::error::AppError;
use super::model::{CreateUserRequest, User};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(pool: &MySqlPool, payload: CreateUserRequest) -> Result<User, AppError> {
        let result = sqlx::query(
            r#"
            INSERT INTO users (username, email)
            VALUES (?, ?)
            "#
        )
        .bind(&payload.username)
        .bind(&payload.email)
        .execute(pool)
        .await?;

        let id = result.last_insert_id() as i64;

        Ok(User {
            id,
            username: payload.username,
            email: payload.email,
        })
    }

    pub async fn find_all(pool: &MySqlPool) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, username, email FROM users"
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn find_by_id(pool: &MySqlPool, id: i64) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}
