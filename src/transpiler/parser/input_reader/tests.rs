#[cfg(test)]
mod tests {

    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        CodePosition,
    };

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
            Err(err) => assert_eq!(matches!(err, InputReaderError::Done), true),
        }

        assert_eq!(reader.current_position.character, 7);
        assert_eq!(reader.current_position.line, 2);

        Ok(())
    }

    #[test]
    fn test_empty() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("".as_bytes());

        match reader.peek(5) {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::Done), true),
        }
        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 0);

        match reader.peek(10) {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::Done), true),
        }

        match reader.consume(10) {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::Done), true),
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
            Err(err) => assert_eq!(matches!(err, InputReaderError::Done), true),
        }

        match reader.consume(10) {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::Done), true),
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

    #[test]
    fn test_consume_until() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎ unicode \n symbol!".as_bytes());

        assert_eq!(reader.consume_until_or_end("❎")?, "This is a ❎");
        assert_eq!(reader.consume_until_or_end("\n")?, " unicode \n");
        assert_eq!(reader.consume_until_or_end("nonexistent")?, " symbol!");

        match reader.consume_until_or_end("nonexistent") {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::Done), true),
        }

        Ok(())
    }

    #[test]
    fn test_consume_until_only_nonexistent() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎ unicode 👶 symbol!".as_bytes());

        assert_eq!(
            reader.consume_until_or_end("\n")?,
            "This is a ❎ unicode 👶 symbol!"
        );

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

        fn approve(current: char, total: &String) -> bool {
            if current == '❎' {
                assert_eq!(total, "This is a ❎");
            }
            return current != '❎';
        }
        let ret = reader.peek_until(approve)?;
        assert_eq!(ret, "This is a ");
        assert_eq!(reader.get_current_position().character, 0);

        fn approve2(current: char, _: &String) -> bool {
            return current != '_';
        }
        let ret = reader.peek_until(approve2)?;
        assert_eq!(ret, "This is a ❎ unicode 👶 symbol!");
        assert_eq!(reader.get_current_position().character, 0);

        fn approve3(current: char, _: &String) -> bool {
            return current != '!';
        }
        let ret = reader.peek_until(approve3)?;
        assert_eq!(ret, "This is a ❎ unicode 👶 symbol");
        assert_eq!(reader.get_current_position().character, 0);

        let mut reader = InputReader::new("!".as_bytes());

        fn approve4(current: char, _: &String) -> bool {
            return current != '?';
        }
        let ret = reader.peek_until(approve4)?;
        assert_eq!(ret, "!");
        assert_eq!(reader.get_current_position().character, 0);

        let mut reader = InputReader::new("Hello//".as_bytes());

        fn approve5(current: char, _: &String) -> bool {
            return current != 'o';
        }
        let ret = reader.peek_until(approve5)?;
        assert_eq!(ret, "Hell");
        assert_eq!(reader.get_current_position().character, 0);

        Ok(())
    }

    #[test]
    fn test_peek_until_empty() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("".as_bytes());

        assert_eq!(reader.get_current_position().character, 0);

        fn approve(current: char, _: &String) -> bool {
            return current != '❎';
        }
        match reader.peek_until(approve) {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::Done), true),
        }

        Ok(())
    }

    #[test]
    fn test_peek_empty() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("".as_bytes());

        match reader.peek(1) {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::Done), true),
        }

        Ok(())
    }

    #[test]
    fn test_consume_empty() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("".as_bytes());

        match reader.consume(1) {
            Ok(_) => panic!("Should return error"),
            Err(err) => assert_eq!(matches!(err, InputReaderError::Done), true),
        }

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
