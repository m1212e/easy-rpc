mod tests;

use super::CodePosition;
use std::io::{BufRead, BufReader, Read};

/*
    Wrapper around an input which provides methods and functionality for processing text.
    Can be used to provide input for the lexer.
*/
pub struct InputReader<T: Read> {
    buffer: String,
    done: bool,
    current_position: CodePosition,
    reader: BufReader<T>,
}

/**
   An error which can occur while using the input reader.
*/
#[derive(Debug)]
pub enum InputReaderError {
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
}

impl From<std::io::Error> for InputReaderError {
    fn from(err: std::io::Error) -> Self {
        InputReaderError::Io(err)
    }
}
impl From<std::str::Utf8Error> for InputReaderError {
    fn from(err: std::str::Utf8Error) -> Self {
        InputReaderError::Utf8(err)
    }
}

impl<T: Read> InputReader<T> {
    /**
       Creates a new input reader.
    */
    pub fn new(input: T) -> InputReader<T> {
        InputReader {
            buffer: String::new(),
            done: false,
            current_position: CodePosition {
                line: 0,
                character: 0,
            },
            reader: BufReader::new(input),
        }
    }

    /**
       Consumes a given amount of characters from the input source and returns them.
       Returns none when the amount requested cant be provided and no chars are consumed.
    */
    pub fn consume(&mut self, amount: usize) -> Result<Option<String>, InputReaderError> {
        if self.is_done() {
            return Ok(None);
        }

        self.extend_buffer_by(amount)?;
        let read: String = self.buffer.chars().take(amount).collect();

        if read.len() < amount {
            return Ok(None);
        }

        self.buffer = self.buffer.split_at(read.as_bytes().len()).1.to_string();

        let amount_of_newlines = read.as_bytes().iter().filter(|&&c| c == b'\n').count();
        self.current_position.line += amount_of_newlines as u16;

        if amount_of_newlines == 0 {
            self.current_position.character += read.chars().count() as u16;
        } else {
            self.current_position.character = read.lines().last().unwrap().chars().count() as u16;
        }

        return Ok(Some(read));
    }

    /**
       Consumes chars until a specific string is met. The delimeter is INCLUSIVE and the cursor will be positioned BEHIND the delimeter after execution.
        If the delimeter can't be found, the reader will be consumed until the end.
    */
    pub fn consume_until_or_end(
        &mut self,
        delimeter: &str,
    ) -> Result<Option<String>, InputReaderError> {
        if self.is_done() {
            return Ok(None);
        }

        let mut ret = String::new();

        loop {
            if self.is_done() {
                break;
            }

            let read = self.consume(1)?;
            if read.is_none() {
                break;
            }
            ret.push_str(&read.unwrap());

            if ret.ends_with(delimeter) {
                break;
            }
        }

        if ret.len() > 0 {
            return Ok(Some(ret));
        }
        return Ok(None);
    }

    /**
       Returns the requested amount of chars as string, without consuming them.
       Returns none if the requested amount can't be provided.
    */
    pub fn peek(&mut self, mut amount: usize) -> Result<Option<String>, InputReaderError> {
        if self.is_done() {
            return Ok(None);
        }

        self.extend_buffer_by(amount)?;

        if amount > self.buffer.len() {
            return Ok(None);
        }

        return Ok(Some(self.buffer.chars().take(amount).collect()));
    }

    /**
       Peeks char by char and calls the approve callback function with that char and the total
       peeked chars (including current) as params.
       Returns all approved chars as string EXCLUDING the one where the approve failed.
    */
    pub fn peek_until(
        &mut self,
        approve: fn(current: char, total: &String) -> bool,
    ) -> Result<Option<String>, InputReaderError> {
        let mut offset = 0;

        let mut peeked = String::new();

        loop {
            offset += 1;
            let peek_result = self.peek(offset)?;
            if peek_result.is_none() {
                if peeked.len() > 0 {
                    return Ok(Some(peeked));
                }
                return Ok(None);
            }
            peeked = peek_result.unwrap();

            let mut chars = peeked.chars();
            let last_char = chars.next_back();
            if !approve(last_char.unwrap(), &peeked) {
                return Ok(Some(chars.as_str().to_string()));
            }
        }
    }

    /**
       Checks if the reader has chars left.
    */
    pub fn is_done(&mut self) -> bool {
        // since the self.done is only set AFTER the EOF was detected, try to fill the buffer to detect a possible EOF and return a correct result
        let _ = self.extend_buffer_by(1);
        return self.done && self.buffer.len() == 0;
    }

    /**
       Returns the current position of the reader.
    */
    pub fn get_current_position(&self) -> &CodePosition {
        return &self.current_position;
    }

    /**
        Extends the buffer until it contains minimum n characters or the end is reached.
    */
    fn extend_buffer_by(&mut self, amount: usize) -> Result<(), InputReaderError> {
        let mut buf: Vec<u8> = Vec::new();
        let mut current_buffer_length = self.buffer.len();

        while current_buffer_length < amount {
            let amount_read = self.reader.read_until(b'\n', &mut buf)?;

            if amount_read == 0 {
                self.done = true;
                break;
            }

            let parsed_string = std::str::from_utf8(&mut buf)?;
            self.buffer.push_str(parsed_string);
            buf.clear();

            current_buffer_length += amount_read;
        }

        Ok(())
    }
}
