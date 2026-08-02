use crate::config::{Config, SeasonManifest};
use std::sync::Arc;
use std::path::PathBuf;

pub struct ServiceState {
    pub config: Config,
    pub season_path: PathBuf,
    pub season_manifest: SeasonManifest,
    pub mods_hash: String,
}

pub type SharedState = Arc<ServiceState>;
