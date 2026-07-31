use crate::state::SharedState;

use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;



pub async fn manifest(State(state): State<SharedState>) -> impl IntoResponse {
    let season = &state.season_manifest;

    Json(json!(
        {
            "season_name": season.name,
            "season_description": season.description,
            "game_version": season.game_version,
            "java_version": season.java_version,
            "mods_hash": state.mods_hash,
            "server_ip": season.server_ip,
            "server_port": season.server_port,
        }
    ))
}
