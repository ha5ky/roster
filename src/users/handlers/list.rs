use axum::extract::State;
use crate::{
    error::AppError,
    state::AppState,
    users::{model::User, UserRepository},
    response::ApiResponse,
};

/// Get all users
#[utoipa::path(
    get,
    path = "/users",
    tag = "User",
    responses(
        (status = 200, description = "List all users", body = ApiResponse<Vec<User>>)
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<ApiResponse<Vec<User>>, AppError> {
    let users = UserRepository::find_all(&state.db).await?;
    Ok(ApiResponse::success_list(users))
}
