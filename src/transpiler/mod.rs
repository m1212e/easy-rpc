mod config;
mod generator;
mod parser;
mod validator;
mod tests;

use std::{
    fs::File,
    io::{self},
    path::Path,
};

use serde_json;

use self::{
    config::parse_config,
    parser::{input_reader::InputReaderError, parser::ParseError}, validator::ValidationError,
};

#[derive(Debug)]
pub enum ERPCError {
    /**
       Error which occured on the input reader while parsing erpc source files
    */
    InputReaderError(InputReaderError),
    /**
       Error which occured while parsing the erpc inputs. Mostly logical/syntactical.
    */
    ParseError(ParseError),
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
       Error indicating a mistake in the logical structure of the file. E.g. unknown or double defined types
    */
    ValidationError(ValidationError),
}

impl From<InputReaderError> for ERPCError {
    fn from(err: InputReaderError) -> Self {
        ERPCError::InputReaderError(err)
    }
}
impl From<ParseError> for ERPCError {
    fn from(err: ParseError) -> Self {
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
impl From<ValidationError> for ERPCError {
    fn from(err: ValidationError) -> Self {
        ERPCError::ValidationError(err)
    }
}

fn run(input_directory: &Path) -> Result<(), ERPCError> {
    let config = parse_config(File::open(input_directory.join("erpc.json"))?)?;

    // for source in config.sources {
    //     generate_for_directory_recursively(
    //         &input_directory.join("erpc.json").join(source),
    //         &input_directory.join(".erpc").join("generated"),
    //         &Path::new("/"),
    //         &config.role,
    //     )?
    // }

    Ok(())
}
