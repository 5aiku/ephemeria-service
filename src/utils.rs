use crate::config::SeasonManifest;
use tracing::{debug, info, warn, error};
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub fn load_season(season_path: &PathBuf) -> (SeasonManifest, String) {
    let manifest_path = season_path.join("season.toml");
    let manifest_content = fs::read_to_string(&manifest_path)
        .unwrap_or_else(|_| panic!("Failed to read season manifest at {:?}", manifest_path));

    let season_manifest: SeasonManifest = toml::from_str(&manifest_content)
        .expect("Failed to parse season.toml");
    info!("Season manifest loaded");
    debug!("{:?}", season_manifest);

    let mods_zip_path = season_path.join("mods.zip");

    let mods_hash = calculate_file_hash(mods_zip_path)
        .expect("Failed to calculate mods.zip hash");
    info!("Mods hash calculated");
    debug!("Mods hash: {:?}", mods_hash);

    (season_manifest, mods_hash)
}

pub fn calculate_file_hash(path: impl AsRef<Path>) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();

    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;

        if bytes_read == 0 {
            break
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();

    // 7 bytes for 'sha256:' + 64 bytes for actual hash = 71 bytes
    let mut result = String::with_capacity(71);
    result.push_str("sha256:");

    for byte in hash {
        write!(&mut result, "{:02x}", byte).unwrap();
    }

    Ok(result)
}
