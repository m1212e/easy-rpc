use lazy_static::lazy_static;
use regex::Regex;
use std::io::Read;

use crate::transpiler::parser::{
    input_reader::{InputReader, InputReaderError},
    CodeArea, CodePosition,
};

/**
    Literals are fixed values typed out in the source code of various kinds.
    */
#[derive(Clone)]
pub enum LiteralType {
    Boolean(bool),
    String(String),
    Float(f32),
    Integer(i32),
}
#[derive(Clone)]
pub struct Literal {
    literalType: LiteralType,
    start: CodePosition,
    end: CodePosition,
}

impl Literal {
    pub fn lex_literal<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<Literal>, InputReaderError> {
        match process_boolean(reader)? {
            Some(value) => return Ok(Some(value)),
            None => {}
        }

        match process_string(reader)? {
            Some(value) => return Ok(Some(value)),
            None => {}
        }

        match process_number(reader)? {
            Some(value) => return Ok(Some(value)),
            None => {}
        }

        Ok(None)
    }

    pub fn get_type(&self) -> &LiteralType {
        &self.literalType
    }
}

fn process_boolean<T: Read>(
    reader: &mut InputReader<T>,
) -> Result<Option<Literal>, InputReaderError> {
    let peek = reader.peek(6)?;

    lazy_static! {
        static ref MATCH_WHITESPACE: Regex = Regex::new(r"\s").unwrap();
    };

    if peek.starts_with("true") {
        let next = peek.chars().nth(4);

        if next.is_none() || MATCH_WHITESPACE.is_match(next.unwrap().to_string().as_str()) {
            let start = reader.get_current_position().clone();
            reader.consume(4)?;
            let end = reader.get_current_position().clone();

            return Ok(Some(Literal {
                literalType: LiteralType::Boolean(true),
                start: start,
                end: end,
            }));
        }
    }

    if peek.starts_with("false") {
        let next = peek.chars().nth(5);

        if next.is_none() || MATCH_WHITESPACE.is_match(next.unwrap().to_string().as_str()) {
            let start = reader.get_current_position().clone();
            reader.consume(5)?;
            let end = reader.get_current_position().clone();

            return Ok(Some(Literal {
                literalType: LiteralType::Boolean(false),
                start: start,
                end: end,
            }));
        }
    }

    Ok(None)
}

fn process_string<T: Read>(
    reader: &mut InputReader<T>,
) -> Result<Option<Literal>, InputReaderError> {
    if reader.peek(1)? != '"'.to_string() {
        return Ok(None);
    }
    let start = reader.get_current_position().clone();
    reader.consume(1)?;

    fn approve(current: char, total: &String) -> bool {
        let mut rev = total.chars().rev();
        let (_, second_last) = (rev.next(), rev.next());
        if current == '"' {
            return second_last.is_some() && second_last.unwrap() == '\\';
        }
        return true;
    }

    let ret = reader.peek_until(approve)?;

    reader.consume(ret.len() + 1)?; //+1 for closing "
    let end = reader.get_current_position().clone();

    Ok(Some(Literal {
        start: start,
        end: end,
        literalType: LiteralType::String(ret.to_string()),
    }))
}

fn process_number<T: Read>(
    reader: &mut InputReader<T>,
) -> Result<Option<Literal>, InputReaderError> {
    lazy_static! {
        static ref MATCH_NUMBER_START: Regex = Regex::new("[0-9-]").unwrap();
        static ref MATCH_NUMBER: Regex = Regex::new(r"[\d\.-]").unwrap();
    };

    if !MATCH_NUMBER_START.is_match(reader.peek(1)?.as_str()) {
        return Ok(None);
    }

    fn approve(current: char, _: &String) -> bool {
        return MATCH_NUMBER.is_match(current.to_string().as_str());
    }

    let potential_number = reader.peek_until(approve)?;

    if potential_number.contains(".") {
        let parsed = potential_number.parse::<f32>();

        if parsed.is_ok() {
            let start = reader.get_current_position().clone();
            reader.consume(potential_number.len())?;
            let end = reader.get_current_position().clone();

            return Ok(Some(Literal {
                start: start,
                end: end,
                literalType: LiteralType::Float(parsed.unwrap()),
            }));
        }
    } else {
        let parsed = potential_number.parse::<i32>();

        if parsed.is_ok() {
            let start = reader.get_current_position().clone();
            reader.consume(potential_number.len())?;
            let end = reader.get_current_position().clone();

            return Ok(Some(Literal {
                start: start,
                end: end,
                literalType: LiteralType::Integer(parsed.unwrap()),
            }));
        }
    }

    return Ok(None);
}

impl CodeArea for Literal {
    fn get_start(&self) -> &CodePosition {
        &self.start
    }

    fn get_end(&self) -> &CodePosition {
        &self.end
    }
}
