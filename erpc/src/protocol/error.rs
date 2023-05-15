use http_body_util::Full;
use hyper::body::Bytes;
use log::error;
use serde::{Deserialize, Serialize};

use super::Response;

/**
   Server only errors which cannot be sent to the user and must be transferred to some sendable error
*/
#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    HTTP(hyper::http::Error),
    Serde(serde_json::Error),
    Reqwest(reqwest::Error),
    NotFound,
    Custom(String),
}

impl From<hyper::Error> for Error {
    fn from(value: hyper::Error) -> Self {
        Error::Hyper(value)
    }
}

impl From<hyper::http::Error> for Error {
    fn from(value: hyper::http::Error) -> Self {
        Error::HTTP(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Serde(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Reqwest(value)
    }
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(value: serde_wasm_bindgen::Error) -> Self {
        Error::Custom(value.to_string())
    }
}

impl From<wasm_bindgen::JsValue> for Error {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        // since the to string is not implemented on JsValue we just use the one provided by the serde crate
        Error::Custom(serde_wasm_bindgen::Error::from(value).to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Custom(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Custom(value.to_string())
    }
}

impl From<Error> for hyper::Response<Full<Bytes>> {
    fn from(val: Error) -> Self {
        let sendable: SendableError = val.into();
        sendable.into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SendableError {
    NotFound,
    Internal,
}

impl From<SendableError> for Error {
    fn from(value: SendableError) -> Self {
        match value {
            SendableError::NotFound => Error::NotFound,
            SendableError::Internal => Error::Custom("Internal error".to_string()),
        }
    }
}

impl From<Error> for SendableError {
    fn from(value: Error) -> Self {
        // At this point data contained in the error would be lost, so we log it to the error channel
        error!("{:#?}", value);
        match value {
            Error::NotFound => Self::NotFound,
            _ => Self::Internal,
        }
    }
}

impl From<SendableError> for hyper::Response<Full<Bytes>> {
    fn from(val: SendableError) -> Self {
        Response { body: Err(val) }
            .try_into()
            .expect("Error conversion should not fail")
    }
}
