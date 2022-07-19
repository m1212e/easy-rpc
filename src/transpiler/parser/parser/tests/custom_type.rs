#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::TokenReader,
        parser::{
            custom_type::{CustomType, Field},
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
    fn test_success_one_field() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "type EmptyType {\nfield1 string}".as_bytes(),
        ))?;

        let mut result = CustomType::parse_custom_type(&mut reader).unwrap().unwrap();

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
            "type EmptyType {\nfield1 string\n field2 int}".as_bytes(),
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
        assert_eq!(f1.optional, false);
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
        assert_eq!(f2.optional, false);
        assert!(f2.documentation.is_none());

        Ok(())
    }

    #[test]
    fn test_success_three_fields() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new(
            "type EmptyType {\nfield1 string\n field2 SomeCustomType\nfield3 \"Something\" | true | 17}".as_bytes(),
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
}
