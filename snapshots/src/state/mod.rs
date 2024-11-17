use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileMetaData {
    pub path: PathBuf,
    pub size: u64,
    pub modified_time: std::time::SystemTime,
    pub hash: String,
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: std::time::SystemTime,
    pub files: HashMap<PathBuf, FileMetaData>,
}
