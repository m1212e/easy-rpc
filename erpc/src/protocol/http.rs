use serde::{Deserialize, Serialize};

/**
   An incoming erpc request.
   When no parameters are sent, the vec is empty.
   This is what the http request body contains. Metadata like the method identifier is sent via other
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub parameters: Vec<serde_json::Value>,
}

/**
   An outgoing erpc response
*/
pub type Response = serde_json::Value;
