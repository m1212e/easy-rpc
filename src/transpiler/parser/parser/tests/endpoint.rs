#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::{literal::LiteralType, TokenReader},
        parser::{
            endpoint::Endpoint,
            erpc_type::{ArrayAmount, EnumType, PrimitiveType, Type},
        },
    };

    #[test]
    fn test_no_param_no_return_endpoint() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier()".as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());

        let result = result.unwrap();
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.range.end.character, 32);
        assert_eq!(result.range.start.character, 1);
        assert_eq!(result.documentation, None);
        assert_eq!(result.identifier, "someEndpointIdentifier");
        assert_eq!(result.role, "Server");
        assert_eq!(result.parameters.len(), 0);
        assert!(result.return_type.is_none());

        Ok(())
    }

    #[test]
    fn test_documentational_comment() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "/**\nThis is a documentational comment\n*/Server someEndpointIdentifier()".as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        let result = result.unwrap();
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.identifier, "someEndpointIdentifier");
        assert_eq!(result.role, "Server");
        assert_eq!(result.parameters.len(), 0);
        assert!(result.return_type.is_none());

        assert_eq!(
            result.documentation.unwrap(),
            "\nThis is a documentational comment\n"
        );

        Ok(())
    }

    #[test]
    fn test_documentational_comment_newline() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "/**\nThis is a documentational comment\n*/\nServer someEndpointIdentifier()"
                .as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        let result = result.unwrap();
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.identifier, "someEndpointIdentifier");
        assert_eq!(result.role, "Server");
        assert_eq!(result.parameters.len(), 0);
        assert!(result.return_type.is_none());

        assert_eq!(
            result.documentation.unwrap(),
            "\nThis is a documentational comment\n"
        );

        Ok(())
    }

    #[test]
    fn test_primitve_params_no_return_endpoint() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(paramIdentifier? string[], paramIdentifier2 int[12], paramIdentifier3 float)"
                .as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());

        let result = result.unwrap();

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.range.end.character, 107);
        assert_eq!(result.range.start.character, 1);
        assert_eq!(result.documentation, None);
        assert_eq!(result.identifier, "someEndpointIdentifier");
        assert_eq!(result.role, "Server");
        assert_eq!(result.parameters.len(), 3);
        assert!(result.return_type.is_none());

        assert_eq!(result.parameters[0].optional, true);
        assert_eq!(result.parameters[0].identifier, "paramIdentifier");
        match &result.parameters[0].parameter_type {
            Type::Primitive(primitive) => {
                assert!(matches!(primitive.primitive_type, PrimitiveType::String));
                assert!(matches!(
                    primitive.array_amount,
                    ArrayAmount::NoLengthSpecified
                ));
            }
            _ => panic!("Should not match"),
        }

        assert_eq!(result.parameters[1].optional, false);
        assert_eq!(result.parameters[1].identifier, "paramIdentifier2");
        match &result.parameters[1].parameter_type {
            Type::Primitive(primitive) => {
                assert!(matches!(primitive.primitive_type, PrimitiveType::Int16));
                assert!(matches!(
                    primitive.array_amount,
                    ArrayAmount::LengthSpecified(12)
                ));
            }
            _ => panic!("Should not match"),
        }

        assert_eq!(result.parameters[2].optional, false);
        assert_eq!(result.parameters[2].identifier, "paramIdentifier3");
        match &result.parameters[2].parameter_type {
            Type::Primitive(primitive) => {
                assert!(matches!(primitive.primitive_type, PrimitiveType::Float32));
                assert!(matches!(primitive.array_amount, ArrayAmount::NoArray));
            }
            _ => panic!("Should not match"),
        }

        Ok(())
    }

    #[test]
    fn test_all_primitive_types() -> Result<(), InputReaderError> {
        Endpoint::parse_endpoint(&mut TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(p1 string)".as_bytes(),
        ))?)
        .unwrap()
        .unwrap();

        Endpoint::parse_endpoint(&mut TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(p1 boolean)".as_bytes(),
        ))?)
        .unwrap()
        .unwrap();

        Endpoint::parse_endpoint(&mut TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(p1 int)".as_bytes(),
        ))?)
        .unwrap()
        .unwrap();

        Endpoint::parse_endpoint(&mut TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(p1 int8)".as_bytes(),
        ))?)
        .unwrap()
        .unwrap();

        Endpoint::parse_endpoint(&mut TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(p1 int16)".as_bytes(),
        ))?)
        .unwrap()
        .unwrap();

        Endpoint::parse_endpoint(&mut TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(p1 int32)".as_bytes(),
        ))?)
        .unwrap()
        .unwrap();

        Endpoint::parse_endpoint(&mut TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(p1 int64)".as_bytes(),
        ))?)
        .unwrap()
        .unwrap();

        Endpoint::parse_endpoint(&mut TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(p1 float)".as_bytes(),
        ))?)
        .unwrap()
        .unwrap();

        Endpoint::parse_endpoint(&mut TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(p1 float32)".as_bytes(),
        ))?)
        .unwrap()
        .unwrap();

        Endpoint::parse_endpoint(&mut TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(p1 float64)".as_bytes(),
        ))?)
        .unwrap()
        .unwrap();

        Ok(())
    }

    #[test]
    fn test_enum_params_no_return_endpoint() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(paramIdentifier? \"hello\" | 17 | 16.8 | -155 | -5656.45 | true | false | CustomTypeTest | string)"
                .as_bytes(),
        ))?;

        let mut result = Endpoint::parse_endpoint(&mut reader).unwrap().unwrap();
        assert_eq!(result.parameters.len(), 1);

        let mut parameter_enum_values = match result.parameters.remove(0).parameter_type {
            Type::Enum(en) => en.values,
            _ => {
                panic!("should not match")
            }
        };
        assert_eq!(parameter_enum_values.len(), 9);

        match parameter_enum_values.remove(0) {
            EnumType::Literal(value) => match value {
                LiteralType::String(value) => assert_eq!(value, "hello"),
                _ => {
                    panic!("should not match")
                }
            },
            _ => {
                panic!("should not match")
            }
        }

        match parameter_enum_values.remove(0) {
            EnumType::Literal(value) => match value {
                LiteralType::Integer(value) => assert_eq!(value, 17),
                _ => {
                    panic!("should not match")
                }
            },
            _ => {
                panic!("should not match")
            }
        }

        match parameter_enum_values.remove(0) {
            EnumType::Literal(value) => match value {
                LiteralType::Float(value) => assert_eq!(value, 16.8),
                _ => {
                    panic!("should not match")
                }
            },
            _ => {
                panic!("should not match")
            }
        }

        match parameter_enum_values.remove(0) {
            EnumType::Literal(value) => match value {
                LiteralType::Integer(value) => assert_eq!(value, -155),
                _ => {
                    panic!("should not match")
                }
            },
            _ => {
                panic!("should not match")
            }
        }

        match parameter_enum_values.remove(0) {
            EnumType::Literal(value) => match value {
                LiteralType::Float(value) => assert_eq!(value, -5656.45),
                _ => {
                    panic!("should not match")
                }
            },
            _ => {
                panic!("should not match")
            }
        }

        match parameter_enum_values.remove(0) {
            EnumType::Literal(value) => match value {
                LiteralType::Boolean(value) => assert_eq!(value, true),
                _ => {
                    panic!("should not match")
                }
            },
            _ => {
                panic!("should not match")
            }
        }

        match parameter_enum_values.remove(0) {
            EnumType::Literal(value) => match value {
                LiteralType::Boolean(value) => assert_eq!(value, false),
                _ => {
                    panic!("should not match")
                }
            },
            _ => {
                panic!("should not match")
            }
        }

        match parameter_enum_values.remove(0) {
            EnumType::Custom(value) => {
                assert_eq!(value.identifier, "CustomTypeTest")
            }
            _ => {
                panic!("should not match")
            }
        }

        match parameter_enum_values.remove(0) {
            EnumType::Primitive(value) => match value.primitive_type {
                PrimitiveType::String => {}
                _ => panic!("should not match"),
            },
            _ => {
                panic!("should not match")
            }
        }

        Ok(())
    }

    #[test]
    fn test_custom_params_no_return_endpoint() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(paramIdentifier? CustomType, paramIdentifier2 CustomType2[], paramIdentifier3 CustomType3[10])"
                .as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());

        let result = result.unwrap();

        let mut result = result.unwrap();

        assert_eq!(result.parameters.len(), 3);

        match result.parameters.remove(0).parameter_type {
            Type::Custom(value) => {
                assert_eq!(value.identifier, "CustomType");
                assert!(matches!(value.array_amount, ArrayAmount::NoArray));
            }
            _ => {
                panic!("should not match")
            }
        }

        match result.parameters.remove(0).parameter_type {
            Type::Custom(value) => {
                assert_eq!(value.identifier, "CustomType2");
                assert!(matches!(value.array_amount, ArrayAmount::NoLengthSpecified));
            }
            _ => {
                panic!("should not match")
            }
        }

        match result.parameters.remove(0).parameter_type {
            Type::Custom(value) => {
                assert_eq!(value.identifier, "CustomType3");
                assert!(matches!(
                    value.array_amount,
                    ArrayAmount::LengthSpecified(10)
                ));
            }
            _ => {
                panic!("should not match")
            }
        }

        Ok(())
    }

    #[test]
    fn test_primitive_return_value() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier() string".as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());

        let result = result.unwrap();

        let ret_type = result.unwrap().return_type;

        match ret_type.unwrap() {
            Type::Primitive(value) => match value.primitive_type {
                PrimitiveType::String => {
                    assert!(matches!(value.array_amount, ArrayAmount::NoArray));
                }
                _ => {
                    panic!("Should not match")
                }
            },
            _ => {
                panic!("Should not match")
            }
        }

        Ok(())
    }

    #[test]
    fn test_primitive_array_return_value() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier() string[]".as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());

        let result = result.unwrap();

        let ret_type = result.unwrap().return_type;

        match ret_type.unwrap() {
            Type::Primitive(value) => match value.primitive_type {
                PrimitiveType::String => {
                    assert!(matches!(value.array_amount, ArrayAmount::NoLengthSpecified));
                }
                _ => {
                    panic!("Should not match")
                }
            },
            _ => {
                panic!("Should not match")
            }
        }

        Ok(())
    }

    #[test]
    fn test_primitive_array_return_value_2() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier() string[100]".as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());

        let result = result.unwrap();

        let ret_type = result.unwrap().return_type;

        match ret_type.unwrap() {
            Type::Primitive(value) => match value.primitive_type {
                PrimitiveType::String => {
                    assert!(matches!(
                        value.array_amount,
                        ArrayAmount::LengthSpecified(100)
                    ));
                }
                _ => {
                    panic!("Should not match")
                }
            },
            _ => {
                panic!("Should not match")
            }
        }

        Ok(())
    }

    #[test]
    fn test_invalid() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            " true someEndpointIdentifier() string[100]".as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_invalid_2() -> Result<(), InputReaderError> {
        let mut reader =
            TokenReader::new(InputReader::new(" Server true() string[100]".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_invalid_3() -> Result<(), InputReaderError> {
        let mut reader =
            TokenReader::new(InputReader::new(" Server true|) string[100]".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_invalid_4() -> Result<(), InputReaderError> {
        let mut reader =
            TokenReader::new(InputReader::new(" Server trueA) string[100]".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_invalid_5() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(" Server something(".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_invalid_6() -> Result<(), InputReaderError> {
        let mut reader =
            TokenReader::new(InputReader::new(" Server hello(p1 string,)".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Expected parameters instead of closing bracket"
            );
        }

        Ok(())
    }

    #[test]
    fn test_newline_after_parameter() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            " Server hello(p1 string,\np2 string)".as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_invalid_7() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(" Server hello(p1".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Not enough tokens to form a valid parameter"
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_8() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(" Server hello(true true".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Expected parameter identifier"
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_9() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(" Server hello(p1| string".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Unexpected operator. Only ? is valid here."
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_10() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(" Server hello(p1 .".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Expected a parameter type"
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_11() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(" Server hello(p1 true |".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Expected a literal for this enum type"
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_12() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            " Server hello(p1 true | something".as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Expected more tokens for correct endpoint syntax"
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_13() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            " Server hello(p1 string[invalid]".as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Expected integer or closing bracket"
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_14() -> Result<(), InputReaderError> {
        let mut reader =
            TokenReader::new(InputReader::new(" Server hello(p1 string[15.5]".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Expected integer or closing bracket"
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_15() -> Result<(), InputReaderError> {
        let mut reader =
            TokenReader::new(InputReader::new(" Server hello(p1 string[true".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Expected integer or closing bracket"
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_16() -> Result<(), InputReaderError> {
        let mut reader =
            TokenReader::new(InputReader::new(" Server hello(p1 string[0])".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Size of the array must be above or equal to 1"
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_17() -> Result<(), InputReaderError> {
        let mut reader =
            TokenReader::new(InputReader::new(" Server hello(p1 string[-18])".as_bytes()))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Size of the array must be above or equal to 1"
            );
        }

        Ok(())
    }

    #[test]
    fn test_middleware_endpoint() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "&SomeValidMiddlwareIdentifier
&AnotherValidMiddlwareIdentifier
 Server someEndpointIdentifier()"
                .as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader).unwrap().unwrap();

        assert_eq!(result.range.start.character, 0);
        assert_eq!(result.range.start.line, 0);
        assert_eq!(result.range.end.character, 32);
        assert_eq!(result.range.end.line, 2);
        assert_eq!(result.documentation, None);
        assert_eq!(
            result.middleware_identifiers,
            vec![
                "SomeValidMiddlwareIdentifier",
                "AnotherValidMiddlwareIdentifier"
            ]
        );
        assert_eq!(result.identifier, "someEndpointIdentifier");
        assert_eq!(result.role, "Server");
        assert_eq!(result.parameters.len(), 0);
        assert!(result.return_type.is_none());

        Ok(())
    }

    #[test]
    fn test_middleware_endpoint_with_docs_spaces() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "/**These are some docs*/
&SomeValidMiddlwareIdentifier
&AnotherValidMiddlwareIdentifier

 Server someEndpointIdentifier()"
                .as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader).unwrap().unwrap();

        assert_eq!(result.range.start.character, 0);
        assert_eq!(result.range.start.line, 0);
        assert_eq!(result.range.end.character, 32);
        assert_eq!(result.range.end.line, 4);
        assert_eq!(result.documentation, Some("These are some docs".to_string()));
        assert_eq!(
            result.middleware_identifiers,
            vec![
                "SomeValidMiddlwareIdentifier",
                "AnotherValidMiddlwareIdentifier"
            ]
        );
        assert_eq!(result.identifier, "someEndpointIdentifier");
        assert_eq!(result.role, "Server");
        assert_eq!(result.parameters.len(), 0);
        assert!(result.return_type.is_none());

        Ok(())
    }
}
