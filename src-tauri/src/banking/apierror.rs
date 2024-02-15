extern crate reqwest;

use std::{error::Error, fmt};

use reqwest::header::InvalidHeaderValue;
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    InvalidHeader(InvalidHeaderValue),
    Custom(String)
}

impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Reqwest(err) => write!(f, "Reqwest Error: {}", err),
            ApiError::Serde(err) => write!(f, "Serde Error: {}", err),
            ApiError::InvalidHeader(err) => write!(f, "Invalid Header Value: {}", err),
            ApiError::Custom(err) => write!(f, "Custom Error: {}", err),
        }
    }
}

impl Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> ApiError {
        ApiError::Reqwest(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> ApiError {
        ApiError::Serde(err)
    }
}

impl From<InvalidHeaderValue> for ApiError {
    fn from(err: InvalidHeaderValue) -> ApiError {
        ApiError::InvalidHeader(err)
    }
}

impl From<std::string::String> for ApiError {
    fn from(err: String) -> ApiError {
        ApiError::Custom(err)
    }
}