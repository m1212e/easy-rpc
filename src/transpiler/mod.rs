pub mod config;
mod generator;
mod parser;
mod tests;
pub mod validator;

use std::{
    fmt::Display,
    io::{self},
    path::Path,
};

use async_channel::RecvError;
use serde_json;

use self::{
    generator::{generate_for_directory, translator::typescript::TypeScriptTranslator},
    parser::{input_reader::InputReaderError, parser::ParseError},
    validator::ValidationError,
};

#[derive(Debug)]
pub enum ERPCError {
    /**
        Error which occured on the input reader while parsing erpc source files
    */
    InputReaderError(InputReaderError),
    /**
       Error indicating a mistake in the logical structure of the file. E.g. unknown or double defined types.
       Second value is the path to the file on which the error occured
    */
    ValidationError((ValidationError, String)),
    /**
       Error which occured while parsing the erpc inputs. Mostly logical/syntactical.
       Second value is the path to the file on which the error occured
    */
    ParseError((ParseError, String)),
    /**
       Error which occured while parsing JSON config files.
    */
    JSONError(serde_json::Error),
    /**
       Error indicating a mistake in the setup/configuration of easy-rpc. E.g. a missing file or directory.
    */
    ConfigurationError(String),
    /**
        An IO error while processing non .erpc files
    */
    IO(io::Error),
    /**
       Error occuring while watching a dir
    */
    NotifyError(notify::Error),
    /**
       Recv Error
    */
    RecvError(RecvError),
}

impl From<InputReaderError> for ERPCError {
    fn from(err: InputReaderError) -> Self {
        ERPCError::InputReaderError(err)
    }
}
impl From<(ParseError, String)> for ERPCError {
    fn from(err: (ParseError, String)) -> Self {
        ERPCError::ParseError(err)
    }
}
impl From<serde_json::Error> for ERPCError {
    fn from(err: serde_json::Error) -> Self {
        ERPCError::JSONError(err)
    }
}
impl From<io::Error> for ERPCError {
    fn from(err: io::Error) -> Self {
        ERPCError::IO(err)
    }
}
impl From<String> for ERPCError {
    fn from(err: String) -> Self {
        ERPCError::ConfigurationError(err)
    }
}
impl From<(ValidationError, String)> for ERPCError {
    fn from(err: (ValidationError, String)) -> Self {
        ERPCError::ValidationError(err)
    }
}
impl From<notify::Error> for ERPCError {
    fn from(err: notify::Error) -> Self {
        ERPCError::NotifyError(err)
    }
}
impl From<RecvError> for ERPCError {
    fn from(err: RecvError) -> Self {
        ERPCError::RecvError(err)
    }
}

impl Display for ERPCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ERPCError {
    pub fn to_string(&self) -> String {
        match self {
            ERPCError::InputReaderError(val) => {
                format!("InputReaderError:\n{}", val)
            }
            ERPCError::ValidationError(val) => {
                format!("ValidationError:\n{:#?}\n in:{}\n", val.0, val.1)
            }
            ERPCError::ParseError(val) => {
                format!("ParseError:\n{:#?}\n in: {}\n", val.0, val.1)
            }
            ERPCError::JSONError(val) => {
                format!("JSONError:\n{}", val)
            }
            ERPCError::ConfigurationError(val) => {
                format!("ConfigurationError:\n{}", val)
            }
            ERPCError::IO(val) => {
                format!("IOError:\n{}", val)
            }
            ERPCError::NotifyError(val) => {
                format!("NotifyError:\n{}", val)
            }
            ERPCError::RecvError(val) => {
                format!("RecvError:\n{}", val)
            }
        }
    }
}

pub async fn run(source: &Path, output: &Path, selected_role_name: &str) -> Vec<ERPCError> {
    match generate_for_directory::<TypeScriptTranslator>(source, output, selected_role_name) {
        Ok(_) => vec![],
        Err(err) => vec![err],
    }
}
