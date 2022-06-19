use std::io::{Error, Read};

use crate::transpiler::parser::{
    input_reader::{InputReader, InputReaderError},
    CodeArea, CodePosition,
};

/**
   A comment which can be ignored.
*/
pub struct DisposeableComment {
    content: String,
    start: CodePosition,
    end: CodePosition,
}

impl DisposeableComment {
    pub fn lex_disposeable_comment<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<DisposeableComment>, InputReaderError> {
        let peek = reader.peek(3)?;

        let start = reader.get_current_position().clone();

        /*
            There are 3 types of disposeable comments:
            // Single line comments introduced with a double slash
            # Single line comments introduced with a number sign

            /*
                Multi line comments enclosed by /* and */
            */
        */

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

        if peek.starts_with("/*") && !peek.starts_with("/**") {
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
}

impl CodeArea for DisposeableComment {
    fn get_start(&self) -> &CodePosition {
        &self.start
    }

    fn get_end(&self) -> &CodePosition {
        &self.end
    }
}
