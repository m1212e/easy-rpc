#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::TokenReader,
    };

    #[test]
    fn test_success() -> Result<(), InputReaderError> {
        todo!();
        let mut reader =
            TokenReader::new(InputReader::new("//Hello from the other side".as_bytes()))?;
    }
}
