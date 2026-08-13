// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2026 Alexander Galay <alexander.galay@proton.me>

mod config;
mod error;
mod state;
mod router;
mod api;
mod utils;

use std::sync::Arc;
use std::process::ExitCode;

use config::{Config, LogLevel, LogFormat};
use crate::router::*;
use crate::state::*;
use crate::utils::*;
use crate::error::{Result, ServiceError};


use tracing::{debug, info};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;
// use tracing_subscriber::fmt::{self, format::FmtSpan};

#[tokio::main]
async fn main() -> ExitCode {
    match bootstrap().await {
        Ok(_) => {
            info!("Server shut down gracefully");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Fatal error. Failed to start API server:");
            eprintln!(" -> {}", e);
            ExitCode::FAILURE
        }
    }
}

async fn bootstrap() -> Result<()> {
    println!("Starting Ephemeria Service API...");

    let config = Config::from_file("./config.toml")?;
    init_logger(&config.api.log_level, &config.api.log_format);
    let active_season = &config.minecraft.active_season;

    info!("Config loaded. Active season: '{}'", active_season);
    debug!("{:?}", config);

    let season_path = config.minecraft.sync_dir.join(active_season);
    if !season_path.exists() {
        return Err(
            ServiceError::SeasonNotFound(active_season.clone())
        );
    }

    let season_manifest = load_season(&season_path)?;

    let mods_zip_path = season_path.join("mods.zip");
    if !mods_zip_path.exists() {
        return Err(
            ServiceError::FileNotFound(mods_zip_path.to_string_lossy().to_string())
        );
    }

    let mods_hash = calculate_file_hash(&mods_zip_path)?;
    info!("Mods hash calculated");
    debug!("Mods hash: {}", mods_hash);

    let state: SharedState = Arc::new(ServiceState {
        config,
        season_path,
        season_manifest,
        mods_hash
    });

    let port = state.config.api.port;
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let router = create_router(state);

    info!("Starting API service on {}", addr);
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn init_logger(level: &LogLevel, format: &LogFormat) {
    let filter = level.as_str();

    match format {
        LogFormat::Plain => {
            let subscriber = fmt()
                .compact()
                .with_target(false)
                .with_span_events(FmtSpan::NONE)
                .with_env_filter(filter)
                .finish();
            tracing::subscriber::set_global_default(subscriber)
                .expect("Failed to set global default logger");
        }

        LogFormat::Json => {
            let subscriber = fmt()
                .json()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                .with_current_span(true)
                .with_span_list(true)
                .with_env_filter(filter)
                .finish();
            tracing::subscriber::set_global_default(subscriber)
                .expect("Failed to set global default logger");
        }
    }
}
