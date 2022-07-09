#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::{literal::LiteralType, TokenReader},
        parser::endpoint::{ArrayAmount, Endpoint, ParameterType, PrimitiveType},
    };

    //TODO: check error cases

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

        assert_eq!(result.end.character, 32);
        assert_eq!(result.start.character, 1);
        assert_eq!(result.documentation, None);
        assert_eq!(result.identifier, "someEndpointIdentifier");
        assert_eq!(result.role, "Server");
        assert_eq!(result.parameters.len(), 0);
        assert!(result.return_type.is_none());

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

        assert_eq!(result.end.character, 107);
        assert_eq!(result.start.character, 1);
        assert_eq!(result.documentation, None);
        assert_eq!(result.identifier, "someEndpointIdentifier");
        assert_eq!(result.role, "Server");
        assert_eq!(result.parameters.len(), 3);
        assert!(result.return_type.is_none());

        assert_eq!(result.parameters[0].optional, true);
        assert_eq!(result.parameters[0].identifier, "paramIdentifier");
        match &result.parameters[0].parameter_type {
            ParameterType::Primitive(primitive) => {
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
            ParameterType::Primitive(primitive) => {
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
            ParameterType::Primitive(primitive) => {
                assert!(matches!(primitive.primitive_type, PrimitiveType::Float32));
                assert!(matches!(primitive.array_amount, ArrayAmount::NoArray));
            }
            _ => panic!("Should not match"),
        }

        Ok(())
    }

    #[test]
    fn test_enum_params_no_return_endpoint() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            " Server someEndpointIdentifier(paramIdentifier? \"hello\" | 17 | 16.8 | -155 | -5656.45 | true | false, paramIdentifier? true)"
                .as_bytes(),
        ))?;

        let result = Endpoint::parse_endpoint(&mut reader);

        assert!(result.is_some());

        let result = result.unwrap();

        assert!(result.is_ok());
        let mut result = result.unwrap();

        assert_eq!(result.parameters.len(), 2);

        let mut p1_values = match result.parameters.remove(0).parameter_type {
            ParameterType::Enum(en) => en.values,
            _ => {
                panic!("should not match")
            }
        };

        let mut p2_values = match result.parameters.remove(0).parameter_type {
            ParameterType::Enum(en) => en.values,
            _ => {
                panic!("should not match")
            }
        };

        assert_eq!(p1_values.len(), 7);
        assert_eq!(p2_values.len(), 1);

        match p1_values.remove(0).literal_type {
            LiteralType::String(value) => assert_eq!(value, "hello"),
            _ => {
                panic!("should not match")
            }
        }

        match p1_values.remove(0).literal_type {
            LiteralType::Integer(value) => assert_eq!(value, 17),
            _ => {
                panic!("should not match")
            }
        }

        match p1_values.remove(0).literal_type {
            LiteralType::Float(value) => assert_eq!(value, 16.8),
            _ => {
                panic!("should not match")
            }
        }

        match p1_values.remove(0).literal_type {
            LiteralType::Integer(value) => assert_eq!(value, -155),
            _ => {
                panic!("should not match")
            }
        }

        match p1_values.remove(0).literal_type {
            LiteralType::Float(value) => assert_eq!(value, -5656.45),
            _ => {
                panic!("should not match")
            }
        }

        match p1_values.remove(0).literal_type {
            LiteralType::Boolean(value) => assert_eq!(value, true),
            _ => {
                panic!("should not match")
            }
        }

        match p1_values.remove(0).literal_type {
            LiteralType::Boolean(value) => assert_eq!(value, false),
            _ => {
                panic!("should not match")
            }
        }

        match p2_values.remove(0).literal_type {
            LiteralType::Boolean(value) => assert_eq!(value, true),
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

        Ok(())
    }
}
