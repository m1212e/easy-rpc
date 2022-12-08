use std::{fmt::Display, path::PathBuf};

use tower_lsp::lsp_types::Range;

/**
 * An error of some kind, which can be displayed to the user
 */
#[derive(Debug)]
pub enum DisplayableError {
    Message(Message),
    Diagnostic(Diagnostic),
}

#[derive(Debug)]
pub struct Diagnostic {
    /**
     * File where the error occurred
     */
    pub source: PathBuf,
    /**
     * The range where the error occurred
     */
    pub range: Range,
    /**
     * The message to display. This should explain the cause of the error
     */
    pub message: String,
}

#[derive(Debug)]
pub struct Message {
    /**
     * The message to display. This should explain the cause of the error
     */
    pub message: String,
}

impl From<String> for DisplayableError {
    fn from(message: String) -> Self {
        DisplayableError::Message(Message { message })
    }
}

impl From<notify::Error> for DisplayableError {
    fn from(err: notify::Error) -> Self {
        DisplayableError::Message(Message {
            message: format!("Error in file system notifier occurred: {err}"),
        })
    }
}

impl From<async_channel::RecvError> for DisplayableError {
    fn from(err: async_channel::RecvError) -> Self {
        DisplayableError::Message(Message {
            message: format!("Recv error occurred: {err}"),
        })
    }
}
// impl From<notify::Error> for ERPCError {
//     fn from(err: notify::Error) -> Self {
//         ERPCError::NotifyError(err)
//     }
// }

impl Display for DisplayableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DisplayableError::Message(v) => v.message.to_owned(),
                DisplayableError::Diagnostic(v) => format!(
                    "{} in {}:{}:{}\n",
                    v.message,
                    v.source.to_str().unwrap_or("<could not unwrap path>"),
                    v.range.start.line+1,
                    v.range.start.character+1,
                ),
            }
        )
    }
}

impl DisplayableError {
    pub fn message(self) -> String {
        match self {
            DisplayableError::Message(v) => v.message,
            DisplayableError::Diagnostic(v) => v.message,
        }
    }
}
