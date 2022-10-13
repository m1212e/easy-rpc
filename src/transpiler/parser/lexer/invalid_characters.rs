use std::io::Read;

use crate::{
    transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        CodePosition,
    },
    unwrap_result_option,
};

/**
   Invalid characters which are unknown to the parser or are missplaced in a kind of way that the parser
   cant handle them. This is used as a fallback to handle invalid syntax.
*/
#[derive(Clone, Debug)]
pub struct InvalidCharacters {
    pub content: String,
    pub start: CodePosition,
    pub end: CodePosition,
}

impl InvalidCharacters {
    pub fn lex_next_as_invalid_character<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<InvalidCharacters>, InputReaderError> {
        return Ok(Some(InvalidCharacters {
            start: reader.current_position.clone(),
            content: unwrap_result_option!(reader.consume(1)),
            end: reader.current_position.clone(),
        }));
    }
}
