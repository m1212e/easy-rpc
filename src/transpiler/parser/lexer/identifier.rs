use lazy_static::lazy_static;
use regex::Regex;
use std::io::Read;

use crate::transpiler::parser::{
    input_reader::{InputReader, InputReaderError},
    CodePosition, CodeArea,
};

/**
    An Identifier for various elements inside the source code. Identifiers must match ^[A-Za-z0-9_]$ 
 */
pub struct Identifier {
    content: String,
    start: CodePosition,
    end: CodePosition,
}

impl Identifier {
    pub fn lex_identifier<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<Identifier>, InputReaderError> {
        lazy_static! {
            static ref VALIDATOR: Regex = Regex::new("^[A-Za-z0-9_]$").unwrap();
        };

        fn approve(current: char, _: &String) -> bool {
            return VALIDATOR.is_match(current.to_string().as_str());
        }

        let ret = reader.peek_until(approve)?.len();

        if ret == 0 {
            return Ok(None);
        }

        let start = reader.get_current_position().clone();
        let content = reader.consume(ret)?;
        let end = reader.get_current_position().clone();

        Ok(Some(Identifier {
            content: content,
            end: end,
            start: start,
        }))
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }
}

impl CodeArea for Identifier {
    fn get_start(&self) -> &CodePosition {
        &self.start
    }

    fn get_end(&self) -> &CodePosition {
        &self.end
    }
}