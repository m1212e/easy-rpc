use std::fmt::Display;

use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

//TODO: add more error types, to give the user more information about what went wrong
/// Error type that can be sent over the wire, does not contain any sensitive information
#[derive(Debug, Serialize, Deserialize, Error)]
pub enum SendableError {
    NotFound,
    Internal,
}

impl Display for SendableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl From<serde_json::Error> for SendableError {
    fn from(value: serde_json::Error) -> Self {
        error!("{}", value);
        Self::Internal
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<reqwest::Error> for SendableError {
    fn from(value: reqwest::Error) -> Self {
        error!("{}", value);
        Self::Internal
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<salvo::http::errors::StatusError> for SendableError {
    fn from(value: salvo::http::errors::StatusError) -> Self {
        error!("{}", value);
        Self::Internal
    }
}

impl From<serde_wasm_bindgen::Error> for SendableError {
    fn from(value: serde_wasm_bindgen::Error) -> Self {
        error!("{}", value);
        Self::Internal
    }
}

impl From<wasm_bindgen::JsValue> for SendableError {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        //TODO: check for correct error logging
        error!("{:#?}", value);
        Self::Internal
    }
}

impl From<gloo_file::FileReadError> for SendableError {
    fn from(value: gloo_file::FileReadError) -> Self {
        error!("{}", value);
        Self::Internal
    }
}

impl From<String> for SendableError {
    fn from(value: String) -> Self {
        error!("{}", value);
        Self::Internal
    }
}

impl From<&str> for SendableError {
    fn from(value: &str) -> Self {
        error!("{}", value);
        Self::Internal
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl salvo::Piece for SendableError {
    fn render(self, res: &mut salvo::Response) {
        match self {
            Self::NotFound => {
                res.status_code(salvo::http::StatusCode::NOT_FOUND);
            }
            Self::Internal => {
                res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
}
