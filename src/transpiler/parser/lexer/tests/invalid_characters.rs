#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::invalid_characters::InvalidCharacters,
    };

    #[test]
    fn test_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("+##/".as_bytes());
        let output = InvalidCharacters::lex_next_as_invalid_character(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
    assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 1);
        assert_eq!(output.end.line, 0);
        assert_eq!(output.content, "+");

        let output = InvalidCharacters::lex_next_as_invalid_character(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 1);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 2);
        assert_eq!(output.end.line, 0);
        assert_eq!(output.content, "#");

        let output = InvalidCharacters::lex_next_as_invalid_character(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 2);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 3);
        assert_eq!(output.end.line, 0);
        assert_eq!(output.content, "#");

        let output = InvalidCharacters::lex_next_as_invalid_character(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 3);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 4);
        assert_eq!(output.end.line, 0);
        assert_eq!(output.content, "/");

        Ok(())
    }
}
