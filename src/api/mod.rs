// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2026 Alexander Galay <alexander.galay@proton.me>

pub mod server;
pub mod launcher;

use ephemeria_core::dto::api::*;
use crate::error::{JsonResult, FileStreamResult, ServiceError};
use std::path::Path;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tracing::{info, error};
use axum::{
    response::{Response, Json},
    http::{header, StatusCode},
    body::Body,
};


#[tracing::instrument(name = "check_health")]
pub async fn check_health() -> JsonResult<HealthStatusResponse> {
    Ok(Json(HealthStatusResponse {
        status: Status::Up,
    }))
}

pub async fn stream_file(file_path: &Path) -> FileStreamResult {
    let download_name = file_path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .ok_or_else(|| {
            error!("Invalid file path provided for streaming: {:?}", file_path);
            ServiceError::FileNotFound(file_path.display().to_string())
        })?;

    let file = match File::open(file_path).await {
        Ok(file) => file,
        Err(error) => {
            error!("Failed to open file at {:?}: {}", download_name, error);
            return Err(ServiceError::FileNotFound(download_name.to_string()));
        }
    };

    let metadata = file.metadata().await?;
    let file_size = metadata.len();

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    info!("Streaming '{}' (size: {} bytes)", download_name, file_size);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/zip")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", download_name)
        )
        .header(header::CONTENT_LENGTH, file_size)
        .body(body)
        .expect("Never fails");

    Ok(response)
}
