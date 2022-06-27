#[cfg(test)]
mod tests {

    use crate::transpiler::parser::input_reader::{InputReader, InputReaderError};

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
}
