use super::{jsonrpc::Error, LanguageServer};

impl LanguageServer {
    //TODO: correct input params

    pub fn on_shutdown<F: Fn() -> Result<(), Error> + Send + Sync + 'static>(
        &mut self,
        handler: F,
    ) {
        self.register_handler("shutdown".to_string(), move |param| {
            match param {
                Some(_) => {
                    return Err(Error {
                        code: -32602,
                        data: None,
                        message: "Params cant be some".to_string(),
                    })
                }
                None => {}
            };

            match handler() {
                Ok(_) => match serde_json::to_value("{}") {
                    Ok(val) => Ok(val),
                    Err(val) => Err(Error::from(val)),
                },
                Err(err) => Err(err),
            }
        });
    }
}
