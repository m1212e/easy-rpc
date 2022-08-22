use std::collections::HashSet;

mod tests;

use super::{
    config::Role,
    parser::{
        parser::{custom_type::CustomType, endpoint::Endpoint, field_type::Type},
        CodePosition,
    },
};

#[derive(Debug)]
pub struct ValidationError {
    start: CodePosition,
    end: CodePosition,
    message: String,
}

/**
   Checks an .erpc source file for logical errors.
*/
pub fn validate(
    endpoints: &Vec<Endpoint>,
    custom_types: &Vec<CustomType>,
    roles: &Vec<Role>,
) -> Result<(), ValidationError> {
    // all types which are required by some field, parameter, return type, etc.
    // type, start, end
    let mut required_types: Vec<(String, CodePosition, CodePosition)> = Vec::new();

    for t in custom_types {
        for field in &t.fields {
            match &field.field_type {
                Type::Custom(val) => {
                    required_types.push((val.identifier.to_owned(), t.start, t.end))
                }
                _ => {}
            }
        }
    }

    for ep in endpoints {
        match roles.iter().find(|val| val.name == ep.role) {
            Some(_) => {}
            None => {
                return Err(ValidationError {
                    start: ep.start,
                    end: ep.end,
                    message: format!(
                    "Role {eprole} of endpoint {epidentifier} is not configured in the roles.json",
                    eprole = ep.role,
                    epidentifier = ep.identifier
                ),
                })
            }
        }

        for param in &ep.parameters {
            match &param.parameter_type {
                Type::Custom(val) => {
                    required_types.push((val.identifier.to_owned(), ep.start, ep.end))
                }
                _ => {}
            }
        }

        match &ep.return_type {
            Some(val) => match val {
                Type::Custom(cstm) => {
                    required_types.push((cstm.identifier.to_owned(), ep.start, ep.end))
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
            return Err(ValidationError {
                start: required.1,
                end: required.2,
                message: format!("Type {t} is unknown", t = required.0),
            });
        }
    }

    let mut visited_types = HashSet::<String>::new();
    for custom_type in custom_types {
        if visited_types.contains(&custom_type.identifier) {
            return Err(ValidationError {
                start: custom_type.start,
                end: custom_type.end,
                message: format!("Type {t} is already defined", t = custom_type.identifier),
            });
        }
        visited_types.insert(custom_type.identifier.to_owned());
    }

    // role, identifier
    let mut visited_endpoints = HashSet::<(&str, &str)>::new();
    for endpoint in endpoints {
        if visited_endpoints.contains(&(&endpoint.role, &endpoint.identifier)) {
            return Err(ValidationError {
                start: endpoint.start,
                end: endpoint.end,
                message: format!(
                    "Endpoint {role} {identifier} is already defined",
                    role = endpoint.role,
                    identifier = endpoint.identifier
                ),
            });
        }
        visited_endpoints.insert((&endpoint.role, &endpoint.identifier));
    }

    Ok(())
}
