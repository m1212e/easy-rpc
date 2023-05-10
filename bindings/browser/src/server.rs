use std::iter::FromIterator;

use erpc::protocol;
use log::error;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(typescript_custom_section)]
const SERVER_OPTIONS: &'static str = r#"
interface ServerOptions {}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ServerOptions")]
    pub type ServerOptions;
}

#[wasm_bindgen]
pub struct ERPCServer {
    server: http_client_wasm::Server,
}

//TODO remove unwraps
#[wasm_bindgen]
impl ERPCServer {
    #[wasm_bindgen(constructor)]
    pub fn new(options: ServerOptions, server_type: String, enable_sockets: bool, role: String) -> Self {
        Self {
            server: http_client_wasm::Server::new()
        }
    }

    #[wasm_bindgen(skip_typescript, js_name = "registerERPCHandler")]
    pub fn register_erpc_handler(&mut self, handler: js_sys::Function, identifier: String) {
        self.server.register_raw_handler(
            Box::new(move |input| {
                let parameters = js_sys::Array::from_iter(
                    input
                        .parameters
                        .iter()
                        .map(|param| serde_wasm_bindgen::to_value(param).unwrap()),
                );

                let result = handler
                    .apply(&JsValue::null(), &parameters)
                    .map_err(|err| {
                        error!("Apply call failed: {:#?}", err);
                        "Aplly call failed".to_string()
                    })?;

                Ok(protocol::Response {
                    body: serde_wasm_bindgen::from_value(result).unwrap(),
                })
            }),
            identifier,
        );
    }
}
