mod tests;

use std::io::{Read};

use self::{disposeable_comment::DisposeableComment, token::Token, invalid_characters::InvalidCharacters, documentational_comment::DocumentationalComment, line_break::LineBreak, space::Space, operator::Operator, keyword::Keyword, literal::Literal, identifier::Identifier};

use super::input_reader::{InputReader, InputReaderError};

pub mod disposeable_comment;
pub mod documentational_comment;
pub mod invalid_characters;
pub mod identifier;
pub mod token;
pub mod space;
pub mod keyword;
pub mod line_break;
pub mod literal;
pub mod operator;

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

        // The order in which the tokens are processed matters!

        loop {
            if reader.is_done() {
                break;
            }

            if let Some(value) = DocumentationalComment::lex_documentational_comment(&mut reader)? {
                self.buffer.push(value.into());
            }

            if let Some(value) = DisposeableComment::lex_disposeable_comment(&mut reader)? {
                self.buffer.push(value.into());
            }
            
            Space::skip_space(&mut reader)?;

            if let Some(value) = LineBreak::lex_line_break(&mut reader)? {
                self.buffer.push(value.into());
            }

            if let Some(value) = Operator::lex_operator(&mut reader)? {
                self.buffer.push(value.into());
            }

            if let Some(value) = Keyword::lex_keyword(&mut reader)? {
                self.buffer.push(value.into());
            }

            if let Some(value) = Literal::lex_literal(&mut reader)? {
                self.buffer.push(value.into());
            }

            if let Some(value) = Identifier::lex_identifier(&mut reader)? {
                self.buffer.push(value.into());
            }

            if reader.is_done() {
                break;
            }

            if let Some(value) = InvalidCharacters::lex_next_as_invalid_character(&mut reader)? {
                self.buffer.push(value.into());
            }
        }

        Ok(())
    }

    pub fn peek(&mut self, amount: usize) -> Option<&[Token]> {
        if self.done {
            return None;
        }

        let elements = &self.buffer[1..amount];

        return None;
    }
}
