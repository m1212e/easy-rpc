#[cfg(test)]
mod tests {
    use crate::transpiler::{parser::{parser::{custom_type::{CustomType, Field}, field_type::{Type, Primitive, ArrayAmount, PrimitiveType, Enum, Custom}}, CodePosition, lexer::literal::{Literal, LiteralType}}, generator::{typescript::TypeScriptTranslator, Translator}};


    #[test]
    fn test_success() {
        let t = CustomType {
            documentation: Some("Some sample".to_string()),
            start: CodePosition::zero_initialized(),
            end: CodePosition::zero_initialized(),
            identifier: "MyType".to_string(),
            fields: vec![
                Field {
                    documentation: Some("\nsome\ndocs\n".to_string()),
                    identifier: "field1".to_string(),
                    optional: true,
                    field_type: Type::Primitive(Primitive {
                        array_amount: ArrayAmount::NoArray,
                        primitive_type: PrimitiveType::String,
                    }),
                },
                Field {
                    documentation: None,
                    identifier: "field2".to_string(),
                    optional: false,
                    field_type: Type::Primitive(Primitive {
                        array_amount: ArrayAmount::NoLengthSpecified,
                        primitive_type: PrimitiveType::Boolean,
                    }),
                },
                Field {
                    documentation: None,
                    identifier: "field3".to_string(),
                    optional: false,
                    field_type: Type::Primitive(Primitive {
                        array_amount: ArrayAmount::LengthSpecified(17),
                        primitive_type: PrimitiveType::Int8,
                    }),
                },
                Field {
                    documentation: None,
                    identifier: "field4".to_string(),
                    optional: false,
                    field_type: Type::Primitive(Primitive {
                        array_amount: ArrayAmount::NoArray,
                        primitive_type: PrimitiveType::Int16,
                    }),
                },
                Field {
                    documentation: None,
                    identifier: "field5".to_string(),
                    optional: false,
                    field_type: Type::Primitive(Primitive {
                        array_amount: ArrayAmount::NoArray,
                        primitive_type: PrimitiveType::Int32,
                    }),
                },
                Field {
                    documentation: None,
                    identifier: "field6".to_string(),
                    optional: false,
                    field_type: Type::Primitive(Primitive {
                        array_amount: ArrayAmount::NoArray,
                        primitive_type: PrimitiveType::Int64,
                    }),
                },
                Field {
                    documentation: None,
                    identifier: "field7".to_string(),
                    optional: false,
                    field_type: Type::Primitive(Primitive {
                        array_amount: ArrayAmount::NoArray,
                        primitive_type: PrimitiveType::Float32,
                    }),
                },
                Field {
                    documentation: None,
                    identifier: "field8".to_string(),
                    optional: false,
                    field_type: Type::Primitive(Primitive {
                        array_amount: ArrayAmount::NoArray,
                        primitive_type: PrimitiveType::Float64,
                    }),
                },
                Field {
                    documentation: None,
                    identifier: "field9".to_string(),
                    optional: false,
                    field_type: Type::Enum(Enum {
                        values: vec![
                            Literal {
                                start: CodePosition::zero_initialized(),
                                end: CodePosition::zero_initialized(),
                                literal_type: LiteralType::Boolean(true),
                            },
                            Literal {
                                start: CodePosition::zero_initialized(),
                                end: CodePosition::zero_initialized(),
                                literal_type: LiteralType::Boolean(false),
                            },
                            Literal {
                                start: CodePosition::zero_initialized(),
                                end: CodePosition::zero_initialized(),
                                literal_type: LiteralType::String(
                                    "hello from the other side".to_string(),
                                ),
                            },
                            Literal {
                                start: CodePosition::zero_initialized(),
                                end: CodePosition::zero_initialized(),
                                literal_type: LiteralType::Float(123.456),
                            },
                            Literal {
                                start: CodePosition::zero_initialized(),
                                end: CodePosition::zero_initialized(),
                                literal_type: LiteralType::Integer(-123456),
                            },
                        ],
                    }),
                },
                Field {
                    documentation: None,
                    identifier: "field10".to_string(),
                    optional: false,
                    field_type: Type::Custom(Custom {
                        identifier: "MyCustomType".to_string(),
                        array_amount: ArrayAmount::NoArray,
                    }),
                },
                Field {
                    documentation: None,
                    identifier: "field11".to_string(),
                    optional: false,
                    field_type: Type::Custom(Custom {
                        identifier: "MyCustomType2".to_string(),
                        array_amount: ArrayAmount::NoLengthSpecified,
                    }),
                },
                Field {
                    documentation: None,
                    identifier: "field12".to_string(),
                    optional: false,
                    field_type: Type::Custom(Custom {
                        identifier: "MyCustomType3".to_string(),
                        array_amount: ArrayAmount::LengthSpecified(1000),
                    }),
                },
            ],
        };

        let result = TypeScriptTranslator::custom_type_to_interface(&t);

        assert_eq!(
            result,
            "/**Some sample*/
export interface MyType {
/**
some
docs
*/
    field1?: string
    field2: boolean[]
    field3: number[]
    field4: number
    field5: number
    field6: number
    field7: number
    field8: number
    field9: true | false | \"hello from the other side\" | 123.456 | -123456
    field10: MyCustomType
    field11: MyCustomType2[]
    field12: MyCustomType3[]
}
"
        )
    }
}

//TODO write some tests whith variation (no docs etc.)