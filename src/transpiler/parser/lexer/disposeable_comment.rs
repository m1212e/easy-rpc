use std::io::{Error, Read};

use crate::transpiler::parser::{
    input_reader::{InputReader, InputReaderError},
    CodePosition,
};

pub struct DisposeableComment {
    content: String,
    start: CodePosition,
    end: CodePosition,
}

impl DisposeableComment {
    pub fn lex_disposeable_comment<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<DisposeableComment>, InputReaderError> {
        let peek = reader.peek(2)?;

        let start = reader.get_current_position().clone();

        if peek.starts_with("#") {
            reader.consume(1)?;
            let content = reader.consume_until_or_end("\n")?;
            return Ok(Some(DisposeableComment {
                start: start,
                end: reader.get_current_position().clone(),
                content: content,
            }));
        }

        if peek.starts_with("//") {
            reader.consume(2)?;
            let content = reader.consume_until_or_end("\n")?;
            return Ok(Some(DisposeableComment {
                start: start,
                end: reader.get_current_position().clone(),
                content: content,
            }));
        }

        if peek.starts_with("/*") {
            reader.consume(2)?;
            let content = reader.consume_until_or_end("*/")?;
            return Ok(Some(DisposeableComment {
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

    pub fn get_start(&self) -> &CodePosition {
        &self.start
    }

    pub fn get_end(&self) -> &CodePosition {
        &self.end
    }
}
