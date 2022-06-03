pub mod lexer;
pub mod input_reader;

/**
    A position in code by line and character.
 */
#[derive(Clone)]
struct CodePosition {
    line: u16,
    character: u16
}