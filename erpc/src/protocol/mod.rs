pub mod error;
pub mod socket;

use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use serde::{Serialize, Deserialize};

use self::error::{Error, SendableError};

/**
   The most basic kind of request. Used to pass around request info internally, e.g. to pass into the handlers
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub identifier: String,
    pub parameters: Vec<serde_json::Value>,
}

impl Request {
    pub async fn try_from_hyper_request(
        request: hyper::Request<hyper::body::Incoming>,
    ) -> Result<Self, Error> {
        if !request.uri().path().starts_with("/handlers/") {
            return Err(Error::NotFound);
        }

        let identifier = request
            .uri()
            .path()
            .strip_prefix("/handlers/")
            .expect("The path unexpectedly does not start with /handlers/")
            .to_string();

        let body = request.into_body().collect().await?.to_bytes();

        Ok(Self {
            identifier,
            parameters: serde_json::from_slice(&body)?,
        })
    }
}

/**
    The most basic kind of response. Used to pass around request info internally, e.g. to return from handlers
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    /**
       An error in the result indicates an actual system error and not a user defined error.
       E.g. "connection closed" but not "wrong password"
    */
    pub body: Result<serde_json::Value, SendableError>,
}

impl TryInto<hyper::Response<Full<Bytes>>> for Response {
    type Error = Error;

    fn try_into(self) -> Result<hyper::Response<Full<Bytes>>, Self::Error> {
        let serialized: Bytes = serde_json::to_vec(&self.body?)?.into();
        let response = hyper::Response::builder().body(Full::new(serialized))?;

        Ok(response)
    }
}

impl From<Error> for Response {
    fn from(value: Error) -> Self {
        Response {
            body: Err(value.into()),
        }
    }
}