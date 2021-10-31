use std::io;

use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AniseError {
    #[error("Io error: {source}")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("Invalid regular expression: {source}")]
    InvalidRegex {
        #[from]
        source: regex::Error,
    },
    #[error("Request failed: {source}")]
    Request {
        #[from]
        source: ureq::Error,
    },
    #[error("Unknown error: {source}")]
    Unknown {
        #[from]
        source: Box<dyn std::error::Error>,
    },
}

pub type AniseResult<T> = Result<T, AniseError>;
