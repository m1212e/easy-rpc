#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::{line_break::LineBreak}, CodeArea,
    };

    #[test]
    fn test_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("\n\r\n".as_bytes());

        assert_eq!(reader.get_current_position().character, 0);
        assert_eq!(reader.get_current_position().line, 0);
        let output = LineBreak::lex_line_break(&mut reader)?.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 0);
        assert_eq!(output.get_end().line, 1);
        let output = LineBreak::lex_line_break(&mut reader)?.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 1);
        assert_eq!(output.get_end().character, 0);
        assert_eq!(output.get_end().line, 2);
        assert!(reader.is_done());

        Ok(())
    }
}
