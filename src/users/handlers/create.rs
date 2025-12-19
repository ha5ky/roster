use axum::{extract::State, Json};
use crate::{
    error::AppError,
    state::AppState,
    users::{model::{CreateUserRequest, User}, UserRepository},
};

/// Create a new user
#[utoipa::path(
    post,
    path = "/users",
    tag = "User",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = User)
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {
    let user = UserRepository::create(&state.db, payload).await?;
    Ok(Json(user))
}
