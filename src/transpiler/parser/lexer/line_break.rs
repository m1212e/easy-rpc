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
#[derive(Clone)]
pub struct LineBreak {
    pub start: CodePosition,
    pub end: CodePosition,
}

impl LineBreak {
    pub fn lex_line_break<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<LineBreak>, InputReaderError> {

        //TODO: This should group multiple linebreaks to one token in the future:

        /*
            //example comment
            /n                            <- one Linebreak token
            Server someEndpoint()
        */


        /*
            //example comment
            /n                          ╗
            /n                          ║
            /n                          ║ <- one Linebreak token
            /n                          ║
            /n                          ╝
            Server someEndpoint()
        */

        let peek = unwrap_result_option!(reader.peek(2));

        if peek.starts_with("\n") {
            let start = reader.current_position.clone();
            reader.consume(1)?;
            let end = reader.current_position.clone();
            return Ok(Some(LineBreak { start, end }));
        }

        if peek == "\r\n" {
            let start = reader.current_position.clone();
            reader.consume(2)?;
            let end = reader.current_position.clone();
            return Ok(Some(LineBreak { start, end }));
        }

        Ok(None)
    }
}
