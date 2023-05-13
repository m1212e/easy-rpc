use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite;

use super::error::Error;

/**
   A socket message
*/
#[derive(Debug, Serialize, Deserialize)]
pub enum SocketMessage {
    Request(Request),
    Response(Response),
}

impl TryFrom<String> for SocketMessage {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(&value)?)
    }
}

impl TryFrom<Vec<u8>> for SocketMessage {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(&value)?)
    }
}

impl TryInto<tungstenite::Message> for SocketMessage {
    type Error = Error;

    fn try_into(self) -> Result<tungstenite::Message, Self::Error> {
        match self {
            SocketMessage::Request(req) => req.try_into(),
            SocketMessage::Response(res) => res.try_into(),
        }
    }
}

impl SocketMessage {
    pub fn id(&self) -> &str {
        match self {
            SocketMessage::Request(r) => &r.id,
            SocketMessage::Response(r) => &r.id,
        }
    }

    pub fn try_from_socket_message(value: tungstenite::Message) -> Result<Option<Self>, Error> {
        Ok(match value {
            tungstenite::Message::Text(text) => Some(text.try_into()?),
            tungstenite::Message::Binary(binary) => Some(binary.try_into()?),
            _ => None,
        })
    }
}

/**
    A request via websockets. Wraps around the basic request to add an ID to assign a response to this request
*/
#[derive(Debug, Deserialize, Serialize)]
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

impl TryInto<tungstenite::Message> for Request {
    type Error = Error;

    fn try_into(self) -> Result<tungstenite::Message, Self::Error> {
        Ok(tungstenite::Message::Binary(serde_json::to_vec(&self)?))
    }
}

/*
    A response to a websocket request. Wraps around the basic response to add an ID to assign this response to a request
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    /**
        The id of the request this response refers to
    */
    pub id: String,
    pub response: super::Response,
}

impl TryInto<tungstenite::Message> for Response {
    type Error = Error;

    fn try_into(self) -> Result<tungstenite::Message, Self::Error> {
        Ok(tungstenite::Message::Binary(serde_json::to_vec(&self)?))
    }
}

