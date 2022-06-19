use std::io::{Error, Read};

use crate::transpiler::parser::{
    input_reader::{InputReader, InputReaderError},
    CodePosition, CodeArea,
};

/**
    A documentational comment which can be used to document something.
 */
#[derive(Clone)]
pub struct DocumentationalComment {
    content: String,
    start: CodePosition,
    end: CodePosition,
}

impl DocumentationalComment {
    pub fn lex_documentational_comment<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<DocumentationalComment>, InputReaderError> {
        let peek = reader.peek(3)?;

        let start = reader.get_current_position().clone();

        /*
            Documentational comments are enclosed by a starting /** and a closing */
        */

        if peek.starts_with("/**") {
            reader.consume(3)?;
            let content = reader.consume_until_or_end("*/")?;
            return Ok(Some(DocumentationalComment {
                start: start,
                end: reader.get_current_position().clone(),
                content: content.strip_suffix("*/").unwrap_or(&content).to_string(),
            }));
        }

        Ok(None)
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }
}

impl CodeArea for DocumentationalComment {
    fn get_start(&self) -> &CodePosition {
        &self.start
    }

    fn get_end(&self) -> &CodePosition {
        &self.end
    }
}