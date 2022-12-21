#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::TokenReader,
        parser::middlware::Middleware,
    };

    #[test]
    fn test_success() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("&SomeMiddlewareName".as_bytes()))?;

        let result = Middleware::parse_middleware(&mut reader).unwrap().unwrap();

        assert_eq!(result.identifier, "SomeMiddlewareName");

        Ok(())
    }

    #[test]
    fn test_success_with_space() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("&   SomeMiddlewareName".as_bytes()))?;

        let result = Middleware::parse_middleware(&mut reader).unwrap().unwrap();

        assert_eq!(result.identifier, "SomeMiddlewareName");

        Ok(())
    }

    #[test]
    fn test_invalid_with_keyword() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("&string".as_bytes()))?;

        Middleware::parse_middleware(&mut reader)
            .unwrap()
            .unwrap_err();

        Ok(())
    }

    #[test]
    fn test_invalid_with_number() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("&12".as_bytes()))?;

        Middleware::parse_middleware(&mut reader)
            .unwrap()
            .unwrap_err();

        Ok(())
    }

    #[test]
    fn test_invalid_with_linebreak() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("&
SomeValidMiddlewareName".as_bytes()))?;

        Middleware::parse_middleware(&mut reader)
            .unwrap()
            .unwrap_err();

        Ok(())
    }
}
