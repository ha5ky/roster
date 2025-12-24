use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use crate::response::ApiResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] std::env::VarError),

    #[error("Not found")]
    NotFound,

    #[error("Internal server error: {0}")]
    Anyhow(#[from] anyhow::Error),
    
    // Add more error types here
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, 500, format!("Database error: {}", e)),
            AppError::ConfigError(e) => (StatusCode::INTERNAL_SERVER_ERROR, 500, format!("Configuration error: {}", e)),
            AppError::NotFound => (StatusCode::NOT_FOUND, 404, "Resource not found".to_string()),
            AppError::Anyhow(e) => (StatusCode::INTERNAL_SERVER_ERROR, 500, e.to_string()),
        };

        let response: ApiResponse<()> = ApiResponse::error(code, message);

        (status, Json(response)).into_response()
    }
}
