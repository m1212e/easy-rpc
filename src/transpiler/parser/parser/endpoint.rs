use crate::transpiler::parser::{
    lexer::{identifier::Identifier, literal::Literal, TokenReader},
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
    documentation: String,
    identifier: String,
    role: String,
    parameters: Vec<Parameter>,
    return_type: Option<ParameterType>,
}

impl Endpoint {
    pub fn parse_endpoint(reader: &mut TokenReader) -> Option<Endpoint> {
        /*
            Endpoints always consist of at least 4 tokens:
            1      2           34
            Server endpointName()
        */
        let peeked = reader.peek(4)?;

        // If there are not enough tokens left

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
