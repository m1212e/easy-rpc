use std::{collections::HashMap, sync::Arc};

use erpc::{
    protocol::{self, SendableError},
    target::TargetType,
};
use futures::channel::oneshot;
use log::error;
use parking_lot::Mutex;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, console};

use crate::{Socket, CREATED_TARGETS};

#[derive(Debug, Clone)]
pub struct Target {
    address: String,
    target_type: TargetType,
    socket: Option<Socket>,
    //TODO check if this is optimal
    open_socket_requests: Arc<Mutex<HashMap<String, oneshot::Sender<protocol::Response>>>>,
}

//TODO remove unwraps
impl Target {
    pub fn new(mut address: String, target_type: TargetType) -> Self {
        if address.ends_with('/') {
            address.pop();
        }

        let t = Target {
            address,
            target_type,
            socket: None,
            open_socket_requests: Arc::new(Mutex::new(HashMap::new())),
        };
        CREATED_TARGETS.send(t.clone()).unwrap();
        t
    }

    pub async fn call(&self, request: protocol::Request) -> protocol::Response {
        match self.target_type {
            TargetType::HttpServer => match &self.socket {
                Some(socket) => {
                    let request = protocol::socket::Request {
                        id: nanoid::nanoid!(),
                        request,
                    };

                    let (sender, reciever) = oneshot::channel();
                    self.open_socket_requests
                        .lock()
                        .insert(request.id.clone(), sender);

                    socket.requests.send(request).unwrap();

                    match reciever.await {
                        Ok(v) => v,
                        Err(err) => {
                            SendableError::from(format!("Recieving response cancelled: {}", err)).into()
                        }
                    }
                }
                None => {
                    let mut opts = RequestInit::new();
                    opts.method("POST");
                    opts.mode(RequestMode::Cors);

                    let stringified = match serde_json::to_string(&request.parameters) {
                        Ok(v) => v,
                        Err(err) => return SendableError::from(err).into(),
                    };

                    opts.body(Some(&JsValue::from_str(stringified.as_str())));

                    let url = format!(
                        "{}/{}/{}",
                        self.address,
                        protocol::routes::HANDLERS_ROUTE,
                        request.identifier
                    );
                    let request = match Request::new_with_str_and_init(&url, &opts) {
                        Ok(v) => v,
                        Err(err) => return SendableError::from(err).into(),
                    };

                    match request.headers().set("Content-Type", "application/json") {
                        Ok(_) => {},
                        Err(err) => return SendableError::from(format!("Could not add header to request: {:#?}", err)).into(),
                    };

                    let window = match web_sys::window() {
                        Some(v) => v,
                        None => return SendableError::from("Could not access window object").into(),
                    };
                    let resp_value = match JsFuture::from(window.fetch_with_request(&request)).await
                    {
                        Ok(v) => v,
                        Err(err) => return SendableError::from(err).into(),
                    };

                    let resp: Response = match resp_value.dyn_into() {
                        Ok(v) => v,
                        Err(err) => return SendableError::from(err).into(),
                    };

                    let body = match JsFuture::from(match resp.array_buffer() {
                        Ok(v) => v,
                        Err(err) => return SendableError::from(err).into(),
                    })
                    .await
                    {
                        Ok(v) => v,
                        Err(err) => return SendableError::from(err).into(),
                    };
                    let body: serde_json::Value =
                        match serde_json::from_slice(&js_sys::Uint8Array::new(&body).to_vec()) {
                            Ok(v) => v,
                            Err(err) => return SendableError::from(err).into(),
                        };

                    protocol::Response { body: Ok(body) }
                }
            },
            TargetType::Browser => {
                panic!("Browser to browser is not supported yet")
            }
        }
    }

    pub fn set_socket(&mut self, socket: Socket) {
        let responses = socket.responses.clone();
        self.socket = Some(socket);
        let open_requests = self.open_socket_requests.clone();
        wasm_bindgen_futures::spawn_local(async move {
            while let Ok(response) = responses.recv_async().await {
                let mut open_requests = open_requests.lock();
                let responder = match open_requests.remove(&response.id) {
                    Some(v) => v,
                    None => {
                        error!(
                            "Could not find open request for response with id {}",
                            response.id
                        );
                        continue;
                    }
                };

                match responder.send(response.response) {
                    Ok(_) => {}
                    Err(_) => {
                        error!("Could not send response on oneshot");
                    }
                };
            }
        });
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}
