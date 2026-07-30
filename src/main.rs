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

    let instance_path = &config.minecraft.instance_path;

    info!("Calculating hash for instance file: {:?}", instance_path);
    let instance_hash = calculate_file_hash(instance_path)
        .expect("Failed to calculate instance hash. Does the file exist?");
    info!("Instance hash calculated: {}", instance_hash);

    let state = Arc::new(ApiState { config, instance_hash });
    let router = create_router(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("Starting server on {}", addr);

    axum::serve(listener, router).await.unwrap();
}
