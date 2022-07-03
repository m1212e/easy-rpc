pub mod input_reader;
pub mod lexer;
pub mod parser;

/**
   A position in code defined by line and character.
*/
#[derive(Copy, Clone, Debug)]
struct CodePosition {
    line: u16,
    character: u16,
}

/**
    A CodeArea marks a location inside some source code. It's defined through a start and an end.
*/
trait CodeArea {
    fn get_start(&self) -> &CodePosition;

    fn get_end(&self) -> &CodePosition;
}
