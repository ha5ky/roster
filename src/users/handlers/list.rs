use axum::{extract::State, Json};
use crate::{
    error::AppError,
    state::AppState,
    users::{model::User, UserRepository},
};

/// Get all users
#[utoipa::path(
    get,
    path = "/users",
    tag = "User",
    responses(
        (status = 200, description = "List all users", body = Vec<User>)
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = UserRepository::find_all(&state.db).await?;
    Ok(Json(users))
}
