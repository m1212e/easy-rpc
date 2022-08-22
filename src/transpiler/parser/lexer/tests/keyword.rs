#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::keyword::{Keyword, KeywordType},
    };

    #[test]
    fn test_type_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("type".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 4);
        assert_eq!(output.end.line, 0);
        assert_eq!(matches!(output.keyword_type, KeywordType::Type), true);

        Ok(())
    }

    #[test]
    fn test_only_type_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("type".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 4);
        assert_eq!(output.end.line, 0);
        assert_eq!(matches!(output.keyword_type, KeywordType::Type), true);
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
    fn test_boolean_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("boolean".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 7);
        assert_eq!(output.end.line, 0);
        assert_eq!(matches!(output.keyword_type, KeywordType::Boolean), true);

        Ok(())
    }

    #[test]
    fn test_int_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("int )".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 3);
        assert_eq!(output.end.line, 0);
        assert_eq!(matches!(output.keyword_type, KeywordType::Int), true);

        Ok(())
    }

    #[test]
    fn test_int8_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("int8".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 4);
        assert_eq!(output.end.line, 0);
        assert_eq!(matches!(output.keyword_type, KeywordType::Int8), true);

        Ok(())
    }

    #[test]
    fn test_int16_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("int16".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 5);
        assert_eq!(output.end.line, 0);
        assert_eq!(matches!(output.keyword_type, KeywordType::Int16), true);

        Ok(())
    }

    #[test]
    fn test_int32_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("int32".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 5);
        assert_eq!(output.end.line, 0);
        assert_eq!(matches!(output.keyword_type, KeywordType::Int32), true);

        Ok(())
    }

    #[test]
    fn test_int64_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("int64".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 5);
        assert_eq!(output.end.line, 0);
        assert_eq!(matches!(output.keyword_type, KeywordType::Int64), true);

        Ok(())
    }

    #[test]
    fn test_float_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("float".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 5);
        assert_eq!(output.end.line, 0);
        assert_eq!(matches!(output.keyword_type, KeywordType::Float), true);

        Ok(())
    }

    #[test]
    fn test_float32_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("float32".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 7);
        assert_eq!(output.end.line, 0);
        assert_eq!(matches!(output.keyword_type, KeywordType::Float32), true);

        Ok(())
    }

    #[test]
    fn test_float64_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("float64".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 7);
        assert_eq!(output.end.line, 0);
        assert_eq!(matches!(output.keyword_type, KeywordType::Float64), true);

        Ok(())
    }

    #[test]
    fn test_string_success() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("string".as_bytes());
        let output = Keyword::lex_keyword(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.start.character, 0);
        assert_eq!(output.start.line, 0);
        assert_eq!(output.end.character, 6);
        assert_eq!(output.end.line, 0);
        assert_eq!(matches!(output.keyword_type, KeywordType::String), true);

        Ok(())
    }
}
