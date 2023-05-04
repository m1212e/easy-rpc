use erpc::protocol;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::CREATED_TARGETS;

#[derive(Debug, Clone)]
pub enum TargetType {
    HTTPServer,
    Browser,
}

#[derive(Debug, Clone)]
pub struct Target {
    address: String,
    target_type: TargetType,
}

impl Target {
    pub fn new(mut address: String, port: u16, target_type: TargetType) -> Self {
        if address.ends_with('/') {
            address.pop();
        }

        let t = Target {
            address,
            target_type,
        };
        CREATED_TARGETS.send(t.clone());
        t
    }

    pub async fn call(&self, request: protocol::Request) -> Result<protocol::Response, String> {
        match self.target_type {
            TargetType::HTTPServer => {
                let mut opts = RequestInit::new();
                opts.method("POST");
                opts.mode(RequestMode::Cors);
                opts.body(Some(&request.parameters));

                let url = format!("{}/{}", self.options.address, request.identifier);
                let request = Request::new_with_str_and_init(&url, &opts)?;

                let window = web_sys::window().ok_or("Could not access window object")?;
                let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

                // `resp_value` is a `Response` object.
                assert!(resp_value.is_instance_of::<Response>());
                let resp: Response = resp_value.dyn_into().expect("Cannot convert into response");

                Ok(serde_wasm_bindgen::from_value(
                    JsFuture::from(resp.array_buffer()?).await?,
                ))
            }
            TargetType::Browser => Err("Requests to other browsers are not supported yet"),
        }
    }
}
