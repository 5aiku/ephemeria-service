use crate::state::SharedState;
use crate::dto::*;

use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

pub async fn manifest(State(state): State<SharedState>) -> Json<ManifestResponse> {
    let season = &state.season_manifest;

    Json(ManifestResponse {
        season_name: season.name.clone(),
        season_description: season.description.clone(),
        game_version: season.game_version.clone(),
        java_version: season.java_version.clone(),
        mods_hash: state.mods_hash.clone(),
        server_ip: season.server_ip.clone(),
        server_port: season.server_port.clone(),
    })
}
