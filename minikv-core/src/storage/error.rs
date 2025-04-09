use std::fmt;

#[derive(Debug)]

pub enum KvError {
    KeyNotFound,
    StorageError(String),
    Internal(String),
}

impl fmt::Display for KvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KvError::KeyNotFound => write!(f, "Key not found"),
            KvError::Internal(msg) => write!(f, "Internal error: {}", msg),
            KvError::StorageError(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

impl std::error::Error for KvError {}
