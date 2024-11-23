use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, time::SystemTime};

pub trait DeepEq {
    fn deep_eq(&self, other: &Self) -> bool;
    fn deep_ne(&self, other: &Self) -> bool;
}

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
    /// This implementation **ignores** the `id` and `timestamp`.
    ///
    /// It only checks for the content of files
    fn eq(&self, other: &Self) -> bool {
        if self.files.len() != other.files.len() {
            return false;
        }
        for (path, self_metadata) in &self.files {
            match other.files.get(path) {
                Some(other_metadata) => {
                    if self_metadata != other_metadata {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }
    fn ne(&self, other: &Self) -> bool {
        !Self::eq(&self, other)
    }
}

impl DeepEq for Snapshot {
    /// This function compares `timestamp` and `id` of snapshot
    fn deep_eq(&self, other: &Snapshot) -> bool {
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
    fn deep_ne(&self, other: &Self) -> bool {
        !Self::deep_eq(&self, other)
    }
}
