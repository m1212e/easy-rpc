pub mod socket;

use serde::{Deserialize, Serialize};

/**
   In incoming erpc request.
   When no parameters are sent, the vec is empty
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
  pub identifier: String,
  pub parameters: Vec<serde_json::Value>,
}

/**
   An outgoing erpc response
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  pub body: serde_json::Value,
}
