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
        assert!(reader.consume(10).is_none());

        assert!(reader.is_done());
        assert!(reader.done);

        Ok(())
    }

    #[test]
    fn test_consume_until() -> Result<(), InputReaderError> {
        let mut reader = TokenReader::new(InputReader::new("|?,|||".as_bytes()))?;

        let res = reader
            .consume_until(|current, total| {
                match current {
                    Token::Operator(v) => {
                        if matches!(v.get_type(), OperatorType::QuestionMark) {
                            assert_eq!(total.len(), 2);
                            return false;
                        }
                    }
                    _ => {
                        panic!("Should never be called")
                    }
                }
                return true;
            })
            .unwrap();

        assert_eq!(res.len(), 2);

        match &res[0] {
            Token::Operator(value) => assert!(matches!(value.get_type(), OperatorType::Pipe)),
            _ => {
                panic!("This case should never match")
            }
        }

        match &res[1] {
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

        let res = reader
            .consume_until(|_, _| {
                return true;
            })
            .unwrap();

        assert_eq!(res.len(), 6);

        Ok(())
    }
}
