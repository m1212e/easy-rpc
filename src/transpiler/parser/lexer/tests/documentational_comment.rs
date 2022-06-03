#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::documentational_comment::DocumentationalComment,
    };

    #[test]
    fn test_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("/**\nSome documentational comment\n*/".as_bytes());
        let output = DocumentationalComment::lex_documentational_comment(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 2);
        assert_eq!(output.get_end().line, 2);
        assert_eq!(output.get_content(), "\nSome documentational comment\n");

        Ok(())
    }
}
