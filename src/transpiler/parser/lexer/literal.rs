use lazy_static::lazy_static;
use regex::Regex;
use std::io::Read;
use tower_lsp::lsp_types::Range;

use crate::{
    transpiler::parser::input_reader::{InputReader, InputReaderError},
    unwrap_result_option,
};

/**
Literals are fixed values typed out in the source code of various kinds.
*/
#[derive(Clone, Debug)]
pub enum LiteralType {
    Boolean(bool),
    String(String),
    Float(f32),
    Integer(i32),
}
#[derive(Clone, Debug)]
pub struct Literal {
    pub literal_type: LiteralType,
    pub range: Range,
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
}

fn process_boolean<T: Read>(
    reader: &mut InputReader<T>,
) -> Result<Option<Literal>, InputReaderError> {
    lazy_static! {
        // the same validator pattern that is used to determine the end of identifiers
        static ref VALIDATOR: Regex = Regex::new("^[A-Za-z0-9_]$").unwrap();
    };

    let peek = unwrap_result_option!(reader.peek(4));
    if peek == "true" {
        let next_char = reader.peek(5)?.unwrap_or(String::new()).chars().nth(4);

        // check if the word is over so we can make sure were not detecting a literal
        if next_char.is_none() || !VALIDATOR.is_match(next_char.unwrap().to_string().as_str()) {
            let start = reader.current_position.clone();
            reader.consume(4)?;
            let end = reader.current_position.clone();

            return Ok(Some(Literal {
                literal_type: LiteralType::Boolean(true),
                range: Range { start, end },
            }));
        }
    }

    let peek = unwrap_result_option!(reader.peek(5));
    if peek == "false" {
        let next_char = reader.peek(6)?.unwrap_or(String::new()).chars().nth(5);

        // check if the word is over so we can make sure were not detecting a literal
        if next_char.is_none() || !VALIDATOR.is_match(next_char.unwrap().to_string().as_str()) {
            let start = reader.current_position.clone();
            reader.consume(5)?;
            let end = reader.current_position.clone();

            return Ok(Some(Literal {
                literal_type: LiteralType::Boolean(false),
                range: Range { start, end },
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
    if reader.peek(ret.len() + 1)?.is_none() {
        return Ok(None);
    }

    // consume the starting "
    let start = reader.current_position.clone();
    reader.consume(1)?;

    // consume the string
    let string_content = reader.consume(ret.len() - 1)?.unwrap();

    // consume the closing "
    reader.consume(1)?;
    let end = reader.current_position.clone();

    Ok(Some(Literal {
        range: Range { start, end },
        literal_type: LiteralType::String(string_content),
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

    let potential_number = unwrap_result_option!(reader.peek_until(|current, _| {
        return MATCH_NUMBER.is_match(current.to_string().as_str());
    }));

    if potential_number.contains(".") {
        let parsed = potential_number.parse::<f32>();

        if parsed.is_ok() {
            let start = reader.current_position.clone();
            reader.consume(potential_number.len())?;
            let end = reader.current_position.clone();

            return Ok(Some(Literal {
                range: Range { start, end },
                literal_type: LiteralType::Float(parsed.unwrap()),
            }));
        }
    } else {
        let parsed = potential_number.parse::<i32>();

        if parsed.is_ok() {
            let start = reader.current_position.clone();
            reader.consume(potential_number.len())?;
            let end = reader.current_position.clone();

            return Ok(Some(Literal {
                range: Range { start, end },
                literal_type: LiteralType::Integer(parsed.unwrap()),
            }));
        }
    }

    return Ok(None);
}
