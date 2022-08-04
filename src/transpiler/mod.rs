mod generator;
mod parser;
mod tests;

use std::{
    fs::{self, read_dir, File},
    io::{self, Read},
    path::Path,
};

use serde::Deserialize;
use serde_json;

use self::{
    parser::{input_reader::InputReaderError, parser::ParseError},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub name: String,
    pub types: Vec<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    pub sources: Vec<String>,
    pub role: String,
}

fn parse_roles<T: Read>(input: T) -> Result<Vec<Role>, serde_json::Error> {
    serde_json::from_reader(input)
}

fn parse_config<T: Read>(input: T) -> Result<Config, serde_json::Error> {
    serde_json::from_reader(input)
}

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
