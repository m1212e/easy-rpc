use wasm_bindgen::{prelude::wasm_bindgen, JsValue, JsCast};
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
        let mut address = options.address.trim().trim_end_matches("/").to_string();
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
    pub async fn call(
        &self,
        methodIdentifier: String,
        parameters: Vec<JsValue>,
    ) -> Result<JsValue, JsValue> {
        match self.target_type {
            TargetType::Server => {}
            TargetType::Browser => {
                return Err(
                    JsValue::from_str("Sending requests to other browsers is currently unsupported"),
                )
            }
        };

        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);

        let url = format!("{}/{}", self.options.address, methodIdentifier);
        let request = Request::new_with_str_and_init(&url, &opts)?;

        // request
        //     .headers()
        //     .set("Accept", "application/vnd.github.v3+json")?;

        let window = web_sys::window().expect("Cold not access window object");
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        // `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().expect("Cannot convert into response");

        let b = JsFuture::from(resp.blob()?).await?;

        // Convert this other `Promise` into a rust `Future`.
        // let response_value: Vec<u8> = JsFuture::from(resp.json()?).await?;

        // Send the JSON response back to JS.
        Ok(response_value.body)
    }
}
