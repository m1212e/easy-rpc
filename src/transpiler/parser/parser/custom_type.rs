use crate::transpiler::parser::{
    lexer::{keyword::KeywordType, operator::OperatorType, token::Token, TokenReader},
    CodePosition,
};

use super::{field_type::Type, ParseError};

#[derive(Debug)]
pub struct Field {
    pub optional: bool,
    pub identifier: String,
    pub parameter_type: Type,
}

#[derive(Debug)]
pub struct CustomType {
    pub start: CodePosition,
    pub end: CodePosition,
    pub documentation: Option<String>,
    pub identifier: String,
    pub fields: Vec<Field>,
}

impl CustomType {
    pub fn parse_custom_type(reader: &mut TokenReader) -> Option<Result<Field, ParseError>> {
        /*
            Custom types always consist of at least 4 tokens:
            1    2        3 4
            type typeName { }

            Optionally there could be a documentational comment before the type which is often followed by a newline
        */
        let mut peeked = reader.peek(4)?;
        let has_docs = match &peeked[0] {
            Token::DocumentationalComment(value) => true,
            _ => false,
        };

        let newline_after_doc = if has_docs {
            peeked = &peeked[1..];

            match peeked[0] {
                Token::LineBreak(_) => {
                    peeked = &peeked[1..];
                    true
                }
                _ => false,
            }
        } else {
            false
        };

        if newline_after_doc {
            peeked = &peeked[2..];
        } else if has_docs {
            peeked = &peeked[1..];
        }

        match &peeked[0] {
            Token::Keyword(keyword) => match keyword.keyword_type {
                KeywordType::Type => {}
                _ => {
                    return None;
                }
            },
            _ => {
                return None;
            }
        }

        if newline_after_doc {
            reader.consume(3);
        } else if has_docs {
            reader.consume(2);
        } else {
            reader.consume(1); // only the type keyword
        }

        let identifier = match reader.consume(1).unwrap().remove(0) {
            Token::Identifier(id) => id,
            value => {
                return Some(Err(ParseError {
                    start: value.start(),
                    end: value.end(),
                    message: "Expected type identifier".to_string(),
                }))
            }
        };

        let open_bracket = reader.consume(1);
        if open_bracket.is_none() {
            return Some(Err(ParseError {
                start: reader.last_token_code_start,
                end: reader.last_token_code_end,
                message: "Expected an opening { for the type".to_string(),
            }));
        }
        match open_bracket.unwrap().remove(0) {
            Token::Operator(operator) => match operator.operator_type {
                OperatorType::CurlyOpenBracket => {}
                _ => {
                    return Some(Err(ParseError {
                        start: operator.start,
                        end: operator.end,
                        message: "Expected {".to_string(),
                    }))
                }
            },
            token => {
                return Some(Err(ParseError {
                    start: token.start(),
                    end: token.end(),
                    message: "Expected {".to_string(),
                }))
            }
        }

        loop {
            //TODO continue
        }

        return None;
    }
}
