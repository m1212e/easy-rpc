#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::TokenReader,
        parser::{
            custom_type::CustomType,
            field_type::{PrimitiveType, Type},
        },
    };

    #[test]
    fn test_success_empty() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("type EmptyType {}".as_bytes()))?;

        let result = CustomType::parse_custom_type(&mut reader).unwrap().unwrap();

        assert_eq!(result.fields.len(), 0);

        Ok(())
    }

    #[test]
    fn test_docs() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("/**\nsome docs\n*/\ntype EmptyType {\n/**\nDocs for f1\n*/\nf1 string\n/**more docs*/field2 int\n}".as_bytes()))?;

        let mut result = CustomType::parse_custom_type(&mut reader).unwrap().unwrap();

        assert_eq!(result.documentation.unwrap(), "\nsome docs\n");
        assert_eq!(
            result.fields.remove(0).documentation.unwrap(),
            "\nDocs for f1\n"
        );
        assert_eq!(result.fields.remove(0).documentation.unwrap(), "more docs");

        Ok(())
    }

    #[test]
    fn test_success_one_field() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "type SomeType {\nfield1 string}".as_bytes(),
        ))?;

        let mut result = CustomType::parse_custom_type(&mut reader).unwrap().unwrap();

        assert_eq!(result.identifier, "SomeType");

        assert_eq!(result.fields.len(), 1);
        match result.fields.remove(0).field_type {
            Type::Primitive(primitive) => match primitive.primitive_type {
                PrimitiveType::String => {}
                _ => panic!("Should not match"),
            },
            _ => panic!("Should not match"),
        }

        Ok(())
    }

    #[test]
    fn test_success_two_fields() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "type SomeType {\nfield1? string\n field2 int}".as_bytes(),
        ))?;

        let mut result = CustomType::parse_custom_type(&mut reader).unwrap().unwrap();

        assert_eq!(result.fields.len(), 2);
        let f1 = result.fields.remove(0);
        match f1.field_type {
            Type::Primitive(primitive) => match primitive.primitive_type {
                PrimitiveType::String => {}
                _ => panic!("Should not match"),
            },
            _ => panic!("Should not match"),
        }
        assert_eq!(f1.identifier, "field1");
        assert!(f1.optional);
        assert!(f1.documentation.is_none());

        let f2 = result.fields.remove(0);
        match f2.field_type {
            Type::Primitive(primitive) => match primitive.primitive_type {
                PrimitiveType::Int16 => {}
                _ => panic!("Should not match"),
            },
            _ => panic!("Should not match"),
        }
        assert_eq!(f2.identifier, "field2");
        assert!(!f2.optional);
        assert!(f2.documentation.is_none());

        Ok(())
    }

    #[test]
    fn test_success_three_fields() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "type SomeType {\nfield1 string\n field2 SomeCustomType\nfield3 \"Something\" | true | 17}".as_bytes(),
        ))?;

        let mut result = CustomType::parse_custom_type(&mut reader).unwrap().unwrap();

        assert_eq!(result.fields.len(), 3);
        let f1 = result.fields.remove(0);
        match f1.field_type {
            Type::Primitive(primitive) => match primitive.primitive_type {
                PrimitiveType::String => {}
                _ => panic!("Should not match"),
            },
            _ => panic!("Should not match"),
        }
        assert_eq!(f1.identifier, "field1");
        assert_eq!(f1.optional, false);
        assert!(f1.documentation.is_none());

        let f2 = result.fields.remove(0);
        match f2.field_type {
            Type::Custom(custom) => {
                assert_eq!(custom.identifier, "SomeCustomType");
            }
            _ => panic!("Should not match"),
        }
        assert_eq!(f2.identifier, "field2");
        assert_eq!(f2.optional, false);
        assert!(f2.documentation.is_none());

        let f3 = result.fields.remove(0);
        match f3.field_type {
            Type::Enum(en) => {
                assert_eq!(en.values.len(), 3);
            }
            _ => panic!("Should not match"),
        }
        assert_eq!(f3.identifier, "field3");
        assert_eq!(f3.optional, false);
        assert!(f3.documentation.is_none());

        Ok(())
    }

    #[test]
    fn test_invalid_1() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "typ SomeType {\nfield1 string\n field2 SomeCustomType\nfield3".as_bytes(),
        ))?;

        let result = CustomType::parse_custom_type(&mut reader);

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_invalid_2() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "import SomeType {\nfield1 string\n field2 SomeCustomType\nfield3".as_bytes(),
        ))?;

        let result = CustomType::parse_custom_type(&mut reader);

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_invalid_3() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "type type {\nfield1 string\n field2 SomeCustomType\nfield3".as_bytes(),
        ))?;

        let result = CustomType::parse_custom_type(&mut reader).unwrap();

        assert!(result.is_err());

        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Expected type identifier"
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_4() -> Result<(), InputReaderError> {
        let mut reader =
            TokenReader::new(InputReader::new("/**hello*/\ntype SomeType |".as_bytes()))?;

        let result = CustomType::parse_custom_type(&mut reader).unwrap();

        assert!(result.is_err());

        unsafe {
            assert_eq!(result.unwrap_err_unchecked().message, "Expected {");
        }

        Ok(())
    }

    #[test]
    fn test_invalid_5() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "/**hello*/\ntype SomeType type".as_bytes(),
        ))?;

        let result = CustomType::parse_custom_type(&mut reader).unwrap();

        assert!(result.is_err());

        unsafe {
            assert_eq!(result.unwrap_err_unchecked().message, "Expected {");
        }

        Ok(())
    }

    #[test]
    fn test_invalid_6() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "/**hello*/type SomeType { /**hello*/\n".as_bytes(),
        ))?;

        let result = CustomType::parse_custom_type(&mut reader).unwrap();

        assert!(result.is_err());

        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Expected field identifier"
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_7() -> Result<(), InputReaderError> {
        let mut reader =
            TokenReader::new(InputReader::new("/**hello*/\ntype SomeType ".as_bytes()))?;

        let result = CustomType::parse_custom_type(&mut reader).unwrap();

        assert!(result.is_err());

        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Expected an opening { for the type"
            );
        }

        Ok(())
    }

    #[test]
    fn test_invalid_8() -> Result<(), InputReaderError> {
        let mut reader =
            TokenReader::new(InputReader::new("/**hello*/\ntype SomeType {".as_bytes()))?;

        let result = CustomType::parse_custom_type(&mut reader).unwrap();

        assert!(result.is_err());

        unsafe {
            assert_eq!(result.unwrap_err_unchecked().message, "Expected closing }");
        }

        Ok(())
    }

    #[test]
    fn test_invalid_9() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "/**hello*/\ntype SomeType {field string[hello]}".as_bytes(),
        ))?;

        let result = CustomType::parse_custom_type(&mut reader).unwrap();

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
    fn test_invalid_10() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "/**hello*/\ntype SomeType {field string[hello|}".as_bytes(),
        ))?;

        let result = CustomType::parse_custom_type(&mut reader).unwrap();

        assert!(result.is_err());

        unsafe {
            assert_eq!(
                result.unwrap_err_unchecked().message,
                "Expected integer or closing bracket"
            );
        }

        Ok(())
    }
}
