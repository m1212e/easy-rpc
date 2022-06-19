use std::io::Read;

use crate::transpiler::parser::{
    input_reader::{InputReader, InputReaderError},
};

/**
    Spaces are skipped and not stored for parsing.
 */
pub struct Space;

impl Space {
    pub fn skip_space<T: Read>(reader: &mut InputReader<T>) -> Result<(), InputReaderError> {
        loop {
            if reader.is_done() {
                break;
            }
            let peeked = reader.peek(1)?;
            if peeked == " " || peeked == "	" {
                reader.consume(1)?;
            } else {
                break;
            }
        }

        Ok(())
    }
}
