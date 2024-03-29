use tower_lsp::lsp_types::{Position, Range};

use crate::{
    cast,
    transpiler::parser::lexer::{
        keyword::KeywordType, operator::OperatorType, token::Token, TokenReader,
    },
};

use super::{
    erpc_type::{parse_field_type, Type},
    ParseError,
};

#[derive(Debug)]
pub struct Field {
    pub optional: bool,
    pub identifier: String,
    pub field_type: Type,
    pub documentation: Option<String>,
}

#[derive(Debug)]
pub struct CustomType {
    pub range: Range,
    pub documentation: Option<String>,
    pub identifier: String,
    pub fields: Vec<Field>,
}

impl CustomType {
    pub fn parse_custom_type(reader: &mut TokenReader) -> Option<Result<CustomType, ParseError>> {
        /*
            Custom types always consist of at least 4 tokens:
            1    2        3 4
            type typeName { }

            Optionally there could be a documentational comment before the type which is often followed by a newline
        */
        let mut peeked = reader.peek(4)?;
        let has_docs = match &peeked[0] {
            Token::DocumentationalComment(_) => true,
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

        let start: Position;
        let mut documentation: Option<String> = None;

        if newline_after_doc {
            let mut consumed = reader.consume(3).unwrap();
            let doc_token = consumed.remove(0);
            start = doc_token.range().start;
            documentation = Some(cast!(doc_token, Token::DocumentationalComment).content);
        } else if has_docs {
            let mut consumed = reader.consume(2).unwrap();
            let doc_token = consumed.remove(0);
            start = doc_token.range().start;
            documentation = Some(cast!(doc_token, Token::DocumentationalComment).content);
        } else {
            let t = reader.consume(1).unwrap().remove(0); // only the type keyword
            start = t.range().start;
        }

        let identifier = match reader.consume(1).unwrap().remove(0) {
            Token::Identifier(id) => id,
            value => {
                return Some(Err(ParseError {
                    range: value.range(),
                    message: "Expected type identifier".to_string(),
                }))
            }
        };

        let open_bracket = reader.consume(1);
        if open_bracket.is_none() {
            return Some(Err(ParseError {
                range: reader.last_token_range,
                message: "Expected an opening { for the type".to_string(),
            }));
        }
        match open_bracket.unwrap().remove(0) {
            Token::Operator(operator) => match operator.operator_type {
                OperatorType::CurlyOpenBracket => {}
                _ => {
                    return Some(Err(ParseError {
                        range: operator.range,
                        message: "Expected {".to_string(),
                    }))
                }
            },
            token => {
                return Some(Err(ParseError {
                    range: token.range(),
                    message: "Expected {".to_string(),
                }))
            }
        }

        let mut fields: Vec<Field> = Vec::new();

        loop {
            let next = reader.peek(1);
            if next.is_none() {
                return Some(Err(ParseError {
                    range: reader.last_token_range,
                    message: "Expected closing }".to_string(),
                }));
            }
            let next = next.unwrap();
            match &next[0].to_owned() {
                Token::Operator(operator) => match operator.operator_type {
                    OperatorType::CurlyCloseBracket => {
                        reader.consume(1);
                        break;
                    }
                    _ => {}
                },
                Token::LineBreak(_) => {
                    reader.consume(1);
                    continue;
                }
                _ => {}
            }

            let peeked = reader.peek(2);
            if peeked.is_none() {
                // should never occur
                return Some(Err(ParseError {
                    range: reader.last_token_range,
                    message: "Expected more tokens for correct type body".to_string(),
                }));
            }
            let peeked = peeked.unwrap().to_owned();

            let mut documentation: Option<String> = None;
            match peeked[0] {
                Token::DocumentationalComment(_) => {
                    documentation = Some(
                        cast!(
                            reader.consume(1).unwrap().remove(0),
                            Token::DocumentationalComment
                        )
                        .content,
                    );
                    match peeked[1] {
                        Token::LineBreak(_) => {
                            reader.consume(1);
                        }
                        _ => {}
                    }
                }
                _ => {}
            };

            let next = reader.consume(1);
            if next.is_none() {
                // this should never occur
                return Some(Err(ParseError {
                    range: reader.last_token_range,
                    message: "Expected identifier for field".to_string(),
                }));
            }
            let next = next.unwrap().remove(0);

            let identifier = match next {
                Token::Identifier(id) => id,
                token => {
                    return Some(Err(ParseError {
                        range: token.range(),
                        message: "Expected field identifier".to_string(),
                    }));
                }
            };

            let next = reader.peek(1);
            if next.is_none() {
                // should never occur
                return Some(Err(ParseError {
                    range: reader.last_token_range,
                    message: "Expected more tokens for valid field".to_string(),
                }));
            }
            let next = &next.unwrap()[0];

            let optional = match next {
                Token::Operator(operator) => match operator.operator_type {
                    OperatorType::QuestionMark => {
                        reader.consume(1);
                        true
                    }
                    _ => false,
                },
                _ => false,
            };

            let field_type = parse_field_type(reader);
            if field_type.is_err() {
                return Some(Err(field_type.unwrap_err()));
            }

            fields.push(Field {
                documentation,
                identifier: identifier.content,
                optional,
                field_type: field_type.unwrap(),
            })
        }

        return Some(Ok(CustomType {
            range: Range {
                start,
                end: reader.last_token_range.end,
            },
            documentation,
            identifier: identifier.content,
            fields,
        }));
    }
}
