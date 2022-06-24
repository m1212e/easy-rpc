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

        assert!(reader.consume(10)?.is_none());
        assert_eq!(reader.current_position.character, 5);
        assert_eq!(reader.current_position.line, 2);

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
    fn test_consume_until() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎ unicode \n symbol!".as_bytes());

        assert_eq!(reader.consume_until_or_end("❎")?.unwrap(), "This is a ❎");
        assert_eq!(reader.consume_until_or_end("\n")?.unwrap(), " unicode \n");
        assert_eq!(reader.peek(5)?.unwrap(), " symb");
        assert_eq!(
            reader.consume_until_or_end("nonexistent")?.unwrap(),
            " symbol!"
        );

        assert!(reader.consume_until_or_end("nonexistent")?.is_none());

        Ok(())
    }

    #[test]
    fn test_consume_until_only_nonexistent() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎ unicode 👶 symbol!".as_bytes());

        assert_eq!(
            reader.consume_until_or_end("\n")?.unwrap(),
            "This is a ❎ unicode 👶 symbol!"
        );

        Ok(())
    }

    #[test]
    fn test_consume_until_empty() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("".as_bytes());

        assert!(reader.consume_until_or_end("\n")?.is_none());

        Ok(())
    }

    #[test]
    fn test_get_code_position() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎ unicode 👶 symbol!".as_bytes());

        let cp1 = reader.get_current_position().clone();
        reader.consume(5)?;
        let cp2 = reader.get_current_position().clone();
        reader.consume(5)?;
        let cp3 = reader.get_current_position().clone();
        assert_eq!(cp1.character, 0);
        assert_eq!(cp2.character, 5);
        assert_eq!(cp3.character, 10);

        Ok(())
    }

    #[test]
    fn test_peek_until_valid() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎ unicode 👶 symbol!".as_bytes());

        assert_eq!(reader.get_current_position().character, 0);

        let ret = reader
            .peek_until(|current, total| {
                if current == '❎' {
                    assert_eq!(total, "This is a ❎");
                }
                return current != '❎';
            })?
            .unwrap();
        assert_eq!(ret, "This is a ");
        assert_eq!(reader.get_current_position().character, 0);

        let ret = reader
            .peek_until(|current, total| {
                return current != '_';
            })?
            .unwrap();
        assert_eq!(ret, "This is a ❎ unicode 👶 symbol!");
        assert_eq!(reader.get_current_position().character, 0);

        let ret = reader
            .peek_until(|current, total| {
                return current != '!';
            })?
            .unwrap();
        assert_eq!(ret, "This is a ❎ unicode 👶 symbol");
        assert_eq!(reader.get_current_position().character, 0);

        let mut reader = InputReader::new("!".as_bytes());
        let ret = reader
            .peek_until(|current, total| {
                return current != '?';
            })?
            .unwrap();
        assert_eq!(ret, "!");
        assert_eq!(reader.get_current_position().character, 0);

        let mut reader = InputReader::new("Hello//".as_bytes());
        let ret = reader
            .peek_until(|current, total| {
                return current != 'o';
            })?
            .unwrap();
        assert_eq!(ret, "Hell");
        assert_eq!(reader.get_current_position().character, 0);

        Ok(())
    }

    #[test]
    fn test_peek_until_empty() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("".as_bytes());

        assert_eq!(reader.get_current_position().character, 0);

        assert!(reader
            .peek_until(|current, total| {
                return current != '❎';
            })?
            .is_none());

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

    #[test]
    fn test_is_done() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("abc".as_bytes());

        reader.peek(3)?;
        assert_eq!(reader.is_done(), false);
        reader.consume(2)?;
        assert_eq!(reader.is_done(), false);
        reader.consume(1)?;
        assert_eq!(reader.is_done(), true);

        Ok(())
    }
}
