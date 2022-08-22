use crate::transpiler::parser::lexer::{token::Token, TokenReader};

pub struct LineBreak;

impl LineBreak {
    pub fn skip_line_break(reader: &mut TokenReader) -> Option<()> {
        let peeked = reader.peek(1);

        if peeked.is_none() {
            return None;
        }

        match &peeked.unwrap()[0] {
            Token::LineBreak(_) => {
                reader.consume(1);
                return Some(());
            }
            _ => {
                return None;
            }
        }
    }
}
