pub mod lexer;
pub mod input_reader;

/**
 * A position in code by line and character.
 */
struct CodePosition {
    line: u16,
    character: u16
}