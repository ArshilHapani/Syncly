use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, time::SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetaData {
    pub path: PathBuf,
    pub size: u64,
    pub modified_time: SystemTime,
    pub hash: String,
}

impl PartialEq for FileMetaData {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
            && self.size == other.size
            && self.modified_time == other.modified_time
            && self.hash == other.hash
    }
    fn ne(&self, other: &Self) -> bool {
        !Self::eq(&self, other)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: SystemTime,
    pub files: HashMap<PathBuf, FileMetaData>,
}

impl PartialEq for Snapshot {
    fn eq(&self, other: &Self) -> bool {
        let id_flag = self.id == other.id;
        let time_stamp_flag = self.timestamp == other.timestamp;
        let mut files_flag = true;
        for entry in self.files.iter() {
            if other.files.get(entry.0).is_none() {
                files_flag = false;
                break;
            } else if let Some(other_item) = other.files.get(entry.0) {
                let self_item = entry.1;
                files_flag = self_item == other_item;
            }
        }

        id_flag && time_stamp_flag && files_flag
    }
    fn ne(&self, other: &Self) -> bool {
        !Self::eq(&self, other)
    }
}
