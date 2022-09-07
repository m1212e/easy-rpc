mod initialize;
mod initialized;
mod jsonrpc;
mod show_message;
mod shutdown;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use tokio::sync::mpsc;

use crate::transpiler::ERPCError;

use self::{
    initialize::ServerCapabilities,
    jsonrpc::{Error, JSONRPCServer},
    show_message::MessageType,
};

#[derive(Clone)]
pub struct LanguageServer {
    pub handlers: Arc<
        RwLock<
            HashMap<
                String,
                Box<
                    dyn Fn(Option<serde_json::Value>) -> Result<serde_json::Value, Error>
                        + Send
                        + Sync,
                >,
            >,
        >,
    >,
    pub server: JSONRPCServer,
}

impl LanguageServer {
    pub fn new() -> LanguageServer {
        LanguageServer {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            server: JSONRPCServer::new(),
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        self.server.run(self.handlers.clone()).await
    }

    fn register_handler<
        F: Fn(Option<serde_json::Value>) -> Result<serde_json::Value, Error> + Send + Sync + 'static,
    >(
        &mut self,
        method: String,
        handler: F,
    ) {
        self.handlers
            .write()
            .unwrap()
            .insert(method, Box::new(handler));
    }
}

pub async fn start_language_server(mut rec: mpsc::Receiver<ERPCError>) {
    let mut ls = LanguageServer::new();

    ls.on_initialize(|_| {
        Ok(initialize::Response {
            capabilities: ServerCapabilities {},
        })
    });
    ls.on_shutdown(|| Ok(()));
    ls.on_initialized(|| {});

    // tokio::spawn(async move {
    //     loop {
    //         match rec.recv().await {
    //             Some(val) => {}
    //             None => break,
    //         };
    //     }
    // });

    let ls2 = ls.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(4)).await;

        ls2.show_message(MessageType::Info, "This is a test info message".to_string())
            .await
            .unwrap()
            .unwrap();
    });

    ls.run().await.unwrap();
}
