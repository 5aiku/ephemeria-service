// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2026 Alexander Galay <alexander.galay@proton.me>

use crate::config::{Config, SeasonManifest};
use std::sync::Arc;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ServiceState {
    pub config: Config,
    pub season_path: PathBuf,
    pub season_manifest: SeasonManifest,
    pub mods_hash: String,
}

pub type SharedState = Arc<ServiceState>;
