mod config;
mod state;
mod router;
mod api;
mod utils;

use std::sync::Arc;

use config::{Config, LogFormat};
use crate::router::*;
use crate::state::*;
use crate::utils::*;

use axum::{routing::get, Router};

use tracing::{debug, info, warn, error};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {
    let config = Config::from_file("./config.toml").expect("Error reading config");
    let port = config.server.port;
    let filter = config.server.log_level.as_str();

    match config.server.log_format {
        LogFormat::Plain => {
            let subscriber = fmt()
                .with_env_filter(filter)
                .finish();
            tracing::subscriber::set_global_default(subscriber).unwrap();
        }

        LogFormat::Json => {
            let subscriber = fmt()
                .json()
                .with_env_filter(filter)
                .finish();
            tracing::subscriber::set_global_default(subscriber).unwrap();
        }
    }

    info!("Config loaded");
    debug!("{:?}", config);

    let active_season = &config.minecraft.active_season;
    let season_path = config.minecraft.sync_dir.join(active_season);
    let (season_manifest, mods_hash) = load_season(&season_path);

    let state = Arc::new(AppState { config, season_path, season_manifest, mods_hash });
    let router = create_router(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("Starting server on {}", addr);

    axum::serve(listener, router).await.unwrap();
}
