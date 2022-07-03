mod tests;

use std::io::Read;

use self::{
    disposeable_comment::DisposeableComment, documentational_comment::DocumentationalComment,
    identifier::Identifier, invalid_characters::InvalidCharacters, keyword::Keyword,
    line_break::LineBreak, literal::Literal, operator::Operator, space::Space, token::Token,
};

use super::{
    input_reader::{InputReader, InputReaderError},
    CodePosition, CodeArea,
};

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
    last_token_code_start: CodePosition,
    last_token_code_end: CodePosition,
}

impl TokenReader {
    /**
       Creates a new token reader.
    */
    pub fn new<T: Read>(reader: InputReader<T>) -> Result<TokenReader, InputReaderError> {
        let mut ret = TokenReader {
            buffer: Vec::new(),
            done: false,
            last_token_code_start: CodePosition {
                line: 0,
                character: 0,
            },
            last_token_code_end: CodePosition {
                line: 0,
                character: 0,
            },
        };

        ret.run(reader)?;
        ret.done = ret.buffer.len() == 0;
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
        Returns a given amount of tokens without consuming them.
        Returns none when the amount requested cant be provided.
    */
    pub fn peek(&mut self, amount: usize) -> Option<&[Token]> {
        if self.done || amount > self.buffer.len() {
            return None;
        }

        let elements = &self.buffer[0..amount];

        return Some(elements);
    }

    /**
       Consumes a given amount of tokens.
       Returns none when the amount requested cant be provided. In this case no tokens are consumed at all.
    */
    pub fn consume(&mut self, amount: usize) -> Option<Vec<Token>> {
        if self.done || amount > self.buffer.len() {
            return None;
        }

        let elements: Vec<Token> = self.buffer.drain(0..amount).collect();

        self.last_token_code_start = elements.last().unwrap().get_start().clone();
        self.last_token_code_end = elements.last().unwrap().get_end().clone();

        if self.buffer.len() == 0 {
            self.done = true;
        }

        return Some(elements);
    }

    /**
        Consumes chars until the provided approve function returns false.
        The iteration where the approve function fails (returns false) is INCLUSIVE, the current value will be returned.
        Returns None if no char could be consumed.
    */
    pub fn consume_until<F: FnMut(Token) -> bool>(&mut self, mut approve: F) {
        loop {
            if self.done {
                break;
            }

            let consumed = self.consume(1);

            if consumed.is_none() {
                break;
            }

            if !approve(consumed.unwrap()[0]) {
                break;
            }
        }
    }

    pub fn last_token_code_start(&self)-> &CodePosition {
        return &self.last_token_code_start;
    }
    
    pub fn last_token_code_end(&self)-> &CodePosition {
        return &self.last_token_code_end;
    }

    pub fn is_done(&self) -> bool {
        return self.done;
    }
}
