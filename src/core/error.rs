
use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FilterError {
    #[error("IOError")]
    IoError(#[from] io::Error),
    #[error("JsonError")]
    JsonError(#[from] serde_json::error::Error),
    // #[error("Unkwon error")]
    // Unknown,
}
