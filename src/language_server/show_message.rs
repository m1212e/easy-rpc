use serde::Serialize;

use super::{jsonrpc::Error, LanguageServer};

#[derive(Serialize)]
pub struct Parameters {
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
    pub async fn show_message(
        &mut self,
        message_type: MessageType,
        message: String,
    ) -> Result<(), Error> {
        match self
            .server
            .send_request(
                "window/showMessage",
                serde_json::to_value(Parameters {
                    message,
                    message_type: message_type.into(),
                })?,
                true,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
