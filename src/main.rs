mod config;
mod db;
mod response;

mod error;
mod roster;
mod state;
mod users;
mod route;

use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::db::init_db; // Changed from init_pool
use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "roster=debug,tower_http=debug,sqlx=info".into()),
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

    // Run the server
    // let addr = SocketAddr::from(([127, 0, 0, 1], config.server.port));
    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid address configuration: {}", e))?;

    // Build application router
    let app = route::api_registry()
        .layer(route::cors())
        .layer(route::latency())
        .with_state(state);
    
    info!("listening on {}", addr);
    info!("Swagger UI available at http://{}/swagger-ui/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
