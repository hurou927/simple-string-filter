use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FilterError {
    #[error("IOError")]
    Io(#[from] io::Error),
    #[error("fail to open file. path: {filename}")]
    IoFaileToOpenFile { filename: String },
    #[error("fail to create file. path: ${filename}. hint: --force: overwrite an exisitng file")]
    IoFaileToCreateFile { filename: String },
    #[error("json error({0})")]
    Json(#[from] serde_json::error::Error),
}
