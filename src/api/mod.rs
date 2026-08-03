pub mod server;
pub mod launcher;

pub use server::*;
pub use launcher::*;

use crate::dto::*;
use crate::error::JsonResult;
use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

use crate::state::SharedState;

#[tracing::instrument(name = "check_health", skip(state))]
pub async fn check_health(State(state): State<SharedState>) -> JsonResult<HealthStatusResponse> {
    Ok(Json(HealthStatusResponse {
        status: Status::Up,
    }))
}
