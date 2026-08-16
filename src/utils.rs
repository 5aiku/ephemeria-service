// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2026 Alexander Galay <alexander.galay@proton.me>

use crate::config::SeasonManifest;
use crate::error::Result;
use tracing::{debug, info, warn};
use axum::http::{HeaderValue, Uri};
use sha1::{Sha1, Digest};
use std::fmt::Write;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn load_season(season_path: &PathBuf) -> Result<SeasonManifest> {
    let manifest_path = season_path.join("season.toml");
    let manifest_content = fs::read_to_string(&manifest_path)?;

    let season_manifest: SeasonManifest = toml::from_str(&manifest_content)?;

    info!("Season manifest loaded");
    debug!("{:?}", season_manifest);

    Ok(season_manifest)
}

pub fn calculate_dir_hash(path: impl AsRef<Path>) -> Result<Option<String>> {
    let dir_path = path.as_ref();

    if !dir_path.exists() {
        return Ok(None);
    }

    let mut files: Vec<_> = fs::read_dir(dir_path)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "jar"))
        .collect();

    if files.is_empty() {
        return Ok(None);
    }

    files.sort_by_key(|e| e.file_name());

    let mut combined_hashes = String::new();

    for entry in files {
        let file_hash = calculate_file_hash(entry.path())?;
        combined_hashes.push_str(&file_hash);
    }

    let mut final_hasher = Sha1::new();
    final_hasher.update(combined_hashes.as_bytes());
    let hash = final_hasher.finalize();

    let mut result = String::with_capacity(40);
    for byte in hash {
        write!(&mut result, "{:02x}", byte).expect("write! to String never fails, right?");
    }

    Ok(Some(result))
}

pub fn calculate_file_hash(path: impl AsRef<Path>) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha1::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;

        if bytes_read == 0 {
            break
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();

    let mut result = String::with_capacity(64);

    for byte in hash {
        write!(&mut result, "{:02x}", byte).expect("write! to String never fails, right?");
    }

    Ok(result)
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Recieved Ctrl + C event, shutting down the server..."),
        _ = terminate => info!("Recieved SIGTERM, shutting down the server..."),
    }
}

pub fn validate_origin(origin_str: &str) -> Option<HeaderValue> {
    let uri = origin_str.parse::<Uri>().ok()?;

    let is_valid = uri.scheme().is_some()
        && uri.authority().is_some()
        && (uri.path() == "" || uri.path() == "/");

    if is_valid {
        origin_str.parse::<HeaderValue>().ok()
    } else {
        warn!(
            "Invalid CORS origin format: '{}')",
            origin_str
        );
        None
    }
}
