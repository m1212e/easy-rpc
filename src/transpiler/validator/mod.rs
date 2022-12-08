use std::collections::HashSet;

mod tests;

use tower_lsp::lsp_types::Range;

use super::{
    config::Role,
    parser::parser::{
        custom_type::CustomType,
        endpoint::Endpoint,
        erpc_type::{EnumType, Type},
    },
};

#[derive(Debug)]
pub struct ValidationError {
    pub range: Range,
    pub message: String,
}

/**
   Checks an .erpc source file for logical errors.
*/
pub fn validate(
    endpoints: &Vec<Endpoint>,
    custom_types: &Vec<CustomType>,
    roles: &Vec<Role>,
) -> Vec<ValidationError> {
    let mut errors = vec![];

    // all types which are required by some field, parameter, return type, etc.
    // type, start, end
    let mut required_types: Vec<(String, Range)> = Vec::new();

    for t in custom_types {
        for field in &t.fields {
            match &field.field_type {
                Type::Custom(val) => required_types.push((val.identifier.to_owned(), t.range)),
                _ => {}
            }
        }
    }

    for endpoint in endpoints {
        match roles.iter().find(|val| val.name == endpoint.role) {
            Some(_) => {}
            None => errors.push(ValidationError {
                range: endpoint.range,
                message: format!(
                    "Role {eprole} of endpoint {epidentifier} is not configured in the roles.json",
                    eprole = endpoint.role,
                    epidentifier = endpoint.identifier
                ),
            }),
        }

        for param in &endpoint.parameters {
            match &param.parameter_type {
                Type::Custom(val) => {
                    required_types.push((val.identifier.to_owned(), endpoint.range))
                }
                Type::Enum(val) => {
                    for val in &val.values {
                        match val {
                            EnumType::Custom(val) => {
                                required_types.push((val.identifier.to_owned(), endpoint.range))
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        match &endpoint.return_type {
            Some(val) => match val {
                Type::Custom(cstm) => {
                    required_types.push((cstm.identifier.to_owned(), endpoint.range))
                }
                Type::Enum(val) => {
                    for val in &val.values {
                        match val {
                            EnumType::Custom(val) => {
                                required_types.push((val.identifier.to_owned(), endpoint.range))
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            None => {}
        }
    }

    for required in required_types {
        if custom_types
            .iter()
            .find(|val| val.identifier == required.0)
            .is_none()
        {
            errors.push(ValidationError {
                range: required.1,
                message: format!("Type {t} is unknown", t = required.0),
            })
        }
    }

    let mut visited_types = HashSet::<String>::new();
    for custom_type in custom_types {
        if visited_types.contains(&custom_type.identifier) {
            errors.push(ValidationError {
                range: custom_type.range,
                message: format!("Type {t} is already defined", t = custom_type.identifier),
            });
        }
        visited_types.insert(custom_type.identifier.to_owned());
    }

    // role, identifier
    let mut visited_endpoints = HashSet::<(&str, &str)>::new();
    for endpoint in endpoints {
        if visited_endpoints.contains(&(&endpoint.role, &endpoint.identifier)) {
            errors.push(ValidationError {
                range: endpoint.range,
                message: format!(
                    "Endpoint {role} {identifier} is already defined",
                    role = endpoint.role,
                    identifier = endpoint.identifier
                ),
            });
        }
        visited_endpoints.insert((&endpoint.role, &endpoint.identifier));
    }

    // checking double fields on types
    for custom_type in custom_types {
        // we only need to report doubles once, so we need to remember what we already reported
        let mut already_reported = vec![];
        for field in &custom_type.fields {
            let amount = custom_type
                .fields
                .iter()
                .filter(|f| f.identifier == field.identifier)
                .count();

            if amount > 1
                && already_reported
                    .iter()
                    .find(|r| **r == field.identifier)
                    .is_none()
            {
                errors.push(ValidationError {
                    range: custom_type.range,
                    message: format!("Field {} is defined multiple times", field.identifier),
                });

                already_reported.push(field.identifier.clone());
            }
        }
    }

    errors
}
