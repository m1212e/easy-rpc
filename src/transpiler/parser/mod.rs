use tower_lsp::lsp_types::Position;

pub mod input_reader;
pub mod lexer;
pub mod parser;

/**
   A position in code defined by line and character.
*/
#[derive(Copy, Clone, Debug)]
pub struct CodePosition {
    pub line: u32,
    pub character: u32,
}

impl CodePosition {
    // this is used as convenience func for tests which quickly want a null initialized position
    #[allow(dead_code)]
    pub fn zero_initialized() -> CodePosition {
        CodePosition {
            line: 0,
            character: 0,
        }
    }
}

impl Into<Position> for CodePosition {
    fn into(self) -> Position {
        Position {
            line: self.line,
            character: self.character,
        }
    }
}
