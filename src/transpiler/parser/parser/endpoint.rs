use crate::{
    cast,
    transpiler::parser::{
        lexer::{
            identifier::Identifier,
            keyword::{Keyword, KeywordType},
            literal::{Literal, LiteralType},
            operator::OperatorType,
            token::Token,
            TokenReader,
        },
        parser::ParseError,
        CodePosition,
    },
};

#[derive(Debug)]
pub struct Parameter {
    pub optional: bool,
    pub identifier: String,
    pub parameter_type: ParameterType,
}

#[derive(Debug)]
pub enum ParameterType {
    Primitive(Primitive),
    Enum(Enum),
    Custom(Custom),
}

#[derive(Debug)]
pub struct Primitive {
    pub primitive_type: PrimitiveType,
    pub array_amount: ArrayAmount,
}

#[derive(Debug)]
pub enum ArrayAmount {
    NoArray,
    NoLengthSpecified,
    LengthSpecified(i32),
}

#[derive(Debug)]
pub enum PrimitiveType {
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    String,
}

#[derive(Debug)]
pub struct Enum {
    pub values: Vec<Literal>,
}

#[derive(Debug)]
pub struct Custom {
    /*
        If this is a list type
        -1: no list, 0: list but no length defined, >=1: the int is the max length
    */
    pub array_amount: u64,
    pub identifier: Identifier,
}

pub struct Endpoint {
    pub start: CodePosition,
    pub end: CodePosition,
    pub documentation: Option<String>,
    pub identifier: String,
    pub role: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<ParameterType>,
}

