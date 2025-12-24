use crate::roster::routes as roster_routes;
use crate::roster::RosterOpenApi;
use crate::state::AppState;
use crate::users::routes as users_routes;
use crate::users::UserOpenApi;
use axum::Router;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::{info, Level};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi()]
pub struct ApiDoc;

const ROSTER_API_V1: &str = "/api/v1";

pub fn roster_v1_routes() -> Router<AppState> {
    Router::new()
        .nest(ROSTER_API_V1, roster_routes())
        .nest(ROSTER_API_V1, users_routes())
}

pub fn api_registry() -> Router<AppState> {
    let mut api_docs = ApiDoc::openapi();
    api_docs.merge(RosterOpenApi::openapi());
    api_docs.merge(UserOpenApi::openapi());

    // Print registered routes from OpenAPI definition
    info!("🚀 Registered API Routes:");
    let _ = &api_docs.paths.paths.iter().for_each(|(path, item)| {
        item.operations.iter().for_each(|(method, _)| {
            let method_str = match method {
                utoipa::openapi::PathItemType::Get => "GET",
                utoipa::openapi::PathItemType::Post => "POST",
                utoipa::openapi::PathItemType::Put => "PUT",
                utoipa::openapi::PathItemType::Delete => "DELETE",
                utoipa::openapi::PathItemType::Options => "OPTIONS",
                utoipa::openapi::PathItemType::Head => "HEAD",
                utoipa::openapi::PathItemType::Patch => "PATCH",
                utoipa::openapi::PathItemType::Trace => "TRACE",
                utoipa::openapi::PathItemType::Connect => "CONNECT",
            };
            info!("   🚀 {:<6} {}", method_str, path);
        });
    });
    roster_v1_routes()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_docs))
}

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

pub fn latency() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
}
