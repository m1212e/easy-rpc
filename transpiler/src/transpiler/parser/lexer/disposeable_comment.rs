use std::io::Read;

use tower_lsp::lsp_types::Range;

use crate::{
    transpiler::parser::input_reader::{InputReader, InputReaderError},
    unwrap_result_option,
};

/**
   A comment which can be ignored.
*/
#[derive(Clone, Debug)]
pub struct DisposeableComment {
    pub content: String,
    pub range: Range,
}

impl DisposeableComment {
    pub fn lex_disposeable_comment<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<DisposeableComment>, InputReaderError> {
        let peek = unwrap_result_option!(reader.peek(3));
        let start = reader.current_position.clone();

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
            let content = unwrap_result_option!(reader.consume_to_delimeter_or_end("\n"));
            return Ok(Some(DisposeableComment {
                range: Range {
                    start,
                    end: reader.current_position.clone(),
                },
                content,
            }));
        }

        if peek.starts_with("//") {
            reader.consume(2)?;
            let content = unwrap_result_option!(reader.consume_to_delimeter_or_end("\n"));
            return Ok(Some(DisposeableComment {
                range: Range {
                    start,
                    end: reader.current_position.clone(),
                },
                content,
            }));
        }

        if peek.starts_with("/*") && !peek.starts_with("/**") {
            reader.consume(2)?;
            let content = unwrap_result_option!(reader.consume_to_delimeter_or_end("*/"));
            return Ok(Some(DisposeableComment {
                range: Range {
                    start,
                    end: reader.current_position.clone(),
                },
                content: content.strip_suffix("*/").unwrap_or(&content).to_string(),
            }));
        }

        Ok(None)
    }
}
