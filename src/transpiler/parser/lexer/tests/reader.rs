#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::{operator::OperatorType, token::Token, TokenReader},
    };

    #[test]
    fn test_peek_consume() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("|?,".as_bytes()))?;

        assert!(!reader.is_done());
        assert!(!reader.done);

        match &reader.peek(1).unwrap()[0] {
            Token::Operator(value) => assert!(matches!(value.get_type(), OperatorType::Pipe)),
            _ => {
                panic!("This case should never match")
            }
        }
        match &reader.peek(1).unwrap()[0] {
            Token::Operator(value) => assert!(matches!(value.get_type(), OperatorType::Pipe)),
            _ => {
                panic!("This case should never match")
            }
        }

        assert!(!reader.is_done());
        assert!(!reader.done);

        match &reader.consume(2).unwrap()[1] {
            Token::Operator(value) => {
                assert!(matches!(value.get_type(), OperatorType::QuestionMark))
            }
            _ => {
                panic!("This case should never match")
            }
        }
        match &reader.peek(1).unwrap()[0] {
            Token::Operator(value) => assert!(matches!(value.get_type(), OperatorType::Comma)),
            _ => {
                panic!("This case should never match")
            }
        }
        match &reader.consume(1).unwrap()[0] {
            Token::Operator(value) => assert!(matches!(value.get_type(), OperatorType::Comma)),
            _ => {
                panic!("This case should never match")
            }
        }

        assert!(reader.consume(10).is_none());
        assert!(reader.peek(10).is_none());

        assert!(reader.is_done());
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
    fn test_consume_until() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("|?,|||".as_bytes()))?;

        let tokens = Vec::new();
        reader.consume_until(|current| {
            tokens.push(current);
            match current {
                Token::Operator(v) => {
                    if matches!(v.get_type(), OperatorType::QuestionMark) {
                        return false;
                    }
                }
                _ => {
                    panic!("Should never be called")
                }
            }
            return true;
        });

        assert_eq!(tokens.len(), 2);

        match &tokens[0] {
            Token::Operator(value) => assert!(matches!(value.get_type(), OperatorType::Pipe)),
            _ => {
                panic!("This case should never match")
            }
        }

        match &tokens[1] {
            Token::Operator(value) => {
                assert!(matches!(value.get_type(), OperatorType::QuestionMark))
            }
            _ => {
                panic!("This case should never match")
            }
        }

        Ok(())
    }

    #[test]
    fn test_consume_until_til_end() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("|?,|||".as_bytes()))?;

        reader.consume_until(|_| {
            return true;
        });

        assert!(reader.is_done());

        Ok(())
    }

    #[test]
    fn test_consume_until_empty() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("".as_bytes()))?;
        let mut called = false;
        reader.consume_until(|_| {
            called = true;
            return true;
        });

        assert!(!called);

        Ok(())
    }

    #[test]
    fn test_is_done() -> Result<(), InputReaderError> {
        let reader = TokenReader::new(InputReader::new("".as_bytes()))?;

        assert!(reader.is_done());

        Ok(())
    }

    #[test]
    fn test_last_element_position_start() -> Result<(), InputReaderError> {
        todo!()
    }

    #[test]
    fn test_last_element_position_end() -> Result<(), InputReaderError> {
        todo!()
    }
}