impl Endpoint {
    /**
       Parses an endpoint and consumes the reader accordingly. Only returns some if the function is confident that the currently
       parsed tokens are meant to be an endpoint. Returns either a correctly parsed endpoint or an error which describes
       what cancelled the process.
    */
    pub fn parse_endpoint(reader: &mut TokenReader) -> Option<Result<Endpoint, ParseError>> {
        /*
            Endpoints always consist of at least 4 tokens:
            1      2           34
            Server endpointName()

            Optionally there could be a documentational comment before the endpoint which is often followed by a newline
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
            peeked = reader.peek(6)?;
        } else if has_docs {
            peeked = reader.peek(5)?;
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

        let mut start: CodePosition;
        let mut documentation: Option<String> = None;
        let mut role: String;
        let mut identifier: String;

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
                        let close_check = reader.peek(1);
                        if close_check.is_some() {
                            match &close_check.unwrap()[0] {
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

        Some(Ok(Endpoint {
            documentation,
            start,
            end: reader.last_token_code_end,
            identifier,
            parameters,
            return_type: None, //TODO this is for testing only, obviously this needs to be parsed
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
            message: "Expected valid parameter".to_string(),
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
            _ => false,
        },
        _ => false,
    };

    let parameter_type = parse_endpoint_parameter_type(reader)?;

    return Ok(Parameter {
        identifier: identifier.content,
        optional,
        parameter_type,
    });
}

fn parse_endpoint_parameter_type(reader: &mut TokenReader) -> Result<ParameterType, ParseError> {
    let peeked = reader.peek(1);

    if peeked.is_none() {
        return Err(ParseError {
            message: "Expected a parameter type".to_string(),
            start: reader.last_token_code_start,
            end: reader.last_token_code_end,
        });
    }

    let peeked = peeked.unwrap();

    return match &peeked[0].to_owned() {
        Token::Keyword(value) => parse_primitive_type(reader, value),
        Token::Literal(_) => parse_literal_type(reader),
        _ => Err(ParseError {
            message: "Expected a parameter type".to_string(),
            start: reader.last_token_code_start,
            end: reader.last_token_code_end,
        }),
    };
}

fn parse_primitive_type(
    reader: &mut TokenReader,
    keyword: &Keyword,
) -> Result<ParameterType, ParseError> {
    let primitive_type = match keyword.keyword_type {
        KeywordType::Boolean => PrimitiveType::Boolean,
        KeywordType::Int8 => PrimitiveType::Int8,
        KeywordType::Int16 => PrimitiveType::Int16,
        KeywordType::Int32 => PrimitiveType::Int32,
        KeywordType::Int64 => PrimitiveType::Int64,
        KeywordType::Float32 => PrimitiveType::Float32,
        KeywordType::Float64 => PrimitiveType::Float64,
        KeywordType::String => PrimitiveType::String,
        KeywordType::Int => PrimitiveType::Int16,
        KeywordType::Float => PrimitiveType::Float32,
        _ => {
            return Err(ParseError {
                start: keyword.start,
                end: keyword.start,
                message: "Invalid keyword for primitive type".to_string(),
            })
        }
    };

    reader.consume(1);

    return Ok(ParameterType::Primitive(Primitive {
        primitive_type,
        array_amount: parse_array_length(reader)?,
    }));
}

fn parse_literal_type(reader: &mut TokenReader) -> Result<ParameterType, ParseError> {
    let mut values: Vec<Literal> = Vec::new();
    loop {
        let token = reader.consume(1);
        if token.is_none() {
            return Err(ParseError {
                start: reader.last_token_code_start,
                end: reader.last_token_code_end,
                message: "Expected a literal for this enum type".to_string(),
            });
        }
        let token = token.unwrap().remove(0);

        match token {
            Token::Literal(literal) => values.push(literal),
            _ => {
                return Err(ParseError {
                    start: token.start(),
                    end: token.end(),
                    message: "Expected literal token".to_string(),
                })
            }
        };

        let next = reader.peek(1);

        if next.is_none() {
            break;
        }

        match &next.unwrap()[0] {
            Token::Operator(operator) => match operator.operator_type {
                OperatorType::Pipe => {
                    reader.consume(1);
                }
                _ => {
                    break;
                }
            },
            _ => {
                break;
            }
        };
    }

    return Ok(ParameterType::Enum(Enum { values }));
}

fn parse_array_length(reader: &mut TokenReader) -> Result<ArrayAmount, ParseError> {
    let peeked = reader.peek(2);

    if peeked.is_none() {
        return Ok(ArrayAmount::NoArray);
    }

    let peeked = peeked.unwrap().to_owned();

    let array_opened = match &peeked[0] {
        Token::Operator(value) => match value.operator_type {
            OperatorType::SquareOpenBracket => true,
            _ => false,
        },
        _ => false,
    };

    if !array_opened {
        return Ok(ArrayAmount::NoArray);
    }

    reader.consume(1);

    let length_token = reader.consume(1);
    if length_token.is_none() {
        return Err(ParseError {
            start: reader.last_token_code_start,
            end: reader.last_token_code_end,
            message: "Expected token to complete the length definition of this array".to_string(),
        });
    }

    let length = match length_token.unwrap().remove(0) {
        Token::Operator(operator) => match operator.operator_type {
            OperatorType::SquareCloseBracket => {
                return Ok(ArrayAmount::NoLengthSpecified);
            }
            _ => {
                return Err(ParseError {
                    start: operator.start,
                    end: operator.end,
                    message: "Expected integer or closing bracket".to_string(),
                })
            }
        },
        Token::Literal(literal) => match literal.literal_type {
            LiteralType::Integer(integer) => integer,
            _ => {
                return Err(ParseError {
                    start: literal.start,
                    end: literal.end,
                    message: "Expected integer or closing bracket".to_string(),
                })
            }
        },
        token => {
            return Err(ParseError {
                start: token.start(),
                end: token.end(),
                message: "Expected integer or closing bracket".to_string(),
            })
        }
    };

    if length < 1 {
        return Err(ParseError {
            start: reader.last_token_code_start,
            end: reader.last_token_code_end,
            message: "Size of the array must be above or equal to 1".to_string(),
        });
    }

    reader.consume(1);

    return Ok(ArrayAmount::LengthSpecified(length));
}
