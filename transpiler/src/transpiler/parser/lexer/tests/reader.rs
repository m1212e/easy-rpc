#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::{operator::OperatorType, token::Token, TokenReader},
    };

    #[test]
    fn test_peek_consume() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("|?,".as_bytes()))?;

        assert!(!reader.done);

        match &reader.peek(1).unwrap()[0] {
            Token::Operator(value) => assert!(matches!(value.operator_type, OperatorType::Pipe)),
            _ => {
                panic!("This case should never match")
            }
        }
        match &reader.peek(1).unwrap()[0] {
            Token::Operator(value) => assert!(matches!(value.operator_type, OperatorType::Pipe)),
            _ => {
                panic!("This case should never match")
            }
        }

        assert!(!reader.done);

        match &reader.consume(2).unwrap()[1] {
            Token::Operator(value) => {
                assert!(matches!(value.operator_type, OperatorType::QuestionMark))
            }
            _ => {
                panic!("This case should never match")
            }
        }
        match &reader.peek(1).unwrap()[0] {
            Token::Operator(value) => assert!(matches!(value.operator_type, OperatorType::Comma)),
            _ => {
                panic!("This case should never match")
            }
        }
        match &reader.consume(1).unwrap()[0] {
            Token::Operator(value) => assert!(matches!(value.operator_type, OperatorType::Comma)),
            _ => {
                panic!("This case should never match")
            }
        }

        assert!(reader.consume(10).is_none());
        assert!(reader.peek(10).is_none());

        assert!(reader.done);

        Ok(())
    }

    #[test]
    fn test_peek_consume_empty() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("".as_bytes()))?;

        assert!(reader.peek(1).is_none());
        assert!(reader.consume(1).is_none());

        Ok(())
    }

    #[test]
    fn test_done_after_creation() -> Result<(), InputReaderError> {
        // done is tested in peek/consume tests
        let reader = TokenReader::new(InputReader::new("".as_bytes()))?;

        assert!(reader.done);

        Ok(())
    }

    #[test]
    fn test_last_element_position() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("? Identifier ?\n ||".as_bytes()))?;

        assert_eq!(reader.last_token_range.start.character, 0);
        assert_eq!(reader.last_token_range.start.line, 0);
        assert_eq!(reader.last_token_range.end.character, 0);
        assert_eq!(reader.last_token_range.end.line, 0);
        reader.peek(2);
        assert_eq!(reader.last_token_range.start.character, 0);
        assert_eq!(reader.last_token_range.start.line, 0);
        assert_eq!(reader.last_token_range.end.character, 0);
        assert_eq!(reader.last_token_range.end.line, 0);
        reader.consume(2);
        assert_eq!(reader.last_token_range.start.character, 2);
        assert_eq!(reader.last_token_range.start.line, 0);
        assert_eq!(reader.last_token_range.end.character, 12);
        assert_eq!(reader.last_token_range.end.line, 0);
        reader.consume(3);
        assert_eq!(reader.last_token_range.start.character, 1);
        assert_eq!(reader.last_token_range.start.line, 1);
        assert_eq!(reader.last_token_range.end.character, 2);
        assert_eq!(reader.last_token_range.end.line, 1);

        Ok(())
    }
}
