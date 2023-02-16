#[cfg(test)]
mod tests {

    use crate::transpiler::parser::input_reader::{InputReader, InputReaderError};

    #[test]
    fn test() -> Result<(), InputReaderError> {
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
    fn test_getter() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎ unicode 👶 symbol!".as_bytes());

        let cp1 = reader.current_position.clone();
        reader.consume(5)?;
        let cp2 = reader.current_position.clone();
        reader.consume(5)?;
        let cp3 = reader.current_position.clone();
        assert_eq!(cp1.character, 0);
        assert_eq!(cp1.line, 0);
        assert_eq!(cp2.character, 5);
        assert_eq!(cp2.line, 0);
        assert_eq!(cp3.character, 10);
        assert_eq!(cp3.line, 0);

        Ok(())
    }

    #[test]
    fn test_consume_delimeter() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎\n unicode 👶 symbol!".as_bytes());

        reader.consume_to_delimeter_or_end("❎")?;
        assert_eq!(reader.current_position.character, 11);
        assert_eq!(reader.current_position.line, 0);
        reader.consume_to_delimeter_or_end("👶")?;
        assert_eq!(reader.current_position.character, 10);
        assert_eq!(reader.current_position.line, 1);
        reader.consume_to_delimeter_or_end("nonexistent")?;
        assert_eq!(reader.current_position.character, 18);
        assert_eq!(reader.current_position.line, 1);

        Ok(())
    }

    #[test]
    fn test_peek_until() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎\n unicode 👶 symbol!".as_bytes());

        reader.peek_until(|current, _| {
            return current != '\n';
        })?;
        assert_eq!(reader.current_position.character, 0);
        assert_eq!(reader.current_position.line, 0);

        Ok(())
    }
}
