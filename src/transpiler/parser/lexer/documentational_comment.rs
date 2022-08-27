use std::io::Read;

use crate::{
    transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        CodePosition,
    },
    unwrap_result_option,
};

/**
   A documentational comment which can be used to document something.
*/
#[derive(Clone, Debug)]
pub struct DocumentationalComment {
    pub content: String,
    pub start: CodePosition,
    pub end: CodePosition,
}

impl DocumentationalComment {
    pub fn lex_documentational_comment<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<DocumentationalComment>, InputReaderError> {
        let peek = unwrap_result_option!(reader.peek(3));

        let start = reader.current_position.clone();

        /*
            Documentational comments are enclosed by a starting /** and a closing */
        */

        if peek.starts_with("/**") {
            reader.consume(3)?;
            let content = unwrap_result_option!(reader.consume_to_delimeter_or_end("*/"));
            return Ok(Some(DocumentationalComment {
                start: start,
                end: reader.current_position.clone(),
                content: content.strip_suffix("*/").unwrap_or(&content).to_string(),
            }));
        }

        Ok(None)
    }

}
