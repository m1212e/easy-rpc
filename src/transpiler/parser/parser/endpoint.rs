use tower_lsp::lsp_types::{Position, Range};

use crate::{
    cast,
    transpiler::parser::{
        lexer::{identifier::Identifier, operator::OperatorType, token::Token, TokenReader},
        parser::ParseError,
    },
};

use super::erpc_type::{parse_field_type, Type};

#[derive(Debug)]
pub struct Parameter {
    pub optional: bool,
    pub identifier: String,
    pub parameter_type: Type,
}

#[derive(Debug)]
pub struct Endpoint {
    pub range: Range,
    pub documentation: Option<String>,
    pub identifier: String,
    pub role: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub middleware_identifiers: Vec<String>,
}

impl Endpoint {
    pub fn parse_endpoint(reader: &mut TokenReader) -> Option<Result<Endpoint, ParseError>> {
        /*
           To ensure that the function only detects a hit and starts consuming whenever its very likely that the tokens are meant to build an endpoint,
           we need to work with peeked instead of consumed tokens until the first opening bracket is found.
        */

        let mut token_counter = 0;
        let current_peek = &reader.peek(2)?;

        // check if the first token is a documentational comment
        let documentation = match &current_peek[0] {
            Token::DocumentationalComment(documentation) => {
                token_counter += 1;

                match current_peek[1] {
                    Token::LineBreak(_) => {
                        token_counter += 1;
                    }
                    _ => {}
                }

                Some(documentation.content.to_owned())
            }
            _ => None,
        };

        // collect all middleware identifiers
        let mut middleware_identifiers = Vec::new();
        loop {
            let current_peek = match reader.peek(3 + token_counter) {
                Some(v) => v,
                None => break,
            };

            match &current_peek[0 + token_counter] {
                Token::Operator(operator) => match operator.operator_type {
                    OperatorType::Ampersand => {}
                    _ => break,
                },
                _ => break,
            };

            let middleware_identifier = match &current_peek[1 + token_counter] {
                Token::Identifier(identifier) => identifier.content.to_owned(),
                _ => break,
            };

            match current_peek[2 + token_counter] {
                Token::LineBreak(_) => {}
                _ => break,
            };

            middleware_identifiers.push(middleware_identifier);
            token_counter += 3;
        }

        // now check if all required tokens for a valid endpoint are present
        let current_peek = reader.peek(4 + token_counter)?;
        let role = match &current_peek[0 + token_counter] {
            Token::Identifier(v) => v.content.to_owned(),
            _ => {
                return None;
            }
        };

        let identifier = match &current_peek[1 + token_counter] {
            Token::Identifier(v) => v.content.to_owned(),
            _ => return None,
        };

        // check the opening bracket
        match &current_peek[2 + token_counter] {
            Token::Operator(value) => match value.operator_type {
                OperatorType::OpenBracket => {}
                _ => return None,
            },
            _ => return None,
        };

        // at this point it's pretty safe that the currently parsed tokens are meant to build an endpoint, therefore we can start consuming
        // we also checked the types/order of the following tokens and can consume them directly, without re-checking

        // remember the start position
        let start = reader.peek(1)?[0].range().start;
        // and remove all already processed tokens
        reader.consume(token_counter + 3);

        let mut parameters: Vec<Parameter> = Vec::new();

        loop {
            let peeked = reader.peek(1);
            // in valid cases this is either a parameter token or the closing bracket which at this point is not yet consumed
            if peeked.is_none() {
                return Some(Err(ParseError {
                    range: reader.last_token_range,
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
                                            range: operator.range,
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
            range: Range {
                start,
                end: reader.last_token_range.end,
            },
            identifier,
            parameters,
            return_type,
            role,
            middleware_identifiers,
        }))
    }
}

fn parse_endpoint_parameter(reader: &mut TokenReader) -> Result<Parameter, ParseError> {
    let peeked = reader.peek(2); // at least 2 tokens for a valid parameter

    if peeked.is_none() {
        return Err(ParseError {
            range: reader.last_token_range,
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
                range: value.range(),
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
                    range: operator.range,
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
