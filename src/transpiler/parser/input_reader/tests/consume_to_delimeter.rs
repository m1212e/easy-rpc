#[cfg(test)]
mod tests {

    use crate::transpiler::parser::input_reader::{InputReader, InputReaderError};

    #[test]
    fn test_consume_to_delimeter() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎ unicode \n symbol!".as_bytes());

        assert_eq!(
            reader.consume_to_delimeter_or_end("❎")?.unwrap(),
            "This is a ❎"
        );
        assert_eq!(
            reader.consume_to_delimeter_or_end("\n")?.unwrap(),
            " unicode \n"
        );
        assert_eq!(reader.peek(5)?.unwrap(), " symb");
        assert_eq!(
            reader.consume_to_delimeter_or_end("nonexistent")?.unwrap(),
            " symbol!"
        );

        assert!(reader.consume_to_delimeter_or_end("nonexistent")?.is_none());

        Ok(())
    }

    #[test]
    fn test_consume__to_delimeter_only_nonexistent() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("This is a ❎ unicode 👶 symbol!".as_bytes());

        assert_eq!(
            reader.consume_to_delimeter_or_end("\n")?.unwrap(),
            "This is a ❎ unicode 👶 symbol!"
        );

        Ok(())
    }

    #[test]
    fn test_consume__to_delimeter_empty() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("".as_bytes());

        assert!(reader.consume_to_delimeter_or_end("\n")?.is_none());

        Ok(())
    }
}
