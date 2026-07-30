use crate::state::SharedState;

use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;



pub async fn manifest(State(state): State<SharedState>) -> impl IntoResponse {
    let server = &state.config.minecraft;

    Json(json!(
        {
            "game_version": server.game_version,
            "java_version": server.java_version,
            "instance_hash": state.instance_hash,
            "server_ip": "mc.ephemeria.fun",
            "server_port": 25565,
        }
    ))
}
