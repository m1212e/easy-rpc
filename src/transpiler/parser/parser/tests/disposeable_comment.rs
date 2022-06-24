#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::TokenReader,
        parser::{disposeable_comment::DisposeableComment},
    };

    #[test]
    fn test_success() -> Result<(), InputReaderError> {
        let mut reader =
            TokenReader::new(InputReader::new("//Hello from the other side".as_bytes()))?;
        let result = DisposeableComment::skip_disposeable_comment(&mut reader);
        assert!(result.is_some());
        assert!(reader.is_done());

        let result = DisposeableComment::skip_disposeable_comment(&mut reader);
        assert!(result.is_none());

        Ok(())
    }
}
