use std::collections::HashMap;

//TODO think of some clever error handling
//TODO this could use some optimizations to improve performance
//TODO ideally we recycle already existing ws connections to a backend when two frontend server are connecting to the
// same machine

use erpc::protocol;
use log::{error, info, warn};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

use crate::CREATED_TARGETS;

type InternalHandler = Box<dyn Fn(protocol::Request) -> Result<protocol::Response, String>>;

pub struct Server {
    role: String,
    handlers: HashMap<String, InternalHandler>,
}

impl Server {
    pub fn new(role: String) -> Self {
        Self {
            role,
            handlers: HashMap::new(),
        }
    }

    pub fn register_raw_handler(&mut self, handler: InternalHandler, identifier: String) {
        self.handlers.insert(identifier, handler);
    }

    pub fn run(&self) {
        let role = self.role.clone();
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
                // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
                ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
                // create callback
                let cloned_ws = ws.clone();
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

                    info!("got {:#?}", message);
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
                let onopen_callback = Closure::<dyn FnMut()>::new(move || {
                    // match cloned_ws.send_with_str("ping") {
                    //     Ok(_) => console_log!("message successfully sent"),
                    //     Err(err) => console_log!("error sending message: {:?}", err),
                    // }
                    // // send off binary message
                    // match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
                    //     Ok(_) => console_log!("binary message successfully sent"),
                    //     Err(err) => console_log!("error sending message: {:?}", err),
                    // }
                });
                ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
                onopen_callback.forget();
            }
        });
    }

    pub fn stop(&self) -> Result<(), String> {
        //TODO
        Err("Stopping not supported yet".to_string())
    }
}
