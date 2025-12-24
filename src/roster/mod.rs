pub mod model;
pub mod repository;
pub mod handlers;

pub use repository::RosterRepository;

use axum::Router;
use axum::routing::get;
use crate::state::AppState;
use handlers::list::list_roster;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::list::list_roster,
    ),
    components(
        schemas(
            model::Roster,
            model::RosterQuery,
        )
    ),
    tags(
        (name = "Roster", description = "Roster management endpoints")
    )
)]
pub struct RosterOpenApi;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/roster", get(list_roster))
}
