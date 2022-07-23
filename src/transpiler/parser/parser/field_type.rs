use crate::transpiler::parser::{
    lexer::{
        keyword::KeywordType,
        literal::{Literal, LiteralType},
        operator::OperatorType,
        token::Token,
        TokenReader,
    },
    parser::ParseError,
};

#[derive(Debug)]
pub enum Type {
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
    pub array_amount: ArrayAmount,
    pub identifier: String,
}

pub fn parse_field_type(reader: &mut TokenReader) -> Result<Type, ParseError> {
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
        Token::Keyword(_) => parse_primitive_type(reader),
        Token::Literal(_) => parse_enum_type(reader),
        Token::Identifier(_) => parse_custom_type(reader),
        _ => Err(ParseError {
            message: "Expected a parameter type".to_string(),
            start: reader.last_token_code_start,
            end: reader.last_token_code_end,
        }),
    };
}

fn parse_primitive_type(reader: &mut TokenReader) -> Result<Type, ParseError> {
    let primitive_type = match reader.consume(1).unwrap().remove(0) {
        Token::Keyword(keyword) => match keyword.keyword_type {
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
                // should not occur
                return Err(ParseError {
                    start: keyword.start,
                    end: keyword.start,
                    message: "Invalid keyword for primitive type".to_string(),
                })
            }
        },
        token => {
            // should not occur
            return Err(ParseError {
                start: token.start(),
                end: token.start(),
                message: "Invalid token for primitive type".to_string(),
            })
        }
    };

    return Ok(Type::Primitive(Primitive {
        primitive_type,
        array_amount: parse_array_length(reader)?,
    }));
}

fn parse_enum_type(reader: &mut TokenReader) -> Result<Type, ParseError> {
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

    return Ok(Type::Enum(Enum { values }));
}

fn parse_custom_type(reader: &mut TokenReader) -> Result<Type, ParseError> {
    let identifier = match reader.consume(1).unwrap().remove(0) {
        Token::Identifier(id) => id,
        token => {
            // should not occur
            return Err(ParseError {
                start: token.start(),
                end: token.end(),
                message: "Invalid token for custom type".to_string(),
            })
        }
    };

    return Ok(Type::Custom(Custom {
        identifier: identifier.content,
        array_amount: parse_array_length(reader)?,
    }));
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
