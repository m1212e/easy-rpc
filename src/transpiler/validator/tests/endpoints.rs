#[cfg(test)]
mod tests {
    use crate::transpiler::{
        config::Role,
        parser::{
            parser::{
                endpoint::{Endpoint, Parameter},
                field_type::{ArrayAmount, Custom, Type},
            },
            CodePosition,
        },
        validator::validate,
    };

    #[test]
    fn test_double_endpoint() {
        let result = validate(
            &vec![
                Endpoint {
                    documentation: None,
                    start: CodePosition {
                        line: 0,
                        character: 0,
                    },
                    end: CodePosition {
                        line: 0,
                        character: 30,
                    },
                    identifier: "SuperCoolEndpoint".to_string(),
                    role: "SomeRole1".to_string(),
                    return_type: None,
                    parameters: vec![],
                },
                Endpoint {
                    documentation: None,
                    start: CodePosition {
                        line: 1,
                        character: 0,
                    },
                    end: CodePosition {
                        line: 1,
                        character: 30,
                    },
                    identifier: "SuperCoolEndpoint".to_string(),
                    role: "SomeRole2".to_string(),
                    return_type: None,
                    parameters: vec![],
                },
                Endpoint {
                    documentation: None,
                    start: CodePosition {
                        line: 3,
                        character: 0,
                    },
                    end: CodePosition {
                        line: 3,
                        character: 30,
                    },
                    identifier: "SuperCoolEndpoint".to_string(),
                    role: "SomeRole1".to_string(),
                    return_type: None,
                    parameters: vec![],
                },
            ],
            &vec![],
            &vec![
                Role {
                    documentation: None,
                    name: "SomeRole1".to_string(),
                    role_type: "browser".to_string(),
                },
                Role {
                    documentation: None,
                    name: "SomeRole2".to_string(),
                    role_type: "browser".to_string(),
                },
            ],
        )
        .unwrap_err();

        assert_eq!(
            result.message,
            "Endpoint SomeRole1 SuperCoolEndpoint is already defined"
        );
        assert_eq!(result.start.character, 0);
        assert_eq!(result.start.line, 3);
        assert_eq!(result.end.character, 30);
        assert_eq!(result.end.line, 3);
    }

    #[test]
    fn test_unknown_role() {
        let result = validate(
            &vec![Endpoint {
                documentation: None,
                start: CodePosition {
                    line: 0,
                    character: 0,
                },
                end: CodePosition {
                    line: 0,
                    character: 30,
                },
                identifier: "SuperCoolEndpoint".to_string(),
                role: "SomeRole".to_string(),
                return_type: None,
                parameters: vec![],
            }],
            &vec![],
            &vec![Role {
                documentation: None,
                name: "SomeDifferentRole".to_string(),
                role_type: "browser".to_string(),
            }],
        )
        .unwrap_err();

        assert_eq!(
            result.message,
            "Role SomeRole of endpoint SuperCoolEndpoint is not configured in the roles.json"
        );
        assert_eq!(result.start.character, 0);
        assert_eq!(result.start.line, 0);
        assert_eq!(result.end.character, 30);
        assert_eq!(result.end.line, 0);
    }

    #[test]
    fn test_unknown_parameter_type() {
        let result = validate(
            &vec![Endpoint {
                documentation: None,
                start: CodePosition {
                    line: 0,
                    character: 0,
                },
                end: CodePosition {
                    line: 0,
                    character: 30,
                },
                identifier: "SuperCoolEndpoint".to_string(),
                role: "SomeRole".to_string(),
                return_type: None,
                parameters: vec![Parameter {
                    identifier: "something".to_string(),
                    optional: false,
                    parameter_type: Type::Custom(Custom {
                        array_amount: ArrayAmount::NoArray,
                        identifier: "UnknownType".to_string(),
                    }),
                }],
            }],
            &vec![],
            &vec![Role {
                documentation: None,
                name: "SomeRole".to_string(),
                role_type: "browser".to_string(),
            }],
        )
        .unwrap_err();

        assert_eq!(result.message, "Type UnknownType is unknown");
        assert_eq!(result.start.character, 0);
        assert_eq!(result.start.line, 0);
        assert_eq!(result.end.character, 30);
        assert_eq!(result.end.line, 0);
    }

    #[test]
    fn test_unknown_return_type() {
        let result = validate(
            &vec![Endpoint {
                documentation: None,
                start: CodePosition {
                    line: 0,
                    character: 0,
                },
                end: CodePosition {
                    line: 0,
                    character: 30,
                },
                identifier: "SuperCoolEndpoint".to_string(),
                role: "SomeRole".to_string(),
                return_type: Some(Type::Custom(Custom {
                    array_amount: ArrayAmount::NoArray,
                    identifier: "SomeUnknownReturnType".to_string(),
                })),
                parameters: vec![],
            }],
            &vec![],
            &vec![Role {
                documentation: None,
                name: "SomeRole".to_string(),
                role_type: "browser".to_string(),
            }],
        )
        .unwrap_err();

        assert_eq!(result.message, "Type SomeUnknownReturnType is unknown");
        assert_eq!(result.start.character, 0);
        assert_eq!(result.start.line, 0);
        assert_eq!(result.end.character, 30);
        assert_eq!(result.end.line, 0);
    }
}
