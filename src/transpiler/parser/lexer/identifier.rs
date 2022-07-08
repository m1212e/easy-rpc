use lazy_static::lazy_static;
use regex::Regex;
use std::io::Read;

use crate::{
    transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        CodePosition,
    },
    unwrap_result_option,
};

/**
   An Identifier for various elements inside the source code. Identifiers must match ^[A-Za-z0-9_]$
*/
#[derive(Clone, Debug)]
pub struct Identifier {
    pub content: String,
    pub start: CodePosition,
    pub end: CodePosition,
}

impl Identifier {
    pub fn lex_identifier<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<Identifier>, InputReaderError> {
        lazy_static! {
            static ref VALIDATOR: Regex = Regex::new("^[A-Za-z0-9_]$").unwrap();
        };

        let ret = unwrap_result_option!(reader.peek_until(|current, _| {
            return VALIDATOR.is_match(current.to_string().as_str());
        }))
        .len();

        if ret == 0 {
            return Ok(None);
        }

        let start = reader.current_position.clone();
        let content = unwrap_result_option!(reader.consume(ret));
        let end = reader.current_position.clone();

        Ok(Some(Identifier {
            content: content,
            end: end,
            start: start,
        }))
    }

}