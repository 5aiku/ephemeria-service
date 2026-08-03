use crate::state::SharedState;
use crate::error::{JsonResult, FileStreamResult, ServiceError};
use crate::dto::*;

use tokio::fs::File;
use tracing::{debug, info, warn, error};
use axum::{
    body::Body,
    http::{header, HeaderMap, StatusCode},
    extract::State,
    response::{Response, IntoResponse, Json},
};
use tokio_util::io::ReaderStream;
use serde_json::json;

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

#[tracing::instrument(name = "get_instance", skip(state))]
pub async fn instance(State(state): State<SharedState>) -> FileStreamResult {
    let mods_path = state.season_path.join("mods.zip");

    let file = match File::open(&mods_path).await {
        Ok(file) => file,
        Err(error) => {
            error!("Failed to open mods.zip at {:?}: {}", mods_path, error);
            return Err(ServiceError::FileNotFound("mods.zip".to_string()));
        }
    };

    let metadata = file.metadata().await?;
    let file_size = metadata.len();

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    info!("Streaming 'mods.zip' (size: {} bytes)", file_size);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/zip")
        .header(header::CONTENT_DISPOSITION, "attachment; filename=\"mods.zip\"")
        .header(header::CONTENT_LENGTH, file_size)
        .body(body)
        .expect("Never fails");

    Ok(response)
}
