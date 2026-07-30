use std::sync::Arc;
use crate::config::Config;

pub struct ApiState {
    pub config: Config,
    pub instance_hash: String,
}

pub type SharedState = Arc<ApiState>;
