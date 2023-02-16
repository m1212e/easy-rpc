pub mod config;
mod generator;
mod parser;
mod tests;

pub mod validator;

use std::{fs::File, path::Path};

use crate::error::{Diagnostic, DisplayableError};

use self::{
    config::parse_roles,
    generator::{generate_for_directory, translator::typescript::TypeScriptTranslator},
    parser::{
        input_reader::InputReader,
        lexer::TokenReader,
        parser::{endpoint::Endpoint, parse},
    },
    validator::validate,
};

pub async fn run(
    source_directory: &Path,
    output: &Path,
    selected_role_name: &str,
) -> Vec<DisplayableError> {
    // --- Config ---
    let roles_json_path = source_directory.join("roles.json");
    if !roles_json_path.exists() {
        return vec![format!(
            "Could not find roles.json at {path_str}",
            path_str = roles_json_path
                .as_os_str()
                .to_str()
                .unwrap_or("<Unable to unwrap path>")
        )
        .into()];
    }

    let available_roles = match parse_roles(match File::open(roles_json_path.clone()) {
        Ok(v) => v,
        Err(err) => {
            return vec![format!(
                "Could not open {path_str}: {err}",
                path_str = roles_json_path
                    .to_str()
                    .unwrap_or("<Unable to unwrap path>")
            )
            .into()];
        }
    }) {
        Ok(v) => v,
        Err(err) => {
            return vec![format!(
                "Could not parse roles at {path_str}: {err}",
                path_str = roles_json_path
                    .as_os_str()
                    .to_str()
                    .unwrap_or("<Unable to unwrap path>")
            )
            .into()];
        }
    };

    // --- Middleware ---

    let mut available_middleware = Vec::<Endpoint>::new();
    let middleware_erpc_path = source_directory.join("middleware.erpc");
    if middleware_erpc_path.exists() {
        let mut reader =
            match TokenReader::new(InputReader::new(match File::open(&middleware_erpc_path) {
                Ok(v) => v,
                Err(err) => {
                    return vec![format!(
                        "Could not open file {}: {err}",
                        middleware_erpc_path
                            .to_str()
                            .unwrap_or("<could not unwrap path>")
                    )
                    .into()]
                }
            })) {
                Ok(v) => v,
                Err(err) => {
                    return vec![format!(
                        "Input reader error occurred at {}: {err}",
                        middleware_erpc_path
                            .to_str()
                            .unwrap_or("<could not unwrap path>")
                    )
                    .into()];
                }
            };

        let parse_result = match parse(&mut reader) {
            Ok(val) => val,
            Err(err) => {
                return vec![DisplayableError::Diagnostic(Diagnostic {
                    source: middleware_erpc_path,
                    range: err.range,
                    message: err.message,
                })];
            }
        };

        let validation_errors = validate(
            &parse_result.endpoints,
            &parse_result.custom_types,
            &available_roles,
            &vec![],
        );
        if !validation_errors.is_empty() {
            return validation_errors
                .into_iter()
                .map(|err| {
                    DisplayableError::Diagnostic(Diagnostic {
                        source: middleware_erpc_path.clone(),
                        range: err.range,
                        message: err.message,
                    })
                })
                .collect();
        }

        available_middleware = parse_result.endpoints;
    }

    generate_for_directory::<TypeScriptTranslator>(
        source_directory,
        output,
        selected_role_name,
        &available_roles,
        &available_middleware,
    )
}
