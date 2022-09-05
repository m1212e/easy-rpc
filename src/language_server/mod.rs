mod initialize;
mod jsonrpc;
mod show_message;

use std::{collections::HashMap, time::Duration};

use tokio::sync::mpsc;

use crate::transpiler::ERPCError;

use self::{jsonrpc::{Error, JSONRPCServer}, show_message::MessageType};

pub struct LanguageServer {
    pub handlers:
        HashMap<String, Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, Error>>>,
    pub server: JSONRPCServer,
}

impl LanguageServer {
    pub fn new() -> LanguageServer {
        LanguageServer {
            handlers: HashMap::new(),
            server: JSONRPCServer::new(tokio::io::stdin(), tokio::io::stdout()),
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        self.server.run(&self.handlers).await
    }

    fn register_handler<F: Fn(serde_json::Value) -> Result<serde_json::Value, Error> + 'static>(
        &mut self,
        method: String,
        handler: F,
    ) {
        self.handlers.insert(method, Box::new(handler));
    }
}

pub async fn start_language_server(mut rec: mpsc::Receiver<ERPCError>) {
    let mut ls = LanguageServer::new();

    ls.on_initialize(|_| Ok(initialize::Response {}));

    // tokio::spawn(async move {
    //     loop {
    //         match rec.recv().await {
    //             Some(val) => {}
    //             None => break,
    //         };
    //     }
    // });

    let mut ls = &ls;
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(5)).await;

        ls.show_message(MessageType::Info, "This is a test info message".to_string())
    });


    ls.run().await.unwrap();
}
