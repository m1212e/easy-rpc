use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;

use super::SendableError;

/**
   A socket message
*/
#[derive(Debug, Serialize, Deserialize)]
pub enum SocketMessage {
    Request(Request),
    Response(Response),
}

#[cfg(not(target_arch = "wasm32"))]
impl TryFrom<salvo::websocket::Message> for SocketMessage {
    type Error = SendableError;

    fn try_from(value: salvo::websocket::Message) -> Result<Self, Self::Error> {
        serde_json::from_slice(&value.into_bytes()).map_err(|e| e.into())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl TryInto<salvo::websocket::Message> for SocketMessage {
    type Error = SendableError;

    fn try_into(self) -> Result<salvo::websocket::Message, Self::Error> {
        Ok(salvo::websocket::Message::binary(serde_json::to_vec(
            &self,
        )?))
    }
}

impl TryInto<Vec<u8>> for SocketMessage {
    type Error = SendableError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(serde_json::to_vec(&self)?)
    }
}

// #[cfg(target_arch = "wasm32")]
impl SocketMessage {
    pub async fn try_from_wasm_socket_message_event(
        value: web_sys::MessageEvent,
    ) -> Result<Self, SendableError> {
        if let Ok(abuf) = value.data().dyn_into::<js_sys::ArrayBuffer>() {
            let array = js_sys::Uint8Array::new(&abuf);
            Ok(serde_json::from_slice(array.to_vec().as_slice())?)
        } else if let Ok(blob) = value.data().dyn_into::<web_sys::Blob>() {
            let b = gloo_file::futures::read_as_bytes(&gloo_file::Blob::from(blob)).await?;
            Ok(serde_json::from_slice(b.as_slice())?)
        } else if let Ok(txt) = value.data().dyn_into::<js_sys::JsString>() {
            Ok(serde_json::from_str(
                &txt.as_string()
                    .ok_or("Could not convert ws string to rs string")?,
            )?)
        } else {
            Err(SendableError::from("Unknown message type"))
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

impl Request {
    pub fn from_request(request: super::Request, id: &str) -> Self {
        Self {
            id: id.to_string(),
            request,
        }
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

impl Response {
    pub fn from_response(response: super::Response, id: &str) -> Self {
        Self {
            id: id.to_string(),
            response,
        }
    }
}