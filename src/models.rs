use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Manifest {
    pub game_version: String,
    pub java_version: String,
    pub instance_hash: String,
    pub server_ip: String,
    pub server_port: u16,
}

#[derive(Debug, Serialize)]
pub struct ServerStatus {
    pub online: bool,
    pub players: u16,
    pub max_players: u16,
    pub motd: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct LauncherVersion {
    pub version: String,
    pub hash: String,
    pub release_notes: String,
}
