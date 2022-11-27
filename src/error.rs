use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read input")]
    InputReadFailed(#[from] io::Error),
    #[error("invalid regex to parse input")]
    InvalidRegex(#[from] regex::Error),
    #[error("invalid input format")]
    InvalidInputFormat,
    #[error("invalid transform format")]
    InvalidTransformFormat,
    #[error("JSON serialization failed")]
    JsonSerializationFailed(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
