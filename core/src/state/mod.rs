use std::{
    error::Error,
    fmt::Display,
    io::{Error as IOError, ErrorKind as IOErrorKind},
};

#[derive(Debug)]
pub enum SynclyErrorKind {
    FileOrDirNotFound,
    OpenError(IOError),
    FileReadError(IOError),
    WriteError(IOError),
    SyncError(IOError),
    FlushError(IOError),
    PermissionDenied(IOError),
}

impl Display for SynclyErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileOrDirNotFound => write!(f, "Provided file or dir not found"),
            Self::OpenError(e) => write!(f, "Failed to open file {}", e),
            Self::FileReadError(e) => write!(f, "Failed to read file {}", e),
            Self::WriteError(e) => write!(f, "Failed to write content in file {}", e),
            Self::SyncError(e) => write!(f, "Failed to sync the file heads {}", e),
            Self::FlushError(e) => write!(f, "Failed to flush the file memory {}", e),
            Self::PermissionDenied(e) => write!(f, "Permission denied for accessing file {}", e),
        }
    }
}

impl Error for SynclyErrorKind {}

impl From<IOError> for SynclyErrorKind {
    fn from(err: IOError) -> Self {
        match err.kind() {
            IOErrorKind::NotFound => Self::FileOrDirNotFound,
            IOErrorKind::PermissionDenied => Self::PermissionDenied(err),
            IOErrorKind::WriteZero => Self::WriteError(err),
            _ => Self::FileReadError(err),
        }
    }
}
