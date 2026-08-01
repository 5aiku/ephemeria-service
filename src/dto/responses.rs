use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ManifestResponse {
    pub season_name: String,
    pub season_description: String,
    pub game_version: String,
    pub java_version: String,
    pub mods_hash: String,
    pub server_ip: String,
    pub server_port: u16,
}

#[derive(Debug, Serialize)]
pub struct ServerStatusResponse {
    pub online: bool,
    pub players: u16,
    pub max_players: u16,
    pub motd: String,
    pub game_version: String,
}

#[derive(Debug, Serialize)]
pub struct LauncherVersionResponse {
    pub latest_version: String,
    pub hash: String,
    pub release_notes: String,
}
