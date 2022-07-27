pub mod input_reader;
pub mod lexer;
pub mod parser;
pub mod generator;

/**
   A position in code defined by line and character.
*/
#[derive(Copy, Clone, Debug)]
pub struct CodePosition {
    line: u16,
    character: u16,
}

impl CodePosition {
    fn zero_initialized() -> CodePosition {
        CodePosition { line: 0, character: 0 }
    }
}