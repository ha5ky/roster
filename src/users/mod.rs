pub mod handlers;
pub mod model;
pub mod repository;

use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;
use handlers::{create::create_user, list::list_users};

pub use repository::UserRepository;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::list::list_users,
        handlers::create::create_user,
    ),
    components(
        schemas(
            model::User,
            model::CreateUserRequest,
        )
    ),
    tags(
        (name = "User", description = "User management endpoints")
    )
)]
pub struct UserOpenApi;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(list_users))
}
