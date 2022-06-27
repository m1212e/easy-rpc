#[cfg(test)]
mod tests {

    use crate::transpiler::parser::input_reader::{InputReader, InputReaderError};

    #[test]
    fn test_peek_and_consume() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a simple test string!".as_bytes());

        assert_eq!(reader.peek(5)?.unwrap(), "This ");
        assert_eq!(reader.peek(9)?.unwrap(), "This is a");
        assert_eq!(reader.consume(4)?.unwrap(), "This");
        assert_eq!(reader.consume(3)?.unwrap(), " is");
        assert_eq!(reader.peek(9)?.unwrap(), " a simple");
        assert_eq!(reader.peek(9)?.unwrap(), " a simple");
        assert_eq!(reader.consume(1)?.unwrap(), " ");
        assert!(reader.peek(300)?.is_none());
        assert!(reader.consume(300)?.is_none());
        assert_eq!(reader.consume(1)?.unwrap(), "a");

        Ok(())
    }

    #[test]
    fn test_empty() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("".as_bytes());

        assert!(reader.peek(1)?.is_none());
        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 0);
        assert!(reader.peek(10)?.is_none());
        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 0);
        assert!(reader.consume(1)?.is_none());
        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 0);
        assert!(reader.consume(10)?.is_none());
        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 0);

        Ok(())
    }

    #[test]
    fn test_newlines_only() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("\n\n\n".as_bytes());

        assert_eq!(reader.consume(3)?.unwrap(), "\n\n\n");
        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 3);

        assert!(reader.consume(3)?.is_none());
        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 3);

        Ok(())
    }

    #[test]
    fn test_unicode() -> Result<(), InputReaderError> {
        let input: &[u8] = [226, 133, 156, 66, 66].as_slice();
        let mut reader = InputReader::new(input);

        assert_eq!(reader.consume(3)?.unwrap(), "⅜BB");

        Ok(())
    }

    #[test]
    fn test_unicode_2() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎ unicode 👶 symbol!".as_bytes());

        assert_eq!(reader.consume(12)?.unwrap(), "This is a ❎ ");
        assert_eq!(reader.consume(17)?.unwrap(), "unicode 👶 symbol!");

        Ok(())
    }

    #[test]
    fn test_peek_empty() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("".as_bytes());

        assert!(reader.peek(1)?.is_none());

        Ok(())
    }

    #[test]
    fn test_consume_empty() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("".as_bytes());

        assert!(reader.consume(1)?.is_none());

        Ok(())
    }
}
