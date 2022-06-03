mod tests;

use std::io::{Read};

use self::{disposeable_comment::DisposeableComment, token::Token};

use super::input_reader::{InputReader, InputReaderError};

pub mod disposeable_comment;
pub mod documentational_comment;
pub mod token;

pub struct TokenReader {
    buffer: Vec<Token>,
    done: bool,
}

impl TokenReader {
    pub fn new<T: Read>(reader: InputReader<T>) -> Result<TokenReader, InputReaderError> {
        let mut ret = TokenReader {
            buffer: Vec::new(),
            done: false,
        };

        ret.run(reader)?;
        return Ok(ret);
    }

    fn run<T: Read>(&mut self, mut reader: InputReader<T>) -> Result<(), InputReaderError> {
        loop {
            if reader.is_done() {
                break;
            }

            if let Some(value) = DisposeableComment::lex_disposeable_comment(&mut reader)? {
                self.buffer.push(value.into());
            }

            if reader.is_done() {
                break;
            }

            //invalid
        }

        Ok(())
    }
}
