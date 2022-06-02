mod tests;

use super::CodePosition;
use std::io::{BufRead, BufReader, Read};

pub struct InputReader<T: Read> {
    buffer: String,
    done: bool,
    current_position: CodePosition,
    reader: BufReader<T>,
}

#[derive(Debug)]
pub enum InputReaderError {
    IO(std::io::Error),
    UTF8(std::str::Utf8Error),
    DONE,
}

impl From<std::io::Error> for InputReaderError {
    fn from(err: std::io::Error) -> Self {
        InputReaderError::IO(err)
    }
}
impl From<std::str::Utf8Error> for InputReaderError {
    fn from(err: std::str::Utf8Error) -> Self {
        InputReaderError::UTF8(err)
    }
}

impl<T: Read> InputReader<T> {
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

    pub fn consume(&mut self, amount: usize) -> Result<String, InputReaderError> {
        if self.done {
            return Err(InputReaderError::DONE);
        }

        self.extend_buffer_by(amount)?;
        let read: String = self.buffer.chars().take(amount).collect();
        self.buffer = self.buffer.split_at(read.as_bytes().len()).1.to_string();

        let amount_of_newlines = read.as_bytes().iter().filter(|&&c| c == b'\n').count();
        self.current_position.line += amount_of_newlines as u16;

        if amount_of_newlines == 0 {
            self.current_position.character += read.chars().count() as u16;
        } else {
            self.current_position.character = read.lines().last().unwrap().chars().count() as u16;
        }

        return Ok(read);
    }

    pub fn peek(&mut self, amount: usize) -> Result<String, InputReaderError> {
        if self.done {
            return Err(InputReaderError::DONE);
        }

        self.extend_buffer_by(amount)?;

        return Ok(self.buffer.chars().take(amount).collect());
    }

    /**
        Extends the buffer until it contains minimum n characters.
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
