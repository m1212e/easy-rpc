use self::disposeable_comment::DisposeableComment;

use super::lexer::TokenReader;

mod tests;
mod disposeable_comment;
mod endpoint;

pub struct Parser;

impl Parser {
    pub fn run(reader: &mut TokenReader) {
        loop {
            if reader.is_done() {
                break;
            }

            if DisposeableComment::skip_disposeable_comment(reader).is_some() {
                continue;
            }


            if reader.is_done() {
                break;
            }
        }
    }
}