use serde::Serialize;
use tokio::sync::oneshot::{self, Receiver};

use super::{jsonrpc::Error, LanguageServer};

#[derive(Serialize)]
pub struct Parameters {
    #[serde(rename = "type")]
    message_type: u8,
    message: String,
}

#[derive(Serialize)]
pub enum MessageType {
    /**
     * An error message.
     */
    Error,
    /**
     * A warning message.
     */
    Warning,
    /**
     * An information message.
     */
    Info,
    /**
     * A log message.
     */
    Log,
}

impl From<MessageType> for u8 {
    fn from(val: MessageType) -> Self {
        match val {
            MessageType::Error => 1,
            MessageType::Warning => 2,
            MessageType::Info => 3,
            MessageType::Log => 4,
        }
    }
}

impl LanguageServer {
    pub fn show_message(
        &self,
        message_type: MessageType,
        message: String,
    ) -> Receiver<Result<(), Error>> {
        let (sender, reciever) = oneshot::channel::<Result<(), Error>>();

        let r = self.server.send_request(
            "window/showMessage",
            match serde_json::to_value(Parameters {
                message,
                message_type: message_type.into(),
            }) {
                Ok(val) => Some(val),
                Err(err) => {
                    sender.send(Err(err.into())).unwrap();
                    return reciever;
                }
            },
            true,
        );

        tokio::spawn(async move {
            match r.await {
                Ok(val) => match val {
                    Ok(_) => sender.send(Ok(())),
                    Err(err) => sender.send(Err(err.into())),
                },
                Err(err) => sender.send(Err(err.into())),
            }
            .unwrap();
        });

        reciever
    }
}
