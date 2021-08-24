use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_repr::*;

use crate::{Error, Result};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy, Debug)]
#[repr(u16)]
pub enum ErrorCode {
    BadRequest = 101,
    KeyDuplicated = 102,
    NotAuthorized = 103,
    RecordNotFound = 104,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Error {
        ok: bool,
        error_code: ErrorCode,
        error_text: String,
        reason: Vec<String>,
    },
    Success {
        ok: bool,
        result: T,
    },
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct URLRecord {
    pub key: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<u64>,
}

/* Requests */

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetRequest {
    pub key: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PostRequest {
    pub key: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PutRequest {
    pub key: String,
    pub value: String,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DelRequest {
    pub key: String,
    pub token: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VerifySingle {
    pub key: String,
    pub token: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VerifyRequest {
    pub values: Vec<VerifySingle>,
}

/* Responses */

pub type GetResponse = URLRecord;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PostResponse {
    pub token: String,
    pub key: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<u64>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UrlStored {
    pub key: String,
    pub value: String,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<u64>,
}

pub type PutResponse = PostResponse;

pub type DelResponse = URLRecord;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ListResponse {
    pub keys: Vec<String>,
    pub list_complete: bool,
    pub cursor: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VerifyResponse {
    pub matched: Vec<String>,
    pub unmatched: Vec<String>,
    pub missing: Vec<String>,
}

pub trait Req: Serialize {
    type Response: DeserializeOwned;
    const METHOD_NAME: &'static str;

    fn to_string(&self) -> Result<String> {
        serde_json::to_string(&self).map_err(Error::Serialize)
    }
}

impl Req for GetRequest {
    type Response = GetResponse;
    const METHOD_NAME: &'static str = "get_record";
}

impl Req for PostRequest {
    type Response = PostResponse;
    const METHOD_NAME: &'static str = "new_record";
}

impl Req for PutRequest {
    type Response = PutResponse;
    const METHOD_NAME: &'static str = "update_record";
}

impl Req for DelRequest {
    type Response = DelResponse;
    const METHOD_NAME: &'static str = "delete_record";
}

impl Req for ListRequest {
    type Response = ListResponse;
    const METHOD_NAME: &'static str = "list_record";
}

impl Req for VerifyRequest {
    type Response = VerifyResponse;
    const METHOD_NAME: &'static str = "verify_record";
}
