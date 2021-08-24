use reqwest::StatusCode;
use thiserror::Error;

use crate::ErrorCode;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Bad key format, should be 3 - 12 chars long, using [a-zA-Z0-9_-]")]
    BadKey,

    #[error("API returns an error: [{0:?}] {1}")]
    ApiError(ErrorCode, String),

    #[error("Bad request")]
    BadRequest(reqwest::Error),

    #[error("Request failed")]
    RequestFailed(StatusCode, String),

    #[error("Bad URL format")]
    BadUrl(url::ParseError),

    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),

    #[error("Parsing error, illegal JSON")]
    Parse(serde_json::Error),

    #[error("Serialize error, illegal data: {0}")]
    Serialize(serde_json::Error),

    #[cfg(feature = "file")]
    #[error("Read json error")]
    ReadError(std::io::Error),

    #[cfg(feature = "file")]
    #[error("Write json error")]
    WriteError(std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
