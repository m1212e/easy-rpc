mod jsonrpc;

use tokio::sync::mpsc;

use crate::transpiler::ERPCError;

pub async fn start_language_server(rec: mpsc::Receiver<ERPCError>) {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
}
