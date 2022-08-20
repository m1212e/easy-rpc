use crate::transpiler::parser::lexer::{token::Token, TokenReader};

pub struct DisposeableComment;

impl DisposeableComment {
    pub fn skip_disposeable_comment(reader: &mut TokenReader) -> Option<()> {
        let peeked = reader.peek(1);

        if peeked.is_none() {
            return None;
        }

        match &peeked.unwrap()[0] {
            Token::DisposeableComment(_) => {
                reader.consume(1);
                return Some(());
            }
            _ => {
                return None;
            }
        }
    }
}
