use serde::{Serialize, Deserialize};

/**
   A socket message
*/
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SocketMessage {
  Request(Request),
  Response(Response),
}

/**
    A request via websockets
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
  /**
      When requesting via websocket we need an id to refer to when sending a response
  */
  pub id: String,
  /**
      The actual request
  */
  pub request: super::Request,
}

/*
    A response to a websocket request
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct Response {
  /**
      The id of the request this response refers to
  */
  pub id: String,
  /**
      A result containing the response or an error string if there has been an internal error while processing the request.
      The error does not indicate a user defined error (e.g. wrongPassword) but a internal error (e.g. could not parse body).
      We need this error type because when requesting via sockets there is no way of indicating an error via the http status code.
      The user should not be able to set the error value, this is reserved to indicate an actual internal error.
   */
  pub body: Result<super::Response, String>,
}
