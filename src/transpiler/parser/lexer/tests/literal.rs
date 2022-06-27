#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::literal::{Literal, LiteralType},
        CodeArea,
    };

    #[test]
    fn test_true() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("true ".as_bytes());

        let output = Literal::lex_literal(&mut reader)?.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 4);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(
            matches!(output.get_type(), LiteralType::Boolean(true)),
            true
        );
        assert!(!reader.is_done());

        Ok(())
    }

    #[test]
    fn test_true_short() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("true".as_bytes());

        let output = Literal::lex_literal(&mut reader)?.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 4);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(
            matches!(output.get_type(), LiteralType::Boolean(true)),
            true
        );
        assert!(reader.is_done());

        Ok(())
    }

    #[test]
    fn test_true_invalid() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("trueSomething".as_bytes());

        let output = Literal::lex_literal(&mut reader)?;
        assert!(output.is_none());

        Ok(())
    }

    #[test]
    fn test_false() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("false ".as_bytes());

        let output = Literal::lex_literal(&mut reader)?.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 5);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(
            matches!(output.get_type(), LiteralType::Boolean(false)),
            true
        );

        Ok(())
    }

    #[test]
    fn test_false_invalid() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("falseSomething".as_bytes());

        let output = Literal::lex_literal(&mut reader)?;
        assert!(output.is_none());

        Ok(())
    }

    #[test]
    fn test_string() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("\"some string literal\" ".as_bytes());

        let output = Literal::lex_literal(&mut reader)?.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 21);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), LiteralType::String(_)), true);
        match output.get_type() {
            LiteralType::String(value) => assert_eq!(value, "some string literal"),
            _ => {}
        }

        Ok(())
    }

    #[test]
    fn test_string_escaped_quotation_mark() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("\"some string\\\" literal\" ".as_bytes());

        let output = Literal::lex_literal(&mut reader)?.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 23);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), LiteralType::String(_)), true);
        match output.get_type() {
            LiteralType::String(value) => assert_eq!(value, "some string\\\" literal"),
            _ => {}
        }

        Ok(())
    }

    #[test]
    fn test_string_empty() -> Result<(), InputReaderError> {
        let output = Literal::lex_literal(&mut InputReader::new("\"\"".as_bytes()))?.unwrap();
        match output.get_type() {
            LiteralType::String(value) => assert_eq!(value, ""),
            _ => {}
        }

        Ok(())
    }

    #[test]
    fn test_string_invalid() -> Result<(), InputReaderError> {
        let reader = &mut InputReader::new("\"".as_bytes());
        let out = Literal::lex_literal(reader)?;
        
        assert!(out.is_none());
        assert_eq!(reader.get_current_position().line, 0);
        assert_eq!(reader.get_current_position().character, 0);

        Ok(())
    }

    #[test]
    fn test_string_invalid_2() -> Result<(), InputReaderError> {
        let reader = &mut InputReader::new("\"daawawdawd".as_bytes());
        let out = Literal::lex_literal(reader)?;
        
        assert!(out.is_none());
        assert_eq!(reader.get_current_position().line, 0);
        assert_eq!(reader.get_current_position().character, 0);

        Ok(())
    }

    #[test]
    fn test_number_float() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("5458.5166".as_bytes());

        let output = Literal::lex_literal(&mut reader)?.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 9);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), LiteralType::Float(_)), true);
        match output.get_type() {
            LiteralType::Float(value) => assert_eq!(value.to_string(), 5458.5166.to_string()),
            _ => {}
        }

        Ok(())
    }

    #[test]
    fn test_number_float_negative() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("-5458.5166".as_bytes());

        let output = Literal::lex_literal(&mut reader)?.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 10);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), LiteralType::Float(_)), true);
        match output.get_type() {
            LiteralType::Float(value) => assert_eq!(value.to_string(), (-5458.5166).to_string()),
            _ => {}
        }

        Ok(())
    }

    #[test]
    fn test_number_int() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("5458".as_bytes());

        let output = Literal::lex_literal(&mut reader)?.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 4);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), LiteralType::Integer(_)), true);
        match output.get_type() {
            LiteralType::Float(value) => assert_eq!(value.to_string(), 5458.to_string()),
            _ => {}
        }

        Ok(())
    }

    #[test]
    fn test_number_int_negative() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("-5458".as_bytes());

        let output = Literal::lex_literal(&mut reader)?.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 5);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), LiteralType::Integer(_)), true);
        match output.get_type() {
            LiteralType::Float(value) => assert_eq!(value.to_string(), (-5458).to_string()),
            _ => {}
        }

        Ok(())
    }

    #[test]
    fn test_number_invalid() -> Result<(), InputReaderError> {
        assert_eq!(
            Literal::lex_literal(&mut InputReader::new("-98234.".as_bytes()))?.is_none(),
            false
        ); // should be -98234.0
        assert_eq!(
            Literal::lex_literal(&mut InputReader::new("-".as_bytes()))?.is_none(),
            true
        );
        assert_eq!(
            Literal::lex_literal(&mut InputReader::new(".".as_bytes()))?.is_none(),
            true
        );
        assert_eq!(
            Literal::lex_literal(&mut InputReader::new("-98234.234.".as_bytes()))?.is_none(),
            true
        );
        assert_eq!(
            Literal::lex_literal(&mut InputReader::new("-98234.234.34234".as_bytes()))?.is_none(),
            true
        );
        assert_eq!(
            Literal::lex_literal(&mut InputReader::new("--98234".as_bytes()))?.is_none(),
            true
        );
        assert_eq!(
            Literal::lex_literal(&mut InputReader::new(".98234".as_bytes()))?.is_none(),
            true
        );

        Ok(())
    }
}
