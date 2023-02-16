use std::io::Read;

use tower_lsp::lsp_types::Range;

use crate::{
    transpiler::parser::input_reader::{InputReader, InputReaderError},
    unwrap_result_option,
};

/**
   Invalid characters which are unknown to the parser or are missplaced in a kind of way that the parser
   cant handle them. This is used as a fallback to handle invalid syntax.
*/
#[derive(Clone, Debug)]
pub struct InvalidCharacters {
    pub content: String,
    pub range: Range,
}

impl InvalidCharacters {
    pub fn lex_next_as_invalid_character<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<InvalidCharacters>, InputReaderError> {
        let start = reader.current_position.clone();
        let content = unwrap_result_option!(reader.consume(1));
        let end = reader.current_position.clone();

        return Ok(Some(InvalidCharacters {
            content,
            range: Range { start, end },
        }));
    }
}
