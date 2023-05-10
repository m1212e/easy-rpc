use std::fmt::format;

use erpc::{protocol, target::TargetType};
use flume::r#async;
use log::{error, warn};
use wasm_bindgen::{JsCast, JsError, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::CREATED_TARGETS;

#[derive(Debug, Clone)]
pub struct Target {
    address: String,
    target_type: TargetType,
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
        };
        CREATED_TARGETS.send(t.clone()).unwrap();
        t
    }

    pub async fn call(&self, request: protocol::Request) -> Result<protocol::Response, JsValue> {
        match self.target_type {
            TargetType::HTTPServer => {
                let mut opts = RequestInit::new();
                opts.method("POST");
                opts.mode(RequestMode::Cors);

                let body = serde_wasm_bindgen::to_value(
                    &serde_json::to_string(&request.parameters).map_err(|err| {
                        JsError::new(&format!("Could not serialize parameters: {err}"))
                    })?,
                )
                .map_err(|err| format!("Could not serialize body: {err}"))?;
                opts.body(Some(&body));

                let url = format!("{}/handlers/{}", self.address, request.identifier);
                let request = Request::new_with_str_and_init(&url, &opts)
                    .map_err(|err| format!("Could not create request: {:#?}", err))?;

                let window = web_sys::window().ok_or("Could not access window object")?;
                let resp_value = JsFuture::from(window.fetch_with_request(&request))
                    .await
                    .map_err(|err| format!("Could not fetch: {:#?}", err))?;

                // `resp_value` is a `Response` object.
                assert!(resp_value.is_instance_of::<Response>());
                let resp: Response = resp_value.dyn_into().expect("Cannot convert into response");

                let body = JsFuture::from(resp.array_buffer()?).await?;
                let body: serde_json::Value =
                    serde_json::from_slice(&js_sys::Uint8Array::new(&body).to_vec())
                        .map_err(|err| format!("Failed to parse JSON: {}", err))?;

                let response = protocol::Response { body };

                Ok(response)
            }
            TargetType::Browser => {
                Err(JsError::new("Browser to browser is not supported yet").into())
            }
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}
