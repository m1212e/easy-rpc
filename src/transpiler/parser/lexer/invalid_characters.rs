use std::io::Read;

use crate::transpiler::parser::{
    input_reader::{InputReader, InputReaderError},
    CodePosition, CodeArea,
};

/**
    Invalid characters which are unknown to the parser or are missplaced in a kind of way that the parser
    cant handle them. This is used as a fallback to handle invalid syntax.
 */
#[derive(Clone)]
pub struct InvalidCharacters {
    content: String,
    start: CodePosition,
    end: CodePosition,
}

impl InvalidCharacters {
    pub fn lex_next_as_invalid_character<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<InvalidCharacters>, InputReaderError> {
        return Ok(Some(InvalidCharacters {
            start: reader.get_current_position().clone(),
            content: reader.consume(1)?,
            end: reader.get_current_position().clone(),
        }));
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }
}

impl CodeArea for InvalidCharacters {
    fn get_start(&self) -> &CodePosition {
        &self.start
    }

    fn get_end(&self) -> &CodePosition {
        &self.end
    }
}