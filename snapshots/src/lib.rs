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
    use std::{fs, io::Write};

    use super::*;

    /// Try changing the content of `Cargo.toml` file inside **snapshots** dir and rerun the test!
    #[test]
    fn test_hash_file() {
        let current_dir_str = env!("CARGO_MANIFEST_DIR");
        let file_name = format!("{}/{}", current_dir_str, "hello.txt");
        let file_name_1 = format!("{}/{}", current_dir_str, "hello_1.txt");

        let mut file = File::create(&file_name).unwrap();
        let mut file_1 = File::create(&file_name_1).unwrap();

        let content = b"Hello, World!";
        let content_1 = b"Hello, World"; // Removed the exclamation mark
        file.write_all(content).unwrap();
        file_1.write_all(content_1).unwrap();

        let path_buf = PathBuf::from(&file_name);
        let path_buf_1 = PathBuf::from(&file_name_1);

        let hash = hash_file(&path_buf).unwrap_or("".to_string());
        let hash_1 = hash_file(&path_buf_1).unwrap_or("".to_string());

        file_1.flush().unwrap();
        file.flush().unwrap();
        file_1.sync_all().unwrap();
        file.sync_all().unwrap();
        drop(file);
        drop(file_1);
        fs::remove_file(&file_name).unwrap();
        fs::remove_file(&file_name_1).unwrap();

        let hello_world_file_hash =
            "288a86a79f20a3d6dccdca7713beaed178798296bdfa7913fa2a62d9727bf8f8";
        assert_eq!(hash, hello_world_file_hash);
        assert_ne!(hash_1, hello_world_file_hash);
    }
}
