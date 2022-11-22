#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::operator::{Operator, OperatorType},
    };

    #[test]
    fn test_pipe() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("|".as_bytes());
        let output = Operator::lex_operator(&mut reader)?.unwrap();

        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 1);
        assert_eq!(output.range.end.line, 0);
        assert_eq!(matches!(output.operator_type, OperatorType::Pipe), true);

        Ok(())
    }

    #[test]
    fn test_curly_open_bracket() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("{".as_bytes());
        let output = Operator::lex_operator(&mut reader)?.unwrap();

        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 1);
        assert_eq!(output.range.end.line, 0);
        assert_eq!(
            matches!(output.operator_type, OperatorType::CurlyOpenBracket),
            true
        );

        Ok(())
    }

    #[test]
    fn test_curly_close_bracket() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("}".as_bytes());
        let output = Operator::lex_operator(&mut reader)?.unwrap();

        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 1);
        assert_eq!(output.range.end.line, 0);
        assert_eq!(
            matches!(output.operator_type, OperatorType::CurlyCloseBracket),
            true
        );

        Ok(())
    }

    #[test]
    fn test_open_bracket() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("(".as_bytes());
        let output = Operator::lex_operator(&mut reader)?.unwrap();

        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 1);
        assert_eq!(output.range.end.line, 0);
        assert_eq!(
            matches!(output.operator_type, OperatorType::OpenBracket),
            true
        );

        Ok(())
    }

    #[test]
    fn test_close_bracket() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new(")".as_bytes());
        let output = Operator::lex_operator(&mut reader)?.unwrap();

        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 1);
        assert_eq!(output.range.end.line, 0);
        assert_eq!(
            matches!(output.operator_type, OperatorType::CloseBracket),
            true
        );

        Ok(())
    }

    #[test]
    fn test_square_open_bracket() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("[".as_bytes());
        let output = Operator::lex_operator(&mut reader)?.unwrap();

        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 1);
        assert_eq!(output.range.end.line, 0);
        assert_eq!(
            matches!(output.operator_type, OperatorType::SquareOpenBracket),
            true
        );

        Ok(())
    }

    #[test]
    fn test_square_close_bracket() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("]".as_bytes());
        let output = Operator::lex_operator(&mut reader)?.unwrap();

        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 1);
        assert_eq!(output.range.end.line, 0);
        assert_eq!(
            matches!(output.operator_type, OperatorType::SquareCloseBracket),
            true
        );

        Ok(())
    }

    #[test]
    fn test_comma() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new(",".as_bytes());
        let output = Operator::lex_operator(&mut reader)?.unwrap();

        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 1);
        assert_eq!(output.range.end.line, 0);
        assert_eq!(matches!(output.operator_type, OperatorType::Comma), true);

        Ok(())
    }

    #[test]
    fn test_question_mark() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("?".as_bytes());
        let output = Operator::lex_operator(&mut reader)?.unwrap();

        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 1);
        assert_eq!(output.range.end.line, 0);
        assert_eq!(
            matches!(output.operator_type, OperatorType::QuestionMark),
            true
        );

        Ok(())
    }
}
