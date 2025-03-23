#![deny(clippy::unwrap_used, clippy::expect_used)]

use thiserror::Error;

pub mod gen_passwords;
pub mod gen_hashes;
pub mod dump_hashes;

#[derive(Debug, Error)]
pub enum GenPassError {
    #[error("Basic eror: {0}")]
    Basic(String),

    #[error("Thread join failed: {0}")]
    ThreadJoin(String),

    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("Failed to lock mutex: {0}")]
    MutexLock(String),

    #[error("Parse error: {source}")]
    Parse {
        #[from]
        source: std::num::ParseIntError,
    },
}

#[derive(Debug, Error)]
pub enum DumpHashError {
    #[error("Basic eror: {0}")]
    Basic(String),

    #[error("Thread join failed: {0}")]
    ThreadJoin(String),

    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("Failed to lock mutex: {0}")]
    MutexLock(String),

    #[error("Parse error: {source}")]
    Parse {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("Issue converting u8 slice to string: {source}")]
    FromUtf8 {
        #[from]
        source: std::string::FromUtf8Error,
    },
}

