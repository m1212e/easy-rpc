use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::transpiler::ERPCError;

use serde_json::Value;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    error_reciever: async_channel::Receiver<Result<String, Vec<ERPCError>>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["dummy.do_something".to_string()],
                    work_done_progress_options: Default::default(),
                }),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        let error_reciever = self.error_reciever.clone();
        let client = self.client.clone();

        tokio::spawn(async move {
            let mut last: Vec<String> = Vec::new();
            loop {
                let recieved = match error_reciever.recv().await {
                    Ok(val) => val,
                    Err(err) => {
                        client
                            .show_message(
                                MessageType::ERROR,
                                format!("Recv error: {}", err.to_string()),
                            )
                            .await;
                        continue;
                    }
                };

                let mut diagnostics_per_origin: HashMap<String, Vec<Diagnostic>> = HashMap::new();

                for origin in &last {
                    diagnostics_per_origin.insert(origin.to_owned(), vec![]);
                }

                last.clear();

                if recieved.is_err() {
                    for err in recieved.unwrap_err() {
                        match err {
                            ERPCError::ValidationError((err, origin)) => {
                                let d = Diagnostic {
                                    range: Range {
                                        start: err.start.into(),
                                        end: err.end.into(),
                                    },
                                    severity: Some(DiagnosticSeverity::ERROR),
                                    code: None,
                                    code_description: None,
                                    source: None,
                                    message: err.message,
                                    related_information: None,
                                    tags: None,
                                    data: None,
                                };
                                match diagnostics_per_origin.entry(origin) {
                                    Entry::Occupied(mut entry) => {
                                        entry.get_mut().push(d);
                                    }
                                    Entry::Vacant(entry) => {
                                        entry.insert(vec![d]);
                                    }
                                }
                            }
                            ERPCError::ParseError((err, origin)) => {
                                let d = Diagnostic {
                                    range: Range {
                                        start: err.start.into(),
                                        end: err.end.into(),
                                    },
                                    severity: Some(DiagnosticSeverity::ERROR),
                                    code: None,
                                    code_description: None,
                                    source: None,
                                    message: err.message,
                                    related_information: None,
                                    tags: None,
                                    data: None,
                                };
                                match diagnostics_per_origin.entry(origin) {
                                    Entry::Occupied(mut entry) => {
                                        entry.get_mut().push(d);
                                    }
                                    Entry::Vacant(entry) => {
                                        entry.insert(vec![d]);
                                    }
                                }
                            }
                            ERPCError::InputReaderError(_)
                            | ERPCError::JSONError(_)
                            | ERPCError::ConfigurationError(_)
                            | ERPCError::IO(_)
                            | ERPCError::NotifyError(_)
                            | ERPCError::RecvError(_) => {
                                client
                                    .show_message(MessageType::ERROR, err.to_string())
                                    .await
                            }
                        };
                    }
                }

                for (origin, diagnostics) in diagnostics_per_origin {
                    last.push(origin.to_owned());
                    client
                        .publish_diagnostics(
                            Url::parse(&format!("file://{}", origin)).unwrap(),
                            diagnostics,
                            None,
                        )
                        .await;
                }
            }
        });
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    async fn execute_command(
        &self,
        _: ExecuteCommandParams,
    ) -> tower_lsp::jsonrpc::Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, "command executed!")
            .await;

        Ok(None)
    }

    async fn completion(
        &self,
        _: CompletionParams,
    ) -> tower_lsp::jsonrpc::Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }
}

pub async fn run_language_server(
    error_reciever: async_channel::Receiver<Result<String, Vec<ERPCError>>>,
) {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(|client| Backend {
        client,
        error_reciever,
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
