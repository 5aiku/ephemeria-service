mod config;

use config::{Config, LogFormat};

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use tracing::{debug, info, warn, error};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {
    let config = Config::from_file("./config.toml").expect("Could not read configuration file");

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

    info!("Successfully loaded config");
    debug!("{:?}", config);

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .layer(TraceLayer::new_for_http());

    let addr = format!("0.0.0.0:{}", config.server.port);
    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
