use reqwest::Error as ReqwestError;
use serde_json::error::Error as SerdeError;
use std::result;
use std::time::SystemTimeError;
use thiserror::Error;

pub type Result<T, E = ClientError> = result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Serde error: {0}")]
    SerdeErr(#[from] SerdeError),

    #[error("System time error: {0}")]
    SystemTimeErr(#[from] SystemTimeError),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] ReqwestError),
}
