use std::io::Read;

use crate::{
    transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        CodePosition,
    },
    unwrap_result_option,
};

/**
   Line breaks are all kinds of newline chars. They are lexed seperately to improve parser quality.
*/
#[derive(Clone, Debug)]
pub struct LineBreak {
    pub start: CodePosition,
    pub end: CodePosition,
}

impl LineBreak {
    pub fn lex_line_break<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<LineBreak>, InputReaderError> {
        let peek = unwrap_result_option!(reader.peek(1));

        if peek == "\n" || peek == "\r" {
            let start = reader.current_position.clone();
            reader.consume(1)?;

            loop {
                let peek = reader.peek(1)?;

                match peek {
                    Some(val) => {
                        if val != "\n" && val != "\r" {
                            break;
                        }
                        reader.consume(1)?;
                    }
                    None => break,
                }
            }
            let end = reader.current_position.clone();

            return Ok(Some(LineBreak { start, end }));
        }

        Ok(None)
    }
}
