#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::{identifier::Identifier},
    };

    #[test]
    fn test_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("Hello//".as_bytes());
        let output = Identifier::lex_identifier(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 5);
        assert_eq!(output.end.line, 0);
        assert_eq!(output.content, "Hello");
        assert_eq!(reader.peek(2)?.unwrap(), "//");
        

        Ok(())
    }

    #[test]
    fn test_success_line_end() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("Hello".as_bytes());
        let output = Identifier::lex_identifier(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 5);
        assert_eq!(output.end.line, 0);
        assert_eq!(output.content, "Hello");
        assert!(reader.is_done());

        Ok(())
    }

    #[test]
    fn test_failure() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("/Hello".as_bytes());
        let output = Identifier::lex_identifier(&mut reader)?;

        assert_eq!(output.is_some(), false);

        Ok(())
    }
}
