use http_server::Socket;
use napi::{Env, JsObject, JsUnknown};

#[napi(object)]
pub struct TargetOptions {
    pub address: String,
}

#[napi(js_name = "ERPCTarget")]
pub struct ERPCTarget {
    target: http_server::Target,
}

#[napi]
impl ERPCTarget {
    #[napi(constructor)]
    pub fn new(options: TargetOptions, target_type: String) -> Self {
        let target_type = match target_type.as_str() {
            "browser" => http_server::TargetType::Browser,
            "http-server" => http_server::TargetType::HTTPServer,
            _ => panic!("Unsupported target type {}", target_type),
        };

        ERPCTarget {
            target: http_server::Target::new(options.address, target_type),
        }
    }

    #[napi(skip_typescript)]
    pub fn call(
        &self,
        env: Env,
        method_identifier: String,
        parameters: Option<Vec<serde_json::Value>>,
    ) -> Result<JsObject, napi::Error> {
        let t = self.target.clone();

        env.execute_tokio_future(
            async move {
                let res = match t
                    .call(erpc::protocol::Request {
                        identifier: method_identifier,
                        parameters: parameters.unwrap_or_default(),
                    })
                    .await
                {
                    Ok(v) => v.body,
                    Err(err) => {
                        return Err(napi::Error::from_reason(err));
                    }
                };

                Ok(res)
            },
            |env, data| {
                let ret: JsUnknown = env.to_js_value(&data)?;
                Ok(ret)
            },
        )
    }

    #[napi(skip_typescript, js_name = "setERPCSocket")]
    pub fn set_erpc_socket(&self, env: Env, socket: JsObject) -> Result<(), napi::Error> {
        let mut t = self.target.clone();
        let socket: &mut Socket = env.unwrap(&socket)?;
        let socket = socket.clone();
        env.execute_tokio_future(
            async move {
                t.listen_on_socket(socket).await;
                Ok(())
            },
            |_, _| Ok(()),
        )?;
        Ok(())
    }
}
