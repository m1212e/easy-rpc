#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        lexer::disposeable_comment::DisposeableComment,
    };

    #[test]
    fn test_success_nbs() -> Result<(), InputReaderError> {
        let input = "#This is a simple test string!";
        let mut reader = InputReader::new(input.as_bytes());
        let output = DisposeableComment::lex_disposeable_comment(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, input.len() as u32);
        assert_eq!(output.range.end.line, 0);
        assert_eq!(output.content, "This is a simple test string!");

        Ok(())
    }

    #[test]
    fn test_success_2_nbs() -> Result<(), InputReaderError> {
        let mut reader =
            InputReader::new("#This is a simple test string!\nthis is no comment".as_bytes());
        let output = DisposeableComment::lex_disposeable_comment(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 0);
        assert_eq!(output.range.end.line, 1);
        assert_eq!(output.content, "This is a simple test string!\n");
        assert_eq!(reader.peek(18)?.unwrap(), "this is no comment");

        Ok(())
    }

    #[test]
    fn test_success_slash() -> Result<(), InputReaderError> {
        let input = "//This is a simple test string!";
        let mut reader = InputReader::new(input.as_bytes());
        let output = DisposeableComment::lex_disposeable_comment(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, input.len() as u32);
        assert_eq!(output.range.end.line, 0);
        assert_eq!(output.content, "This is a simple test string!");

        Ok(())
    }

    #[test]
    fn test_success_2_slash() -> Result<(), InputReaderError> {
        let mut reader =
            InputReader::new("//This is a simple test string!\nthis is no comment".as_bytes());
        let output = DisposeableComment::lex_disposeable_comment(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 0);
        assert_eq!(output.range.end.line, 1);
        assert_eq!(output.content, "This is a simple test string!\n");
        assert_eq!(reader.peek(18)?.unwrap(), "this is no comment");

        Ok(())
    }

    #[test]
    fn test_success_multiline() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new(
            "/*\nThis is a simple test string!\nthis also comment\n*/something".as_bytes(),
        );
        let output = DisposeableComment::lex_disposeable_comment(&mut reader)?;

        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.range.start.character, 0);
        assert_eq!(output.range.start.line, 0);
        assert_eq!(output.range.end.character, 2);
        assert_eq!(output.range.end.line, 3);
        assert_eq!(
            output.content,
            "\nThis is a simple test string!\nthis also comment\n"
        );
        assert_eq!(reader.peek(9)?.unwrap(), "something");

        Ok(())
    }

    #[test]
    fn test_fail_multiline() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new(
            "/**\nThis is a simple test string!\nthis also comment\n*/".as_bytes(),
        );
        let output = DisposeableComment::lex_disposeable_comment(&mut reader)?;

        assert_eq!(output.is_some(), false);

        Ok(())
    }
}
