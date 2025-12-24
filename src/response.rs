use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
            total_size: None,
        }
    }
    
    pub fn error(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
            total_size: None,
        }
    }
}

impl<T> ApiResponse<Vec<T>>
where
    T: Serialize,
{
    pub fn success_list(data: Vec<T>) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            total_size: Some(data.len() as u64),
            data: Some(data),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
