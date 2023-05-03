use erpc::protocol;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub enum TargetType {
    Server,
    Browser,
}

#[wasm_bindgen]
pub struct Options {
    address: String,
}

#[wasm_bindgen]
pub struct ERPCTarget {
    target_type: TargetType,
    options: Options,
}

impl ERPCTarget {
    pub fn new(mut options: Options, target_type: TargetType) -> Self {
        let mut address = options.address.trim().trim_end_matches('/').to_string();
        if address.starts_with("http") {
            address = format!("https://{}", address);
        }
        options.address = address;

        Self {
            target_type,
            options,
        }
    }

    // #[wasm_bindgen(skip_typescript)]
    pub async fn call(&self, identifier: String, parameters: JsValue) -> Result<JsValue, JsValue> {
        match self.target_type {
            TargetType::Server => {}
            TargetType::Browser => {
                return Err(JsValue::from_str(
                    "Sending requests to other browsers is currently unsupported",
                ))
            }
        };

        //TODO this is probably not ideal
        // make sure the request struct is created to break this when the protocol should change in the future
        let _ = erpc::protocol::Request {
            identifier: identifier.clone(),
            parameters: serde_wasm_bindgen::from_value(parameters.clone())?,
        };

        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);
        opts.body(Some(&parameters));

        let url = format!("{}/{}", self.options.address, identifier);
        let request = Request::new_with_str_and_init(&url, &opts)?;

        let window = web_sys::window().expect("Cold not access window object");
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        // `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().expect("Cannot convert into response");

        let buffer_promise = resp.array_buffer()?;
        let value: protocol::Response =
            serde_wasm_bindgen::from_value(JsFuture::from(buffer_promise).await?)?;

        serde_wasm_bindgen::to_value(&value.body).map_err(|err| {
            JsValue::from_str(&format!("Could not serialize response body to js: {err}"))
        })
    }
}
