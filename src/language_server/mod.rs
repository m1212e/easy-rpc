use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::PathBuf;

use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::error::DisplayableError;

#[derive(Debug)]
struct Backend {
    client: Client,
    error_reciever: async_channel::Receiver<Vec<DisplayableError>>,
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
            let mut last: Vec<PathBuf> = Vec::new();
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

                let mut diagnostics_per_origin: HashMap<PathBuf, Vec<Diagnostic>> = HashMap::new();

                // push all previous errors with an empty diagnostics array to overwrite errors which don't exist anymore
                for origin in &last {
                    diagnostics_per_origin.insert(origin.to_owned(), vec![]);
                }

                last.clear();

                for err in recieved {
                    match err {
                        DisplayableError::Message(err) => {
                            client.show_message(MessageType::ERROR, err.message).await;
                        }
                        DisplayableError::Diagnostic(err) => {
                            let d = Diagnostic {
                                range: err.range,
                                severity: Some(DiagnosticSeverity::ERROR),
                                code: None,
                                code_description: None,
                                source: None,
                                message: err.message,
                                related_information: None,
                                tags: None,
                                data: None,
                            };

                            match diagnostics_per_origin.entry(err.source) {
                                Entry::Occupied(mut entry) => {
                                    entry.get_mut().push(d);
                                }
                                Entry::Vacant(entry) => {
                                    entry.insert(vec![d]);
                                }
                            }
                        }
                    }
                }

                for (origin, diagnostics) in diagnostics_per_origin {
                    last.push(origin.to_owned());
                    let path = match origin.to_str() {
                        Some(v) => v,
                        None => {
                            client.show_message(MessageType::ERROR, "Could not convert path to string to display diagnostics").await;
                            continue;
                        }
                    };
                    client
                        .publish_diagnostics(
                            Url::parse(&format!("file://{path}")).unwrap(),
                            diagnostics,
                            None,
                        )
                        .await;
                }
            }
        });
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        //TODO
        Ok(())
    }

    // async fn execute_command(
    //     &self,
    //     _: ExecuteCommandParams,
    // ) -> tower_lsp::jsonrpc::Result<Option<Value>> {
    //     self.client
    //         .log_message(MessageType::INFO, "command executed!")
    //         .await;

    //     Ok(None)
    // }

    // async fn completion(
    //     &self,
    //     _: CompletionParams,
    // ) -> tower_lsp::jsonrpc::Result<Option<CompletionResponse>> {
    //     Ok(Some(CompletionResponse::Array(vec![
    //         CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
    //         CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
    //     ])))
    // }
}

pub async fn run_language_server(error_reciever: async_channel::Receiver<Vec<DisplayableError>>) {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(|client| Backend {
        client,
        error_reciever,
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
