#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{input_reader::{InputReaderError, InputReader}, lexer::disposeable_comment::DisposeableComment, CodeArea};

    #[test]
    fn test_success_nbs() -> Result<(), InputReaderError> {
        let input = "#This is a simple test string!";
        let mut reader = InputReader::new(input.as_bytes());
        let output = DisposeableComment::lex_disposeable_comment(&mut reader)?;
        
        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, input.len() as u16);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(output.get_content(), "This is a simple test string!");

        Ok(())
    }

    #[test]
    fn test_success_2_nbs() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("#This is a simple test string!\nthis is no comment".as_bytes());
        let output = DisposeableComment::lex_disposeable_comment(&mut reader)?;
        
        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 0);
        assert_eq!(output.get_end().line, 1);
        assert_eq!(output.get_content(), "This is a simple test string!\n");

        Ok(())
    }

    #[test]
    fn test_success_slash() -> Result<(), InputReaderError> {
        let input = "//This is a simple test string!";
        let mut reader = InputReader::new(input.as_bytes());
        let output = DisposeableComment::lex_disposeable_comment(&mut reader)?;
        
        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, input.len() as u16);
        assert_eq!(output.get_end().line, 0);
        assert_eq!(output.get_content(), "This is a simple test string!");

        Ok(())
    }

    #[test]
    fn test_success_2_slash() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("//This is a simple test string!\nthis is no comment".as_bytes());
        let output = DisposeableComment::lex_disposeable_comment(&mut reader)?;
        
        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 0);
        assert_eq!(output.get_end().line, 1);
        assert_eq!(output.get_content(), "This is a simple test string!\n");

        Ok(())
    }

    #[test]
    fn test_success_multiline() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("/*\nThis is a simple test string!\nthis also comment\n*/".as_bytes());
        let output = DisposeableComment::lex_disposeable_comment(&mut reader)?;
        
        assert_eq!(output.is_some(), true);
        let output = output.unwrap();
        assert_eq!(output.get_start().character, 0);
        assert_eq!(output.get_start().line, 0);
        assert_eq!(output.get_end().character, 2);
        assert_eq!(output.get_end().line, 3);
        assert_eq!(output.get_content(), "\nThis is a simple test string!\nthis also comment\n");

        Ok(())
    }

    #[test]
    fn test_fail_multiline() -> Result<(), InputReaderError> {
        let mut reader = InputReader::new("/**\nThis is a simple test string!\nthis also comment\n*/".as_bytes());
        let output = DisposeableComment::lex_disposeable_comment(&mut reader)?;
        
        assert_eq!(output.is_some(), false);

        Ok(())
    }
}
