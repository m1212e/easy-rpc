mod jsonrpc;

use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::transpiler::ERPCError;

use self::jsonrpc::{Error, JSONRPCServer};

pub struct LanguageServer {
    pub handlers:
        HashMap<String, Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, Error>>>,
}

impl LanguageServer {
    pub fn new() -> LanguageServer {
        LanguageServer {
            handlers: HashMap::new(),
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        JSONRPCServer::new(
            tokio::io::stdin(),
            tokio::io::stdout(),
            |method, value| match self.handlers.get(&method) {
                Some(handler) => handler(value),
                None => Err(Error {
                    code: -32601,
                    message: "The requested method could not be found".to_string(),
                    data: None,
                }),
            },
        )
        .run()
        .await
    }

    pub fn register_handler<F: Fn(serde_json::Value) -> Result<serde_json::Value, Error>>(
        &mut self,
        method: String,
        handler: F,
    ) {
        self.handlers.insert(method, Box::new(handler));
    }
}

pub async fn start_language_server(rec: mpsc::Receiver<ERPCError>) {
    let ls = LanguageServer::new();

    ls.register_handler("something".to_string(), |param| {});
}
