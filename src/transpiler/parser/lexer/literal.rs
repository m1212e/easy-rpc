use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::Cell, io::Read};

use crate::{
    transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        CodeArea, CodePosition,
    },
    unwrap_result_option,
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
    lazy_static! {
        static ref MATCH_WHITESPACE: Regex = Regex::new(r"\s").unwrap();
    };

    let peek = unwrap_result_option!(reader.peek(4));
    if peek.starts_with("true") {
        let next_char = reader.peek(5)?.unwrap_or(String::new()).chars().nth(4);

        // check if the word is over so we can make sure were not detecting a literal
        if next_char.is_none() || MATCH_WHITESPACE.is_match(next_char.unwrap().to_string().as_str())
        {
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

    let peek = unwrap_result_option!(reader.peek(5));
    if peek.starts_with("false") {
        let next_char = reader.peek(6)?.unwrap_or(String::new()).chars().nth(5);

        // check if the word is over so we can make sure were not detecting a literal
        if next_char.is_none() || MATCH_WHITESPACE.is_match(next_char.unwrap().to_string().as_str())
        {
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
    if unwrap_result_option!(reader.peek(1)) != '"'.to_string() {
        return Ok(None);
    }

    let mut first = true;
    let ret = unwrap_result_option!(reader.peek_until(|current, total| -> bool {
        // Skip the first " which we already know exists
        if first {
            first = false;
            return true;
        }
        let mut rev = total.chars().rev();
        let (_, second_last) = (rev.next(), rev.next());
        if current == '"' {
            return second_last.is_some() && second_last.unwrap() == '\\';
        }
        return true;
    }));


    // Check if the closing " exists or if peek until got cancelled by the end of the reader 
    if reader.peek(ret.len()+1)?.is_none() {
        return Ok(None);
    }

    // consume the starting "
    let start = reader.get_current_position().clone();
    reader.consume(1)?;
    
    // consume the string
    let string_content = reader.consume(ret.len()-1)?.unwrap();

    // consume the closing "
    reader.consume(1)?;
    let end = reader.get_current_position().clone();

    Ok(Some(Literal {
        start: start,
        end: end,
        literalType: LiteralType::String(string_content),
    }))
}

fn process_number<T: Read>(
    reader: &mut InputReader<T>,
) -> Result<Option<Literal>, InputReaderError> {
    lazy_static! {
        static ref MATCH_NUMBER_START: Regex = Regex::new("[0-9-]").unwrap();
        static ref MATCH_NUMBER: Regex = Regex::new(r"[\d\.-]").unwrap();
    };

    if !MATCH_NUMBER_START.is_match(unwrap_result_option!(reader.peek(1)).as_str()) {
        return Ok(None);
    }

    let potential_number = unwrap_result_option!(reader.peek_until(|current, total| {
        return MATCH_NUMBER.is_match(current.to_string().as_str());
    }));

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
