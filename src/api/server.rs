use crate::state::SharedState;
use crate::error::{JsonResult, FileStreamResult};
use crate::dto::*;
use crate::api::*;

use axum::{
    extract::State,
    response::Json,
};

#[tracing::instrument(name = "get_manifest", skip(state))]
pub async fn manifest(State(state): State<SharedState>) -> JsonResult<ManifestResponse> {
    let season = &state.season_manifest;

    Ok(Json(ManifestResponse {
        season_name: season.name.clone(),
        season_description: season.description.clone(),
        game_version: season.game_version.clone(),
        java_version: season.java_version.clone(),
        mods_hash: state.mods_hash.clone(),
        server_ip: season.server_ip.clone(),
        server_port: season.server_port.clone(),
    }))
}

#[tracing::instrument(name = "get_mods", skip(state))]
pub async fn mods(State(state): State<SharedState>) -> FileStreamResult {
    let mods_path = state.season_path.join("mods.zip");
    stream_file(&mods_path).await
}
