mod tests;

use std::io::Read;

use self::{
    disposeable_comment::DisposeableComment, documentational_comment::DocumentationalComment,
    identifier::Identifier, invalid_characters::InvalidCharacters, keyword::Keyword,
    line_break::LineBreak, literal::Literal, operator::Operator, space::Space, token::Token,
};

use super::input_reader::{InputReader, InputReaderError};

pub mod disposeable_comment;
pub mod documentational_comment;
pub mod identifier;
pub mod invalid_characters;
pub mod keyword;
pub mod line_break;
pub mod literal;
pub mod operator;
pub mod space;
pub mod token;

/**
   Wraps around an input reader and lexes the input into tokens.
*/
pub struct TokenReader {
    buffer: Vec<Token>,
    done: bool,
}

impl TokenReader {
    /**
       Creates a new token reader.
    */
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
                continue;
            }

            if let Some(value) = DisposeableComment::lex_disposeable_comment(&mut reader)? {
                self.buffer.push(value.into());
                continue;
            }

            Space::skip_space(&mut reader)?;

            if let Some(value) = LineBreak::lex_line_break(&mut reader)? {
                self.buffer.push(value.into());
                continue;
            }

            if let Some(value) = Operator::lex_operator(&mut reader)? {
                self.buffer.push(value.into());
                continue;
            }

            if let Some(value) = Keyword::lex_keyword(&mut reader)? {
                self.buffer.push(value.into());
                continue;
            }

            if let Some(value) = Literal::lex_literal(&mut reader)? {
                self.buffer.push(value.into());
                continue;
            }

            if let Some(value) = Identifier::lex_identifier(&mut reader)? {
                self.buffer.push(value.into());
                continue;
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

    /**
       Peeks the next n token without consuming it. The option contains none if there are no tokens available to return.
       If the requested amount is greater than the amount which can be supplied, none is returned, even tough there are tokens left.
    */
    pub fn peek(&mut self, amount: usize) -> Option<Vec<Token>> {
        if self.done || amount > self.buffer.len() {
            return None;
        }


        let elements = &self.buffer[0..amount];

        return Some(elements.to_vec());
    }

    /**
       Returns the next n tokens and consumes them.
       If the requested amount is greater than the amount which can be supplied, none is returned, even tough there are tokens left.
    */
    pub fn consume(&mut self, amount: usize) -> Option<Vec<Token>> {
        if self.done || amount > self.buffer.len() {
            return None;
        }


        let elements: Vec<Token> = self.buffer.drain(0..amount).collect();

        if self.buffer.len() == 0 {
            self.done = true;
        }

        return Some(elements);
    }

    /**
       Consumes until the callback returns false. INCLUDES the iteration where false has been returned.
    */
    pub fn consume_until(
        &mut self,
        approve: fn(current: &Token, total: &[Token]) -> bool,
    ) -> Option<Vec<Token>> {
        let mut ret: Vec<Token> = Vec::new();

        loop {
            if self.done {
                break;
            }

            let consumed = self.consume(1);

            if consumed.is_none() {
                break;
            }
            let mut tokens = consumed.unwrap();
            ret.append(&mut tokens);

            if !approve(&ret.last().unwrap(), &ret) {
                break;
            }
        }

        if ret.len() == 0 {
            return None;
        }

        return Some(ret);
    }

    pub fn is_done(&self) -> bool {
        return self.done;
    }
}
