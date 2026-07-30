use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

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
