pub mod server;
pub mod launcher;

pub use server::*;
pub use launcher::*;

use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

use crate::state::SharedState;

pub async fn check_health(State(state): State<SharedState>) -> impl IntoResponse {
    Json(json!({ "status": "UP" }))
}
