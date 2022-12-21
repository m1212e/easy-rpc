use tower_lsp::lsp_types::{Position, Range};

use crate::transpiler::parser::lexer::{operator::OperatorType, token::Token, TokenReader};

use super::ParseError;

#[derive(Debug)]
pub struct Middleware {
    pub range: Range,
    pub identifier: String,
}

impl Middleware {
    pub fn parse_middleware(reader: &mut TokenReader) -> Option<Result<Middleware, ParseError>> {
        let peeked = reader.peek(1)?;

        match &peeked[0] {
            Token::Operator(operator) => match operator.operator_type {
                OperatorType::Ampersand => {}
                _ => {
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let read = reader.consume(2)?;

        let identifier = match &read[1] {
            Token::Identifier(identifier) => identifier,
            _ => {
                return Some(Err(ParseError {
                    message: "Expected identifier after middleware operator".to_string(),
                    range: reader.last_token_range,
                }))
            }
        };

        Some(Ok(Middleware {
            identifier: identifier.content.to_owned(),
            range: Range {
                start: read[0].range().start,
                end: read[1].range().end,
            },
        }))
    }
}
