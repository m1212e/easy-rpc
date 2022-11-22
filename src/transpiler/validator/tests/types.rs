#[cfg(test)]
mod tests {
    use tower_lsp::lsp_types::{Position, Range};

    use crate::transpiler::{
        parser::parser::{
            custom_type::{CustomType, Field},
            field_type::{ArrayAmount, Custom, Type},
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
                    range: Range {
                        start: Position::default(),
                        end: Position {
                            line: 3,
                            character: 0,
                        },
                    },
                    fields: vec![],
                    identifier: "MySuperCoolType".to_string(),
                },
                CustomType {
                    documentation: None,
                    range: Range {
                        start: Position {
                            line: 4,
                            character: 0,
                        },
                        end: Position {
                            line: 7,
                            character: 0,
                        },
                    },
                    fields: vec![],
                    identifier: "MySuperCoolType".to_string(),
                },
            ],
            &vec![],
        );

        assert_eq!(result.len(), 1);

        assert_eq!(result[0].message, "Type MySuperCoolType is already defined");
        assert_eq!(result[0].range.start.character, 0);
        assert_eq!(result[0].range.start.line, 4);
        assert_eq!(result[0].range.end.character, 0);
        assert_eq!(result[0].range.end.line, 7);
    }

    #[test]
    fn test_missing_field_type() {
        let result = validate(
            &vec![],
            &vec![CustomType {
                documentation: None,
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { line: 3, character: 0 }
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
        );
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].message, "Type SomeType is unknown");
        assert_eq!(result[0].range.start.character, 0);
        assert_eq!(result[0].range.start.line, 0);
        assert_eq!(result[0].range.end.character, 0);
        assert_eq!(result[0].range.end.line, 3);
    }
}
