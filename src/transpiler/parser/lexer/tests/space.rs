#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::space::Space,
    };

    #[test]
    fn test_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("  		   ".as_bytes());

        assert_eq!(reader.get_current_position().character, 0);
        Space::skip_space(&mut reader)?;
        assert_eq!(reader.get_current_position().character, 7);

        Ok(())
    }
}
