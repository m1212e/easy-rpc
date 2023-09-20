use std::{collections::HashMap, sync::Arc};

//TODO think of some clever error handling
//TODO this could use some optimizations to improve performance
//TODO ideally we recycle already existing ws connections to a backend when two frontend server are connecting to the
// same machine

use erpc::protocol::{self};
use log::error;
use parking_lot::RwLock;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{console, ErrorEvent, MessageEvent, WebSocket};

use crate::CREATED_TARGETS;

type InternalHandler = Box<dyn Fn(protocol::Request) -> protocol::Response>;

pub struct Server {
    role: String,
    handlers: Arc<RwLock<HashMap<String, InternalHandler>>>,
}

impl Server {
    pub fn new(role: String) -> Self {
        Self {
            role,
            handlers: Arc::new(RwLock::new(HashMap::new())), // TODO what does this warning mean?
        }
    }

    pub fn register_raw_handler(&mut self, handler: InternalHandler, identifier: String) {
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
                let address = format!("{address}/{}/{role}", protocol::routes::WEBSOCKETS_ROUTE);

                let ws = match WebSocket::new(&address) {
                    Ok(v) => v,
                    Err(err) => {
                        console::error_2(&JsValue::from_str("Could not create WebSocket: "), &err);
                        return;
                    }
                };
                ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
                let cloned_ws = ws.clone();
                let handlers = handlers.clone();
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    let handlers = handlers.clone();
                    let cloned_ws = cloned_ws.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let message =
                            match protocol::socket::SocketMessage::try_from_wasm_socket_message_event(e)
                                .await
                            {
                                Ok(v) => v,
                                Err(err) => {
                                    error!("Could not convert socket message: {}", err);
                                    return;
                                }
                            };

                        let response: protocol::socket::SocketMessage = match message {
                            protocol::socket::SocketMessage::Request(req) => {
                                let handlers = handlers.read();
                                match handlers.get(&req.request.identifier) {
                                    Some(handler) => {
                                        let response = handler(req.request);
                                        protocol::socket::SocketMessage::Response(
                                            protocol::socket::Response {
                                                id: req.id,
                                                response,
                                            },
                                        )
                                    }
                                    None => {
                                        error!(
                                            "Could not find handler for route {}",
                                            req.request.identifier
                                        );
                                        protocol::socket::SocketMessage::Response(
                                            protocol::socket::Response {
                                                id: req.id,
                                                response: protocol::error::SendableError::NotFound
                                                    .into(),
                                            },
                                        )
                                    }
                                }
                            }
                            protocol::socket::SocketMessage::Response(_) => todo!(), //TODO
                        };

                        println!("Sending response: {:?}", response);
                        //TODO remove unwrap
                        let serialized: Vec<u8> = response.try_into().unwrap();

                        println!("serialized: {:?}", serialized);

                        cloned_ws.send_with_u8_array(&serialized).unwrap();
                    });
                });
                // set message event handler on WebSocket
                ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                // forget the callback to keep it alive
                onmessage_callback.forget();

                let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
                    console::error_2(&JsValue::from_str("websocket error event: "), &e.error());
                });
                ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
                onerror_callback.forget();
            }
        });
    }

    pub fn stop(&self) -> Result<(), String> {
        //TODO
        Err("Stopping not supported yet".to_string())
    }
}
