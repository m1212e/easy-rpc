#[cfg(test)]
mod tests {

    use crate::transpiler::parser::input_reader::{InputReader, InputReaderError};

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

    #[test]
    fn test_is_done_empty() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("".as_bytes());

        assert!(reader.is_done());

        Ok(())
    }
}
