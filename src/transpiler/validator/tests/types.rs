#[cfg(test)]
mod tests {
    use crate::transpiler::{
        parser::{
            parser::{
                custom_type::{CustomType, Field},
                field_type::{ArrayAmount, Custom, Type},
            },
            CodePosition,
        },
        validator::validate,
    };

    #[test]
    fn test_double_type() {
        let result = validate(
            &vec![],
            &vec![
                CustomType {
                    documentation: None,
                    start: CodePosition {
                        character: 0,
                        line: 0,
                    },
                    end: CodePosition {
                        character: 0,
                        line: 3,
                    },
                    fields: vec![],
                    identifier: "MySuperCoolType".to_string(),
                },
                CustomType {
                    documentation: None,
                    start: CodePosition {
                        character: 0,
                        line: 4,
                    },
                    end: CodePosition {
                        character: 0,
                        line: 7,
                    },
                    fields: vec![],
                    identifier: "MySuperCoolType".to_string(),
                },
            ],
            &vec![],
        )
        .unwrap_err();

        assert_eq!(result.message, "Type MySuperCoolType is already defined");
        assert_eq!(result.start.character, 0);
        assert_eq!(result.start.line, 4);
        assert_eq!(result.end.character, 0);
        assert_eq!(result.end.line, 7);
    }

    #[test]
    fn test_missing_field_type() {
        let result = validate(
            &vec![],
            &vec![CustomType {
                documentation: None,
                start: CodePosition {
                    character: 0,
                    line: 0,
                },
                end: CodePosition {
                    character: 0,
                    line: 3,
                },
                fields: vec![Field {
                    documentation: None,
                    identifier: "field1".to_string(),
                    optional: false,
                    field_type: Type::Custom(Custom {
                        array_amount: ArrayAmount::NoArray,
                        identifier: "SomeType".to_string(),
                    }),
                }],
                identifier: "MySuperCoolType".to_string(),
            }],
            &vec![],
        )
        .unwrap_err();

        assert_eq!(result.message, "Type SomeType is unknown");
        assert_eq!(result.start.character, 0);
        assert_eq!(result.start.line, 0);
        assert_eq!(result.end.character, 0);
        assert_eq!(result.end.line, 3);
    }
}
