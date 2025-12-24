use axum::{extract::State, Json};
use crate::{
    error::AppError,
    state::AppState,
    users::{model::{CreateUserRequest, User}, UserRepository},
    response::ApiResponse,
};

/// Create a new user
#[utoipa::path(
    post,
    path = "/users",
    tag = "User",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = ApiResponse<User>)
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<ApiResponse<User>, AppError> {
    let user = UserRepository::create(&state.db, payload).await?;
    Ok(ApiResponse::success(user))
}
