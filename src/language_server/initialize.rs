use serde::{Deserialize, Serialize};

use super::{jsonrpc::Error, LanguageServer};

#[derive(Deserialize)]
pub struct Parameters {}
#[derive(Serialize)]
pub struct Response {
    pub capabilities: ServerCapabilities,
}

#[derive(Serialize)]
pub struct ServerCapabilities {}

impl LanguageServer {
    //TODO: correct input params

    pub fn on_initialize<F: Fn(Parameters) -> Result<Response, Error> + Send + Sync + 'static>(
        &mut self,
        handler: F,
    ) {
        self.register_handler("initialize".to_string(), move |param| {
            let param = match param {
                Some(v) => v,
                None => {
                    return Err(Error {
                        code: -32602,
                        data: None,
                        message: "Params cant be none".to_string(),
                    });
                }
            };

            let parsed_param = serde_json::from_str::<Parameters>(&param.to_string())?;

            match handler(parsed_param) {
                Ok(val) => match serde_json::to_value(val) {
                    Ok(val) => Ok(val),
                    Err(val) => Err(Error::from(val)),
                },
                Err(err) => Err(err),
            }
        });
    }
}
