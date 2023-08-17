use serde::Deserialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

#[wasm_bindgen(typescript_custom_section)]
const TARGET_OPTIONS: &'static str = r#"
interface TargetOptions {
    address: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "TargetOptions")]
    pub type TargetOptions;
}

#[derive(Deserialize)]
struct InternalTargetOptions {
    address: String,
}

#[wasm_bindgen]
pub struct ERPCTarget {
    target: http_client_wasm::Target,
}

#[wasm_bindgen]
impl ERPCTarget {
    #[wasm_bindgen(constructor)]
    pub fn new(options: TargetOptions, target_type: &str) -> Result<ERPCTarget, JsValue> {
        let js_value: JsValue = options.into();

        let options: InternalTargetOptions = serde_wasm_bindgen::from_value(js_value)?;

        Ok(ERPCTarget {
            target: http_client_wasm::Target::new(
                options.address,
                match target_type {
                    "http-server" => erpc::target::TargetType::HttpServer,
                    "browser" => erpc::target::TargetType::Browser,
                    _ => return Err(JsError::new("Invalid value for target type").into()),
                },
            ),
        })
    }

    //TODO remove unwraps
    #[wasm_bindgen(skip_typescript)]
    pub async fn call(
        &self,
        identifier: String,
        parameters: Vec<JsValue>,
    ) -> Result<JsValue, JsValue> {
        let parameters = parameters
            .into_iter()
            .map(|param| serde_wasm_bindgen::from_value(param).unwrap())
            .collect();

        let result = self
            .target
            .call(erpc::protocol::Request {
                identifier,
                parameters,
            })
            .await;

        match result.body {
            Ok(v) => Ok(serde_wasm_bindgen::to_value(&v).unwrap()),
            Err(err) => Err(JsError::new(&err.to_string()).into()),
        }
    }
}
