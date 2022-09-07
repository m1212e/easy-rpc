use crate::transpiler::ERPCError;

use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    error_reciever: async_channel::Receiver<Option<ERPCError>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
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
            loop {
                let r_err = {
                    let r_err = match error_reciever.recv().await {
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
                    match r_err {
                        Some(val) => val,
                        None => continue, //TODO clear errors
                    }
                };
                match r_err {
                    ERPCError::ValidationError(err) => todo!(),
                    ERPCError::ParseError(err) => {
                        client
                            .publish_diagnostics(
                                Url::parse(&format!("file://{}", err.1)).unwrap(),
                                vec![Diagnostic {
                                    range: Range {
                                        start: err.0.start.into(),
                                        end: err.0.end.into(),
                                    },
                                    severity: Some(DiagnosticSeverity::ERROR),
                                    code: None,
                                    code_description: None,
                                    source: None,
                                    message: err.0.message,
                                    related_information: None,
                                    tags: None,
                                    data: None,
                                }],
                                None,
                            )
                            .await;
                    }
                    ERPCError::InputReaderError(_)
                    | ERPCError::JSONError(_)
                    | ERPCError::ConfigurationError(_)
                    | ERPCError::IO(_)
                    | ERPCError::NotifyError(_) => {
                        client
                            .show_message(
                                MessageType::ERROR,
                                format!("Error occured: {}", r_err.to_string()),
                            )
                            .await
                    }
                };
            }
        });
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, "command executed!")
            .await;

        Ok(None)
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }
}

pub async fn start_language_server(error_reciever: async_channel::Receiver<Option<ERPCError>>) {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(|client| Backend {
        client,
        error_reciever,
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
