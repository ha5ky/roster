mod config;
mod db;

mod error;
mod roster;
mod state;
mod users;

use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Config;
use crate::db::init_db; // Changed from init_pool
use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        users::handlers::list::list_users,
        users::handlers::create::create_user,
        roster::handlers::list::list_roster,
    ),
    components(
        schemas(
            users::model::User,
            users::model::CreateUserRequest,
            roster::model::Roster,
            roster::model::RosterQuery,
        )
    ),
    tags(
        (name = "User", description = "User management endpoints")
    )
)]
pub struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "roster=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    // Load configuration
    let config = Config::new()?;

    // Initialize database
    info!("Connecting to database at {}", config.database.url);
    let db = init_db(&config.database.url).await?; // Changed

    // Create app state
    let state = AppState::new(db);

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application router
    let app = Router::new()
        .nest("/api/v1/users", users::routes())
        .nest("/api/v1/roster", roster::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .with_state(state);

    // Run the server
    // let addr = SocketAddr::from(([127, 0, 0, 1], config.server.port));
    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid address configuration: {}", e))?;
    info!("listening on {}", addr);
    info!("Swagger UI available at http://{}/swagger-ui/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
