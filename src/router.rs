// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2026 Alexander Galay <alexander.galay@proton.me>

use axum::{
    http::{Method, HeaderValue},
    routing::get,
    Router
};
use tower_http::{
    trace::TraceLayer,
    cors::{Any, CorsLayer, AllowOrigin},
};
use crate::state::SharedState;
use crate::api;
use crate::utils::*;

pub fn create_router(state: SharedState) -> Router {
    let origins: Vec<HeaderValue> = state.config.cors.allowed_origins
        .iter()
        .filter_map(|s| validate_origin(s))
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    Router::new()
        .route("/health", get(api::check_health))
        .route("/api/v1/server/manifest", get(api::server::manifest))
        .route("/api/v1/server/mods", get(api::server::mods))
        // .route("/api/v1/server/status", get(api::server::status))

        // .route("/api/v1/launcher/version", get(api::launcher::version))
        // .route("/api/v1/launcher/download", get(api::launcher::download))

        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}
