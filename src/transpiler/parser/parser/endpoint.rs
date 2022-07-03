use crate::transpiler::parser::{
    lexer::{
        identifier::Identifier, literal::Literal, operator::OperatorType, token::Token, TokenReader,
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
    /*
        If this is a list type
        -1: no list, 0: list but no length defined, >=1: the int is the max length
    */
    array_amount: u64,
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

    let peeked = peeked.unwrap();

    let mut identifier = match peeked[0] {
        Token::Identifier(value) => {
            reader.consume(1);
            value
        }
        _ => {
            return Err(ParseError {
                start: peeked[0].get_start().clone(),
                end: peeked[0].get_end().clone(),
                message: "Expected parameter identifier".to_string(),
            });
        }
    };

    let optional = match peeked[1] {
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

    return Ok(Parameter{
        identifier: *identifier.get_content(),
        optional,
        parameter_type
    });
}

fn parse_endpoint_parameter_type(
    reader: &mut TokenReader,
) -> Result<ParameterType, ParseError> {
    let peeked = reader.peek(1);

    if peeked.is_none() {
        return Err(ParseError{
            message: "Expected a parameter type".to_string(),
            start: reader.last_token_code_start().clone(),
            end: reader.last_token_code_end().clone()
        })
    }

    let peeked = peeked.unwrap();

    match &peeked[0] {
        Token::Keyword(value) => {

        },
        _ => {todo!()}
    }

    return Err(ParseError{
        message: "Expected a parameter type".to_string(),
        start: reader.last_token_code_start().clone(),
        end: reader.last_token_code_end().clone()
    })
}
