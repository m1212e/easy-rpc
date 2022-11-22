pub mod config;
mod generator;
mod parser;
mod tests;

pub mod validator;

use std::path::Path;

use crate::error::DisplayableError;

use self::generator::{generate_for_directory, translator::typescript::TypeScriptTranslator};

pub async fn run(source: &Path, output: &Path, selected_role_name: &str) -> Vec<DisplayableError> {
    generate_for_directory::<TypeScriptTranslator>(source, output, selected_role_name)
}
