mod config;

use env_logger::Builder;
use log::{debug, info, warn, error};
use axum::{routing::get, Router};
use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::from_file("./config.toml").expect("Could not read configuration file");
    println!("{:?}", config);

    let mut builder = Builder::new();
    builder.filter(None, config.server.log_level.unwrap().into());
    builder.init();

    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    let addr = format!("0.0.0.0:{}", config.server.port);
    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
    info!("Starting server on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
