pub mod state;
pub mod storage;
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

#[cfg(test)]
mod hash_file_test {
    use super::*;

    /// Try changing the content of `Cargo.toml` file inside **snapshots** dir and rerun the test!
    #[test]
    fn test_hash_file() {
        let file = PathBuf::from("Cargo.toml"); // snapshots/Cargo.toml file
        let hash = hash_file(&file).unwrap_or("".to_string());
        println!("{}", hash);
        assert_eq!(
            hash,
            "abc318a7a48ea6169c95461f05ac28f133712f08af64fc2b970b3a1bacd8ff30"
        );
    }
}
