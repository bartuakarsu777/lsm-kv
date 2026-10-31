use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("key not found")]
    NotFound,

    #[error("i/o error: {0}")]
    Io(#[from] io::Error),

    #[error("corrupt record: {0}")]
    Corrupt(String),

    #[error("invalid configuration: {0}")]
    Config(String),

    #[error("store closed")]
    Closed,
}

pub type Result<T, E = StoreError> = std::result::Result<T, E>;