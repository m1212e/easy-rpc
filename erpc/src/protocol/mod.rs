pub mod error;
pub mod routes;
pub mod socket;

pub use self::error::SendableError;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use salvo::writing::Json;

/**
   The most basic kind of request. Used to pass around request info internally, e.g. to pass into the handlers
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub identifier: String,
    pub parameters: Vec<serde_json::Value>,
}

impl Request {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn try_from_salvo_request(
        req: &mut salvo::Request,
        identifier: String,
    ) -> Result<Self, SendableError> {
        Ok(Request {
            identifier,
            parameters: req
                .parse_json()
                .await
                .map_err(|err| format!("Could not parse request: {}", err))?,
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

impl From<SendableError> for Response {
    fn from(value: SendableError) -> Self {
        Response { body: Err(value) }
    }
}

impl From<serde_json::Value> for Response {
    fn from(value: serde_json::Value) -> Self {
        Response { body: Ok(value) }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl salvo::Piece for Response {
    fn render(self, res: &mut salvo::Response) {
        match self.body {
            Ok(v) => res.render(Json(v)),
            Err(err) => err.render(res),
        }
    }
}
