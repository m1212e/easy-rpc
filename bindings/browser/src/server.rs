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
    #[wasm_bindgen(skip_typescript)]
    pub fn free(&self) {}

    #[wasm_bindgen(constructor)]
    pub fn new(
        options: ServerOptions,
        server_type: String,
        enable_sockets: bool,
        role: String,
    ) -> Self {
        Self {
            server: http_client_wasm::Server::new(role),
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
                        protocol::error::SendableError::from(format!(
                            "Apply call failed: {:#?}",
                            err
                        ))
                    })
                    .map(|v| serde_wasm_bindgen::from_value(v).unwrap());

                protocol::Response { body: result }
            }),
            identifier,
        );
    }

    #[wasm_bindgen]
    pub fn run(&self) {
        self.server.run();
    }

    // #[wasm_bindgen]
    // pub fn stop(&self) {
    //     self.server.stop();
    // }
}
