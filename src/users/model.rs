use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, FromRow)]
pub struct User {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = "alice")]
    pub username: String,
    #[schema(example = "alice@example.com")]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    #[schema(example = "alice")]
    pub username: String,
    #[schema(example = "alice@example.com")]
    pub email: String,
}
