use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use crate::state::SharedState;
use crate::api;

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/health", get(api::check_health))
        .route("/api/v1/server/manifest", get(api::server::manifest))
        // .route("/api/v1/server/instance", get(api::server::instance))
        // .route("/api/v1/server/status", get(api::server::status))

        // .route("/api/v1/launcher/version", get(api::launcher::version))
        // .route("/api/v1/launcher/download", get(api::launcher::download))

        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
