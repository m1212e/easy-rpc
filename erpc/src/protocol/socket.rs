use serde::{Deserialize, Serialize};

/**
   A socket message
*/
pub enum SocketMessage {
    Request(Request),
    Response(Response),
}

/**
    A request via websockets. Wraps around the basic request to add an ID to assign a response to this request
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
    A response to a websocket request. Wraps around the basic response to add an ID to assign this response to a request
*/
#[derive(Debug)]
pub struct Response {
    /**
        The id of the request this response refers to
    */
    pub id: String,
    pub body: super::Response,
}

impl SocketMessage {
    pub fn id(&self) -> &str {
        match self {
            SocketMessage::Request(r) => &r.id,
            SocketMessage::Response(r) => &r.id,
        }
    }
}
