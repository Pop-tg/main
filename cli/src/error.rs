use std::fmt::Display;

use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    BadRequest(reqwest::Error),
    RequestFailed(StatusCode, String),
    BadUrl(url::ParseError),
    Reqwest(#[from] reqwest::Error),
    Parse(serde_json::Error),
    Serialize(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
