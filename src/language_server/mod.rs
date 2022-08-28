mod jsonrpc;

use std::future::Future;

use tokio::sync::mpsc;

use crate::transpiler::ERPCError;

use self::jsonrpc::{JSONRPCServer};

pub struct LanguageServer
{
    // jsonrpc_server: JSONRPCServer<F, Fut>,
}

impl LanguageServer
{
    pub fn new() -> LanguageServer {

        JSONRPCServer::new(tokio::io::stdin(), tokio::io::stdout(), |method, value| {
            async {
                // Option<serde_json::Value>, Option<Error>
                (None, None)
            }
        });

        LanguageServer { }
    }
}

pub async fn start_language_server(rec: mpsc::Receiver<ERPCError>) {
}
