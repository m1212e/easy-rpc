#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::TokenReader,
        parser::endpoint::Endpoint,
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
            " Server someEndpointIdentifier(paramIdentifier string, paramIdentifier2 int)"
                .as_bytes(),
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
        assert_eq!(result.parameters.len(), 2);
        assert!(result.return_type.is_none());

        Ok(())
    }
}
