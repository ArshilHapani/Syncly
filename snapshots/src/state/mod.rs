use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, time::SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetaData {
    pub path: PathBuf,
    pub size: u64,
    pub modified_time: SystemTime,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: SystemTime,
    pub files: HashMap<PathBuf, FileMetaData>,
}
