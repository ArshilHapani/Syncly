pub mod state;
pub mod storage;
pub mod utils;
pub mod watcher;

use std::{
    collections::HashMap,
    error::Error,
    fs::{metadata, read_dir, File},
    io::Read,
    path::PathBuf,
    time::SystemTime,
};

use state::{FileMetaData, Snapshot};

pub fn hash_file(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let file_result = File::open(path);
    match file_result {
        Ok(mut file) => {
            let mut buffer = vec![0; 1024];
            let mut hasher = blake3::Hasher::new();
            loop {
                let bytes = file.read(&mut buffer).unwrap_or(0);
                if bytes == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes]);
            }
            Ok(hasher.finalize().to_hex().to_string())
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn capture_snapshot(dir: &PathBuf) -> Snapshot {
    let mut files: HashMap<PathBuf, FileMetaData> = HashMap::new();
    let timestamp: SystemTime = std::time::SystemTime::now();
    // TODO handle the file case
    for entry in read_dir(dir).expect("Failed to read dir") {
        let entry = entry.expect("Failed to read directory");
        let path = entry.path();

        if path.is_file() {
            let meta = metadata(&path).expect("Failed to get metadata");
            let hash = hash_file(&path).expect("Failed to hash file");

            files.insert(
                path.clone(),
                FileMetaData {
                    path,
                    size: meta.len(),
                    modified_time: meta.modified().expect("Failed to get modified time"),
                    hash,
                },
            );
        }
    }

    Snapshot {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp,
        files,
    }
}
