#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::TokenReader,
        parser::parse,
    };

    #[test]
    fn test_success() -> Result<(), InputReaderError> {
        let mut r = TokenReader::new(InputReader::new(
            "type TestType {
    field1 string
    field2 int
}

Server test()
Client tes2t()"
                .as_bytes(),
        ))?;
        let result = parse(&mut r).unwrap();

        assert_eq!(result.custom_types.len(), 1);
        assert_eq!(result.endpoints.len(), 2);

        Ok(())
    }
}
