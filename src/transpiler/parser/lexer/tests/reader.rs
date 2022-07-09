#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::{
            operator::{Operator, OperatorType},
            token::Token,
            TokenReader,
        },
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
    fn test_consume_until() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("|?,|||".as_bytes()))?;

        let mut tokens: Vec<Token> = Vec::new();
        reader.consume_until(|current| match current {
            Token::Operator(v) => match v.operator_type {
                OperatorType::QuestionMark => {
                    tokens.push(Token::Operator(v));
                    false
                }
                _ => {
                    tokens.push(Token::Operator(v));
                    true
                }
            },
            _ => {
                panic!("Should never be called")
            }
        });

        assert_eq!(tokens.len(), 2);

        match &tokens[0] {
            Token::Operator(value) => assert!(matches!(value.operator_type, OperatorType::Pipe)),
            _ => {
                panic!("This case should never match")
            }
        }

        match &tokens[1] {
            Token::Operator(value) => {
                assert!(matches!(value.operator_type, OperatorType::QuestionMark))
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

        assert!(reader.done);

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
    fn test_done_after_creation() -> Result<(), InputReaderError> {
        // done is tested in peek/consume tests
        let reader = TokenReader::new(InputReader::new("".as_bytes()))?;

        assert!(reader.done);

        Ok(())
    }

    #[test]
    fn test_last_element_position() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("? Identifier ?\n ||".as_bytes()))?;
        
        assert_eq!(reader.last_token_code_start.character, 0);
        assert_eq!(reader.last_token_code_start.line, 0);
        assert_eq!(reader.last_token_code_end.character, 0);
        assert_eq!(reader.last_token_code_end.line, 0);
        reader.peek(2);
        assert_eq!(reader.last_token_code_start.character, 0);
        assert_eq!(reader.last_token_code_start.line, 0);
        assert_eq!(reader.last_token_code_end.character, 0);
        assert_eq!(reader.last_token_code_end.line, 0);
        reader.consume(2);
        assert_eq!(reader.last_token_code_start.character, 2);
        assert_eq!(reader.last_token_code_start.line, 0);
        assert_eq!(reader.last_token_code_end.character, 12);
        assert_eq!(reader.last_token_code_end.line, 0);
        reader.consume(3);
        assert_eq!(reader.last_token_code_start.character, 1);
        assert_eq!(reader.last_token_code_start.line, 1);
        assert_eq!(reader.last_token_code_end.character, 2);
        assert_eq!(reader.last_token_code_end.line, 1);

        Ok(())
    }

}
