#[cfg(test)]
mod tests {

    use crate::transpiler::parser::input_reader::{InputReader, InputReaderError};

    #[test]
    fn test_peek_and_consume() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a simple test string!".as_bytes());

        assert_eq!(reader.peek(5)?, "This ");
        assert_eq!(reader.peek(9)?, "This is a");
        assert_eq!(reader.consume(4)?, "This");
        assert_eq!(reader.consume(3)?, " is");
        assert_eq!(reader.peek(9)?, " a simple");
        assert_eq!(reader.peek(9)?, " a simple");
        assert_eq!(reader.consume(1)?, " ");
        assert_eq!(reader.peek(300)?, "a simple test string!");

        Ok(())
    }

    #[test]
    fn test_code_position() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a\nsimple test\nstring!".as_bytes());

        reader.peek(5)?;
        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 0);

        reader.consume(4)?;
        assert_eq!(reader.current_position.character, 4);
        assert_eq!(reader.current_position.line, 0);

        reader.peek(5)?;
        assert_eq!(reader.current_position.character, 4);
        assert_eq!(reader.current_position.line, 0);

        reader.consume(7)?;
        assert_eq!(reader.current_position.character, 1);
        assert_eq!(reader.current_position.line, 1);

        reader.consume(6)?;
        assert_eq!(reader.current_position.character, 7);
        assert_eq!(reader.current_position.line, 1);

        reader.consume(10)?;
        assert_eq!(reader.current_position.character, 5);
        assert_eq!(reader.current_position.line, 2);

        reader.consume(10)?;
        assert_eq!(reader.current_position.character, 7);
        assert_eq!(reader.current_position.line, 2);

        match reader.consume(10) {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::DONE), true),
        }

        assert_eq!(reader.current_position.character, 7);
        assert_eq!(reader.current_position.line, 2);

        Ok(())
    }

    #[test]
    fn test_empty() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("".as_bytes());

        assert_eq!(reader.peek(5)?, "");
        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 0);

        match reader.peek(10) {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::DONE), true),
        }

        match reader.consume(10) {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::DONE), true),
        }

        Ok(())
    }

    #[test]
    fn test_newlines_only() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("\n\n\n".as_bytes());

        assert_eq!(reader.consume(5)?, "\n\n\n");
        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 3);

        match reader.peek(10) {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::DONE), true),
        }

        match reader.consume(10) {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::DONE), true),
        }

        Ok(())
    }

    #[test]
    fn test_unicode() -> Result<(), InputReaderError> {
        let input: &[u8] = [226, 133, 156, 66, 66].as_slice();
        let mut reader = InputReader::new(input);

        assert_eq!(reader.consume(5)?, "⅜BB");

        Ok(())
    }

    #[test]
    fn test_unicode_2() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎ unicode 👶 symbol!".as_bytes());

        assert_eq!(reader.consume(12)?, "This is a ❎ ");
        assert_eq!(reader.consume(17)?, "unicode 👶 symbol!");

        Ok(())
    }
}
