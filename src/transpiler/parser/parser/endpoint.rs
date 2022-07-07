use std::collections::VecDeque;

use crate::transpiler::parser::{
    lexer::{
        identifier::Identifier,
        keyword::{Keyword, KeywordType},
        literal::{Literal, LiteralType},
        operator::OperatorType,
        token::Token,
        TokenReader,
    },
    parser::ParseError,
    CodeArea, CodePosition,
};

struct Parameter {
    optional: bool,
    identifier: String,
    parameter_type: ParameterType,
}

enum ParameterType {
    Primitive(Primitive),
    Enum(Enum),
    Custom(Custom),
}

struct Primitive {
    primitive_type: PrimitiveType,
    array_amount: ArrayAmount,
}

enum ArrayAmount {
    NoArray,
    NoLengthSpecified,
    LengthSpecified(i32),
}

enum PrimitiveType {
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    String,
}

struct Enum {
    values: Vec<Literal>,
}

struct Custom {
    /*
        If this is a list type
        -1: no list, 0: list but no length defined, >=1: the int is the max length
    */
    array_amount: u64,
    identifier: Identifier,
}

pub struct Endpoint {
    start: CodePosition,
    end: CodePosition,
    documentation: Option<String>,
    identifier: String,
    role: String,
    parameters: Vec<Parameter>,
    return_type: Option<ParameterType>,
}

impl Endpoint {
    pub fn parse_endpoint(reader: &mut TokenReader) -> Option<(Endpoint, Vec<ParseError>)> {
        /*
            Endpoints always consist of at least 4 tokens:
            1      2           34
            Server endpointName()

            Optionally there could be a documentational comment before the endpoint which is often followed by a newline
        */
        let mut peeked = reader.peek(6)?;
        let documentation = match &peeked[0] {
            Token::DocumentationalComment(value) => Some(value.get_content()),
            _ => None,
        };

        let newline_after_doc = if documentation.is_some() {
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

        let role = match &peeked[0] {
            Token::Identifier(value) => value.get_content(),
            _ => {
                return None;
            }
        };

        let identifier = match &peeked[1] {
            Token::Identifier(value) => value.get_content(),
            _ => return None,
        };

        // check the opening bracket
        match &peeked[2] {
            Token::Operator(value) => match value.get_type() {
                OperatorType::OpenBracket => {}
                _ => return None,
            },
            _ => return None,
        };

        // at this point it's pretty safe that the currently parsed tokens are meant to build an endpoint
        // thatswhy we can start consuming

        if newline_after_doc {
            reader.consume(5);
        } else if !newline_after_doc && documentation.is_some() {
            reader.consume(4);
        } else {
            reader.consume(3);
        }

        //TODO at this point start parsing the parameters
        // if that errors, skip until the closing ) to enable errors for the rest of the file

        None
    }
}

impl CodeArea for Endpoint {
    fn get_start(&self) -> &CodePosition {
        return &self.start;
    }

    fn get_end(&self) -> &CodePosition {
        return &self.end;
    }
}

fn parse_endpoint_parameter(reader: &mut TokenReader) -> Result<Parameter, ParseError> {
    let peeked = reader.peek(2); // at least 2 tokens for a valid parameter

    if peeked.is_none() {
        return Err(ParseError {
            start: reader.last_token_code_start().clone(),
            end: reader.last_token_code_end().clone(),
            message: "Expected valid parameter".to_string(),
        });
    }

    let vec = peeked.unwrap().to_owned();
    let mut peeked = vec.iter();

    let identifier = match peeked.next().unwrap() {
        Token::Identifier(value) => {
            reader.consume(1);
            value
        }
        value => {
            return Err(ParseError {
                start: value.get_start().clone(),
                end: value.get_end().clone(),
                message: "Expected parameter identifier".to_string(),
            });
        }
    };

    let optional = match peeked.next().unwrap() {
        Token::Operator(operator) => match operator.get_type() {
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
        identifier: identifier.get_content().clone(),
        optional,
        parameter_type,
    });
}

fn parse_endpoint_parameter_type(reader: &mut TokenReader) -> Result<ParameterType, ParseError> {
    let peeked = reader.peek(1);

    if peeked.is_none() {
        return Err(ParseError {
            message: "Expected a parameter type".to_string(),
            start: reader.last_token_code_start().clone(),
            end: reader.last_token_code_end().clone(),
        });
    }

    let peeked = peeked.unwrap();

    return match peeked[0].to_owned() {
        Token::Keyword(value) => parse_primitive_type(reader, value),
        _ => Err(ParseError {
            message: "Expected a parameter type".to_string(),
            start: reader.last_token_code_start().clone(),
            end: reader.last_token_code_end().clone(),
        }),
    };
}

fn parse_primitive_type(
    reader: &mut TokenReader,
    keyword: Keyword,
) -> Result<ParameterType, ParseError> {
    let primitive_type = match keyword.get_type() {
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
                start: keyword.get_start().clone(),
                end: keyword.get_end().clone(),
                message: "Invalid keyword type for primitive type".to_string(),
            })
        }
    };

    return Ok(ParameterType::Primitive(Primitive {
        primitive_type,
        array_amount: parse_array_length(reader)?,
    }));
}

fn parse_array_length(reader: &mut TokenReader) -> Result<ArrayAmount, ParseError> {
    let peeked = reader.peek(2);

    if peeked.is_none() {
        return Ok(ArrayAmount::NoArray);
    }

    let peeked = peeked.unwrap().to_owned();

    let arrayOpened = match &peeked[0] {
        Token::Operator(value) => match value.get_type() {
            OperatorType::SquareOpenBracket => true,
            _ => false,
        },
        _ => false,
    };

    if arrayOpened {
        return Ok(ArrayAmount::NoArray);
    }

    reader.consume(1);

    //TODO this can probably be done more elegantly?
    let mut length_token: Option<Token> = None;
    let mut length_token_counter = 0;

    reader.consume_until(|token| {
        return match &peeked[0] {
            Token::Operator(value) => match value.get_type() {
                OperatorType::SquareCloseBracket => true,
                _ => {
                    length_token = Some(token);
                    length_token_counter += 1;
                    false
                }
            },
            _ => {
                length_token = Some(token);
                length_token_counter += 1;
                false
            }
        };
    });

    let length_token = length_token.unwrap();

    if length_token_counter == 0 {
        return Ok(ArrayAmount::NoLengthSpecified);
    }

    if length_token_counter > 1 {
        return Err(ParseError {
            start: reader.last_token_code_start().clone(),
            end: reader.last_token_code_end().clone(),
            message: "Invalid amount of tokens for specifying array length".to_string(),
        });
    }

    let array_length = match &length_token {
        Token::Literal(literal) => match literal.get_type() {
            LiteralType::Integer(value) => {
                if *value < 1 {
                    return Err(ParseError {
                        start: literal.get_start().clone(),
                        end: literal.get_end().clone(),
                        message: "Invalid array length. Value can't be less than 1".to_string(),
                    });
                }
                value
            }
            _ => {
                return Err(ParseError {
                    start: literal.get_start().clone(),
                    end: literal.get_end().clone(),
                    message: "Integer literal required to specify array length".to_string(),
                });
            }
        },
        token => {
            return Err(ParseError {
                start: token.get_start().clone(),
                end: token.get_end().clone(),
                message: "Integer literal required to specify array length".to_string(),
            });
        }
    };

    return Ok(ArrayAmount::LengthSpecified(*array_length));
}
