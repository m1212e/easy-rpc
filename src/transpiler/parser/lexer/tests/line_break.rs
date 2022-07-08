#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::line_break::LineBreak,
    };

    #[test]
    fn test_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("\n\r\n".as_bytes());

        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 0);
        let output = LineBreak::lex_line_break(&mut reader)?.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 0);
        assert_eq!(output.end.line, 1);
        let output = LineBreak::lex_line_break(&mut reader)?.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 1);
        assert_eq!(output.end.character, 0);
        assert_eq!(output.end.line, 2);
        assert!(reader.is_done());

        Ok(())
    }
}
