//! Error types for markdown-rag

use std::io;
use thiserror::Error;

/// Result type for markdown-rag operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during document loading and splitting
#[derive(Error, Debug)]
pub enum Error {
    /// I/O errors during file reading
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Document has no content
    #[error("Document is empty: {path}")]
    EmptyDocument { path: String },

    /// Invalid path provided
    #[error("Invalid path: {path}")]
    InvalidPath { path: String },

    /// Chunking configuration error
    #[error("Invalid chunk configuration: {reason}")]
    InvalidConfig { reason: String },

    /// UTF-8 decoding error
    #[error("UTF-8 decode error in {path}: {reason}")]
    InvalidUtf8 { path: String, reason: String },

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}
