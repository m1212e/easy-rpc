use std::{collections::HashMap, sync::Arc};

//TODO think of some clever error handling
//TODO this could use some optimizations to improve performance
//TODO ideally we recycle already existing ws connections to a backend when two frontend server are connecting to the
// same machine

use erpc::protocol;
use log::{error, info, warn};
use parking_lot::RwLock;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

use crate::CREATED_TARGETS;

type InternalHandler = Box<dyn Fn(protocol::Request) -> Result<protocol::Response, String>>;

pub struct Server {
    role: String,
    handlers: Arc<RwLock<HashMap<String, InternalHandler>>>,
}

impl Server {
    pub fn new(role: String) -> Self {
        Self {
            role,
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register_raw_handler(&mut self, handler: InternalHandler, identifier: String) {
        warn!("inserting handler for identifier {}", identifier);
        self.handlers.write().insert(identifier, handler);
    }

    pub fn run(&self) {
        let role = self.role.clone();
        let handlers = self.handlers.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let reciever = match CREATED_TARGETS.reciever() {
                Ok(v) => v,
                Err(err) => {
                    error!("{}", err);
                    return;
                }
            };

            loop {
                let target = match reciever.recv_async().await {
                    Ok(v) => v,
                    Err(err) => {
                        error!("Couldn't receive target: {}", err);
                        return;
                    }
                };

                let address = target
                    .address()
                    .replace("http://", "ws://")
                    .replace("https://", "wss://");
                let address = format!("{address}/ws/{}", role);

                let ws = match WebSocket::new(&address) {
                    Ok(v) => v,
                    Err(err) => {
                        error!("Could not create WebSocket: {:#?}", err);
                        return;
                    }
                };
                ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
                let cloned_ws = ws.clone();
                let handlers = handlers.clone();
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    let message: protocol::socket::SocketMessage =
                        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                            let array = js_sys::Uint8Array::new(&abuf);

                            match serde_json::from_slice(array.to_vec().as_slice()) {
                                Ok(v) => v,
                                Err(err) => {
                                    error!("Could not deserialize incoming message: {:#?}", err);
                                    return;
                                }
                            }
                        } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                            warn!("Recieved blob message, ignoring. Blob type not yet supported");
                            return;
                            //TODO
                            // let array_buffer = match JsFuture::from(blob.array_buffer()).await {
                            //     Ok(v) => v,
                            //     Err(err) => {
                            //         error!("Could not convert blob to array buffer: {:#?}", err);
                            //         return;
                            //     }
                            // };

                            // let array_buffer = match array_buffer.dyn_into::<js_sys::ArrayBuffer>() {
                            //     Ok(v) => v,
                            //     Err(err) => {
                            //         error!("Could not cast converted blob to array buffer: {:#?}", err);
                            //         return;
                            //     }
                            // };

                            // match serde_json::from_slice(
                            //     js_sys::Uint8Array::new(&array_buffer).to_vec().as_slice(),
                            // ) {
                            //     Ok(v) => v,
                            //     Err(err) => {
                            //         error!("Could not deserialize incoming message: {:#?}", err);
                            //         return;
                            //     }
                            // }
                        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                            let txt = match txt.as_string() {
                                Some(v) => v,
                                None => {
                                    error!("Could not convert string");
                                    return;
                                }
                            };
                            match serde_json::from_str(&txt) {
                                Ok(v) => v,
                                Err(err) => {
                                    error!("Could not deserialize incoming message: {:#?}", err);
                                    return;
                                }
                            }
                        } else {
                            error!("message event, received Unknown: {:?}", e.data());
                            return;
                        };

                    let id = message.id().to_string();
                    let response: Result<protocol::Response, String> = match message {
                        protocol::socket::SocketMessage::Request(req) => handlers
                            .read()
                            .get(&req.request.identifier)
                            .ok_or(format!(
                                "Could not find handler for request {}",
                                req.request.identifier
                            ))
                            .and_then(|handler| handler(req.request)),
                        protocol::socket::SocketMessage::Response(res) => {
                            error!("Recieved websocket message of response type. This is not supported yet");
                            Err("Recieved websocket message of response type. This is not supported yet".to_string())
                        }
                    };

                    //TODO remove unwrap
                    cloned_ws
                        .send_with_u8_array(
                            &serde_json::to_vec(&protocol::socket::Response { id, body: response.unwrap() })
                                .unwrap(),
                        )
                        .unwrap();
                });
                // set message event handler on WebSocket
                ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                // forget the callback to keep it alive
                onmessage_callback.forget();

                let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
                    error!("error event: {:?}", e);
                });
                ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
                onerror_callback.forget();

                let cloned_ws = ws.clone();
            }
        });
    }

    pub fn stop(&self) -> Result<(), String> {
        //TODO
        Err("Stopping not supported yet".to_string())
    }
}
