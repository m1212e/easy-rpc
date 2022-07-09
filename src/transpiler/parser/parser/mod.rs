use self::disposeable_comment::DisposeableComment;

use super::{lexer::TokenReader, CodePosition};

mod tests;
mod disposeable_comment;
mod endpoint;

#[derive(Debug)]
pub struct ParseError {
    pub start: CodePosition,
    pub end: CodePosition,
    pub message: String
}

pub struct Parser {
    errors: Vec<ParseError>,
}

impl Parser {

    fn new(reader: &mut TokenReader) -> Parser {
        let mut ret = Parser { errors: Vec::new() };
        ret.run(reader);
        return ret;
    }

    fn run(&mut self, reader: &mut TokenReader) {
        loop {
            if reader.done {
                break;
            }

            if DisposeableComment::skip_disposeable_comment(reader).is_some() {
                continue;
            }


            if reader.done {
                break;
            }
        }
    }
}