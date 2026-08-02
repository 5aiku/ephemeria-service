use crate::config::SeasonManifest;
use crate::error::Result;
use tracing::{debug, info, warn, error};
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub fn load_season(season_path: &PathBuf) -> Result<SeasonManifest> {
    let manifest_path = season_path.join("season.toml");
    let manifest_content = fs::read_to_string(&manifest_path)?;

    let season_manifest: SeasonManifest = toml::from_str(&manifest_content)?;

    info!("Season manifest loaded");
    debug!("{:?}", season_manifest);

    Ok(season_manifest)
}

pub fn calculate_file_hash(path: impl AsRef<Path>) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;

        if bytes_read == 0 {
            break
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();

    // 7 bytes for 'sha256:' + 64 bytes for actual hash = 71 bytes
    let mut result = String::with_capacity(71);
    result.push_str("sha256:");

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
