mod config;
mod generator;
mod parser;
mod tests;
mod validator;

use std::{
    fs::File,
    io::{self},
    path::Path,
    sync::mpsc::{channel, RecvError},
    time::Duration,
};

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use serde_json;

use crate::util::normalize_path::normalize_path;

use self::{
    config::parse_config,
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
       Error occuring in the watcher channel
    */
    RecvError(RecvError),
    /**
       Error occuring while watching a dir
    */
    NotifyError(notify::Error),
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
impl From<RecvError> for ERPCError {
    fn from(err: RecvError) -> Self {
        ERPCError::RecvError(err)
    }
}
impl From<notify::Error> for ERPCError {
    fn from(err: notify::Error) -> Self {
        ERPCError::NotifyError(err)
    }
}

impl ERPCError {
    pub fn to_string(&self) -> String {
        match self {
            ERPCError::InputReaderError(val) => {
                format!("InputReaderError:\n{}", val)
            }
            ERPCError::ValidationError(val) => {
                format!("ValidationError in {}:\n{:#?}", val.1, val.0)
            }
            ERPCError::ParseError(val) => {
                format!("ParseError in {}:\n{:#?}", val.1, val.0)
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
            ERPCError::RecvError(val) => {
                format!("RecvError:\n{}", val)
            }
            ERPCError::NotifyError(val) => {
                format!("NotifyError:\n{}", val)
            }
        }
    }
}

/**
   Runs the transpiler on an input directory. Expects a erpc.json to parse in the specified directory.
*/
pub fn run(input_directory: &Path, watch: bool) -> Result<(), ERPCError> {
    let path = input_directory.join("erpc.json");
    if !path.exists() {
        return Err(ERPCError::ConfigurationError(format!(
            "Could not find erpc.json at {path_str}",
            path_str = path
                .as_os_str()
                .to_str()
                .unwrap_or("<Unable to unwrap path>")
        )));
    }
    let config = parse_config(File::open(path)?)?;

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    for source in &config.sources {
        let path = normalize_path(&input_directory.join(source));
        if watch {
            watcher.watch(&path, RecursiveMode::Recursive).unwrap();
        } else {
            generate_for_directory::<TypeScriptTranslator>(
                &path,
                &input_directory.join(".erpc").join("generated"),
                &config.role,
            )?;
        }
    }

    if watch {
        loop {
            match rx.recv()? {
                DebouncedEvent::Create(val)
                | DebouncedEvent::Write(val)
                | DebouncedEvent::Remove(val)
                | DebouncedEvent::Rename(val, _) => {
                    match config.sources.iter().find_map(|source| {
                        let path = normalize_path(&input_directory.join(source));
                        if val.starts_with(&path) {
                            return Some(path);
                        }
                        None
                    }) {
                        Some(path) => {
                            generate_for_directory::<TypeScriptTranslator>(
                                &path,
                                &input_directory.join(".erpc").join("generated"),
                                &config.role,
                            )?;
                        }
                        None => {
                            return Err(ERPCError::ConfigurationError(
                                "Could not find correct source while processing in watchmode"
                                    .to_string(),
                            ));
                        }
                    };
                }
                DebouncedEvent::Error(err, _) => {
                    return Err(ERPCError::NotifyError(err));
                }
                _ => {}
            }
        }
    }

    Ok(())
}
