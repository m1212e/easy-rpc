use std::io::Read;

use crate::transpiler::parser::{
    input_reader::{InputReader, InputReaderError},
    CodeArea, CodePosition,
};

/**
    Line breaks are all kinds of newline chars. They are lexed seperately to improve parser quality.
 */
#[derive(Clone)]
pub struct LineBreak {
    start: CodePosition,
    end: CodePosition,
}

impl LineBreak {
    pub fn lex_line_break<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<LineBreak>, InputReaderError> {
        let peek = reader.peek(2)?;

        if peek.starts_with("\n") {
            let start = reader.get_current_position().clone();
            reader.consume(1)?;
            let end = reader.get_current_position().clone();
            return Ok(Some(LineBreak { start, end }));
        }

        if peek == "\r\n" {
            let start = reader.get_current_position().clone();
            reader.consume(2)?;
            let end = reader.get_current_position().clone();
            return Ok(Some(LineBreak { start, end }));
        }

        Ok(None)
    }
}

impl CodeArea for LineBreak {
    fn get_start(&self) -> &CodePosition {
        &self.start
    }

    fn get_end(&self) -> &CodePosition {
        &self.end
    }
}
