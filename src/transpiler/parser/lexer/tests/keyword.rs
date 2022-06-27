#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::keyword::{Keyword, KeywordType},
        CodeArea,
    };

    #[test]
    fn test_type_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("typeblah".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 4);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::Type), true);

        assert_eq!(reader.peek(4)?.unwrap(), "blah");
        Ok(())
    }

    #[test]
    fn test_only_type_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("type".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 4);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::Type), true);
        assert!(reader.is_done());

        Ok(())
    }

    #[test]
    fn test_type_invalid() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("typ".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), false);

        Ok(())
    }

    #[test]
    fn test_import_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("importblah".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 6);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::Import), true);

        assert_eq!(reader.peek(4)?.unwrap(), "blah");
        Ok(())
    }

    #[test]
    fn test_boolean_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("booleanblah".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 7);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::Boolean), true);

        assert_eq!(reader.peek(4)?.unwrap(), "blah");
        Ok(())
    }

    #[test]
    fn test_int_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("intblah".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 3);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::Int), true);

        assert_eq!(reader.peek(4)?.unwrap(), "blah");
        Ok(())
    }

    #[test]
    fn test_int8_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("int8blah".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 4);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::Int8), true);

        assert_eq!(reader.peek(4)?.unwrap(), "blah");
        Ok(())
    }

    #[test]
    fn test_int16_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("int16blah".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 5);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::Int16), true);

        assert_eq!(reader.peek(4)?.unwrap(), "blah");
        Ok(())
    }

    #[test]
    fn test_int32_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("int32blah".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 5);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::Int32), true);

        assert_eq!(reader.peek(4)?.unwrap(), "blah");
        Ok(())
    }

    #[test]
    fn test_int64_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("int64blah".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 5);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::Int64), true);

        assert_eq!(reader.peek(4)?.unwrap(), "blah");
        Ok(())
    }

    #[test]
    fn test_float_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("floatblah".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 5);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::Float), true);

        assert_eq!(reader.peek(4)?.unwrap(), "blah");
        Ok(())
    }

    #[test]
    fn test_float32_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("float32blah".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 7);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::Float32), true);

        assert_eq!(reader.peek(4)?.unwrap(), "blah");
        Ok(())
    }

    #[test]
    fn test_float64_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("float64blah".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 7);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::Float64), true);

        assert_eq!(reader.peek(4)?.unwrap(), "blah");
        Ok(())
    }

    #[test]
    fn test_string_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("stringblah".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 6);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(matches!(output.get_type(), KeywordType::String), true);

        assert_eq!(reader.peek(4)?.unwrap(), "blah");
        Ok(())
    }
}
