#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::line_break::LineBreak,
    };

    #[test]
    fn test_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("\n\r\n\n\nabc".as_bytes());

        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 0);
        let output = LineBreak::lex_line_break(&mut reader)?.unwrap();
        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 0);
        assert_eq!(output.range.end.line, 4);
        assert!(LineBreak::lex_line_break(&mut reader)?.is_none());

        Ok(())
    }
}
