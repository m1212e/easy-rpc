pub mod input_reader;
pub mod lexer;
pub mod parser;

/**
   A position in code defined by line and character.
*/
#[derive(Copy, Clone, Debug)]
pub struct CodePosition {
    line: u16,
    character: u16,
}