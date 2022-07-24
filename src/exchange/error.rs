use serde::{Deserialize, Serialize};
use std::result;
use thiserror::Error;

pub type Result<T, E = ExchangeError> = result::Result<T, E>;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum ExchangeErrorType {
    Unknown,
    RequestError,
    RateLimit,
    OrderNotFound,
    OrderCompleted,
    InsufficientFunds,
    InvalidOrder,
    Authentication,
    ParsingError,
    ServiceUnavailable,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Error)]
#[error("Type: {error_type:?} Message: {message} Code {code:?}")]
pub struct ExchangeError {
    pub error_type: ExchangeErrorType,
    pub message: String,
    pub code: Option<i32>,
}

impl ExchangeError {
    pub fn new(error_type: ExchangeErrorType, message: String, code: Option<i32>) -> Self {
        Self {
            error_type,
            message,
            code,
        }
    }

    pub fn request_error(message: String, code: i32) -> Self {
        ExchangeError::new(ExchangeErrorType::RequestError, message, Some(code))
    }

    pub fn parsing_error(message: String) -> Self {
        ExchangeError::new(ExchangeErrorType::ParsingError, message, None)
    }
    pub fn unknown_error(message: &str) -> Self {
        Self {
            error_type: ExchangeErrorType::Unknown,
            message: message.to_owned(),
            code: None,
        }
    }
}
