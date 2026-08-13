// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2026 Alexander Galay <alexander.galay@proton.me>

use crate::state::SharedState;
use crate::error::{JsonResult, FileStreamResult};
use ephemeria_core::dto::api::*;
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
        mod_loader: season.mod_loader.clone(),
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
