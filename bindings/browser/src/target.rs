use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

#[wasm_bindgen]
pub struct ERPCTarget {
    target: http_client_wasm::Target,
}

#[wasm_bindgen]
pub struct TargetOptions {
    address: String,
}

#[wasm_bindgen]
impl ERPCTarget {
    #[wasm_bindgen(constructor)]
    pub fn new(options: TargetOptions, target_type: &str) -> Result<ERPCTarget, JsValue> {
        Ok(ERPCTarget {
            target: http_client_wasm::Target::new(
                options.address,
                match target_type {
                    "http-server" => erpc::target::TargetType::HTTPServer,
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
            .await?;

        Ok(serde_wasm_bindgen::to_value(&result.body).unwrap())
    }
}
