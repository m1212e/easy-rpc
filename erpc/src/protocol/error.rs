use http_body_util::Full;
use hyper::{body::Bytes, Response, StatusCode};
use log::error;

/**
   Server only errors which cannot be sent to the user and must be transferred to some handler error
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

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Custom(value)
    }
}

impl From<Error> for Response<Full<Bytes>> {
    fn from(val: Error) -> Self {
        error!("Request errored: {:#?}", val);
        match val {
            Error::NotFound => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new("Not found".to_string().into_bytes().into())),
            _ => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::new(
                    "Internal server error".to_string().into_bytes().into(),
                )),
        }
        .expect("Could not even send error response")
    }
}
