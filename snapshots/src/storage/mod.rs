use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use flate2::{read::GzEncoder, Compression};

use crate::state::Snapshot;

pub fn save_snapshot(snapshot: &Snapshot, snapshot_dir: &PathBuf) {
    let file_path = snapshot_dir.join(format!("snapshot_{}.json", snapshot.id));
    let mut file = File::create(file_path).expect("Failed to create snapshot file");
    let serialized = serde_json::to_string(snapshot).expect("Failed to serialize snapshot");
    file.write_all(serialized.as_bytes())
        .expect("Failed to write snapshot");
}

pub fn save_file_with_deduplication(
    file_path: &Path,
    hash: &String,
    content_dir: &PathBuf,
    prev_hash: &HashMap<PathBuf, String>,
) -> bool {
    if let Some(prev_hash) = prev_hash.get(file_path) {
        if prev_hash == hash {
            return false;
        }
    }

    let content_path = content_dir.join(hash);
    if !content_path.exists() {
        fs::copy(file_path, &content_path).expect("Failed to copy file");
    }
    println!("File saved {:?}", file_path);
    true
}

pub fn compress_file(input: &Path, output: &Path) {
    let mut input_file = File::open(input).expect("Failed to open input file");
    let output_file = File::create(output).expect("Failed to create output file");

    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let mut buffer = Vec::new();

    input_file
        .read_to_end(&mut buffer)
        .expect("Failed to read file");
    encoder.write_all(&buffer).expect("Failed to compress file");
    encoder.flush().expect("Failed to flush");

    println!("File compressed {:?}", output);
}

pub fn compress_metadata(snapshot: &Snapshot, output: &Path) {
    let serialized = serde_json::to_vec(snapshot).expect("Failed to serialize metadata");
    let mut encoder = GzEncoder::new(File::create(output).unwrap(), Compression::default());
    encoder
        .write_all(&serialized)
        .expect("Failed to write metadata");
    encoder.flush().expect("Failed to flush");
    println!("Snapshot metadata compressed: {:?}", output);
}
