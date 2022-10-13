use crate::{
    cast,
    transpiler::parser::{
        lexer::{identifier::Identifier, operator::OperatorType, token::Token, TokenReader},
        parser::ParseError,
        CodePosition,
    },
};

use super::field_type::{parse_field_type, Type};

#[derive(Debug)]
pub struct Parameter {
    pub optional: bool,
    pub identifier: String,
    pub parameter_type: Type,
}

#[derive(Debug)]
pub struct Endpoint {
    pub start: CodePosition,
    pub end: CodePosition,
    pub documentation: Option<String>,
    pub identifier: String,
    pub role: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
}

impl Endpoint {
    pub fn parse_endpoint(reader: &mut TokenReader) -> Option<Result<Endpoint, ParseError>> {
        /*
            Endpoints always consist of at least 4 tokens:
            1      2           34
            Server endpointName()

            Optionally there could be a documentational comment before the endpoint which is often followed by a newline
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

        if newline_after_doc {
            peeked = reader.peek(6)?;
            peeked = &peeked[2..];
        } else if has_docs {
            peeked = reader.peek(5)?;
            peeked = &peeked[1..];
        }

        match &peeked[0] {
            Token::Identifier(_) => {}
            _ => {
                return None;
            }
        };

        match &peeked[1] {
            Token::Identifier(_) => {}
            _ => return None,
        };

        // check the opening bracket
        match &peeked[2] {
            Token::Operator(value) => match value.operator_type {
                OperatorType::OpenBracket => {}
                _ => return None,
            },
            _ => return None,
        };

        // at this point it's pretty safe that the currently parsed tokens are meant to build an endpoint, therefore we can start consuming
        // we also checked the types/order of the following tokens and can consume them directly, without re-checking

        let start: CodePosition;
        let mut documentation: Option<String> = None;
        let role: String;
        let identifier: String;

        if newline_after_doc {
            let mut consumed = reader.consume(5)?;

            let doc_token = consumed.remove(0);
            start = doc_token.start();
            documentation = Some(cast!(doc_token, Token::DocumentationalComment).content);
            consumed.remove(0); // newline
            role = cast!(consumed.remove(0), Token::Identifier).content;
            identifier = cast!(consumed.remove(0), Token::Identifier).content;
        } else if !newline_after_doc && has_docs {
            let mut consumed = reader.consume(4)?;

            let doc_token = consumed.remove(0);
            start = doc_token.start();
            documentation = Some(cast!(doc_token, Token::DocumentationalComment).content);
            role = cast!(consumed.remove(0), Token::Identifier).content;
            identifier = cast!(consumed.remove(0), Token::Identifier).content;
        } else {
            let mut consumed = reader.consume(3)?;

            let role_token = consumed.remove(0);
            start = role_token.start();
            role = cast!(role_token, Token::Identifier).content;
            identifier = cast!(consumed.remove(0), Token::Identifier).content;
        }

        let mut parameters: Vec<Parameter> = Vec::new();

        loop {
            let peeked = reader.peek(1);
            // in valid cases this is either a parameter token or the closing bracket which at this point is not yet consumed
            if peeked.is_none() {
                return Some(Err(ParseError {
                    start: reader.last_token_code_start,
                    end: reader.last_token_code_end,
                    message: "Expected more tokens for correct endpoint syntax".to_string(),
                }));
            }

            let peeked = &peeked.unwrap()[0];

            match peeked {
                Token::Operator(operator) => match operator.operator_type {
                    OperatorType::CloseBracket => {
                        reader.consume(1);
                        break;
                    }
                    OperatorType::Comma => {
                        reader.consume(1);
                        let next = reader.peek(1);
                        if next.is_some() {
                            match &next.unwrap()[0] {
                                Token::Operator(operator) => match operator.operator_type {
                                    OperatorType::CloseBracket => {
                                        return Some(Err(ParseError {
                                            start: operator.start,
                                            end: operator.end,
                                            message:
                                                "Expected parameters instead of closing bracket"
                                                    .to_string(),
                                        }))
                                    }
                                    _ => {}
                                },
                                Token::LineBreak(_) => {
                                    reader.consume(1);
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            };

            let endpoint = parse_endpoint_parameter(reader);
            if endpoint.is_err() {
                return Some(Err(endpoint.unwrap_err()));
            }
            parameters.push(endpoint.unwrap())
        }

        let mut return_type: Option<Type> = None;
        let return_token = reader.peek(1);

        if return_token.is_some() {
            match return_token.unwrap()[0].to_owned() {
                Token::LineBreak(_) => {}
                _ => {
                    let t = parse_field_type(reader);
                    if t.is_err() {
                        return Some(Err(t.unwrap_err()));
                    }
                    return_type = Some(t.unwrap());
                }
            }
        }

        Some(Ok(Endpoint {
            documentation,
            start,
            end: reader.last_token_code_end,
            identifier,
            parameters,
            return_type,
            role,
        }))
    }
}

fn parse_endpoint_parameter(reader: &mut TokenReader) -> Result<Parameter, ParseError> {
    let peeked = reader.peek(2); // at least 2 tokens for a valid parameter

    if peeked.is_none() {
        return Err(ParseError {
            start: reader.last_token_code_start,
            end: reader.last_token_code_end,
            message: "Not enough tokens to form a valid parameter".to_string(),
        });
    }

    let vec = peeked.unwrap().to_owned();
    let mut peeked = vec.iter();

    let identifier: Identifier = match peeked.next().unwrap() {
        Token::Identifier(_) => {
            cast!(reader.consume(1).unwrap().remove(0), Token::Identifier)
        }
        value => {
            return Err(ParseError {
                start: value.start(),
                end: value.start(),
                message: "Expected parameter identifier".to_string(),
            });
        }
    };

    let optional = match peeked.next().unwrap() {
        Token::Operator(operator) => match operator.operator_type {
            OperatorType::QuestionMark => {
                reader.consume(1);
                true
            }
            _ => {
                return Err(ParseError {
                    start: operator.start,
                    end: operator.end,
                    message: "Unexpected operator. Only ? is valid here.".to_string(),
                })
            }
        },
        _ => false,
    };

    let parameter_type = parse_field_type(reader)?;

    return Ok(Parameter {
        identifier: identifier.content,
        optional,
        parameter_type,
    });
}
