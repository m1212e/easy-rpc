use serde::{Deserialize, Serialize};

use super::{jsonrpc::Error, LanguageServer};

#[derive(Deserialize)]
pub struct Parameters {}
#[derive(Serialize)]
pub struct Response {}

impl LanguageServer {
    //TODO: correct input params

    pub fn on_initialize<F: Fn(Parameters) -> Result<Response, Error> + 'static>(
        &mut self,
        handler: F,
    ) {
        self.register_handler("initialize".to_string(), move |param| {
            let parsed_param = serde_json::from_str::<Parameters>(&param.to_string())?;

            match serde_json::to_value(handler(parsed_param)) {
                Ok(val) => Ok(val),
                Err(val) => Err(Error::from(val)),
            }
        });
    }
}
