pub mod model;
pub mod repository;
pub mod handlers;

pub use repository::RosterRepository;

use axum::Router;
use crate::state::AppState;
use handlers::list::list_roster;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(list_roster))
}
