pub mod handlers;
pub mod model;
pub mod repository;

use axum::Router;
use crate::state::AppState;
use handlers::{create::create_user, list::list_users};

pub use repository::UserRepository;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::post(create_user).get(list_users))
}
