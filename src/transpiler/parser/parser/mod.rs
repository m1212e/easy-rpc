use self::{custom_type::CustomType, disposeable_comment::DisposeableComment, endpoint::Endpoint, line_break::LineBreak};

use super::{lexer::TokenReader, CodePosition};

pub mod custom_type;
mod disposeable_comment;
mod line_break;
pub mod endpoint;
pub mod field_type;
mod tests;

#[derive(Debug)]
pub struct ParseError {
    pub start: CodePosition,
    pub end: CodePosition,
    pub message: String,
}

pub struct ParseResult {
    pub endpoints: Vec<Endpoint>,
    pub custom_types: Vec<CustomType>,
}

pub fn parse(reader: &mut TokenReader) -> Result<ParseResult, ParseError> {
    let mut ret = ParseResult {
        endpoints: Vec::new(),
        custom_types: Vec::new(),
    };
    loop {
        if reader.done {
            break;
        }

        if DisposeableComment::skip_disposeable_comment(reader).is_some() {
            continue;
        }

        if LineBreak::skip_line_break(reader).is_some() {
            continue;
        }

        match Endpoint::parse_endpoint(reader) {
            Some(result) => match result {
                Ok(endpoint) => {
                    ret.endpoints.push(endpoint);
                    continue;
                }
                Err(err) => {
                    return Err(err);
                }
            },
            None => {}
        }

        match CustomType::parse_custom_type(reader) {
            Some(result) => match result {
                Ok(custom_type) => {
                    ret.custom_types.push(custom_type);
                    continue;
                }
                Err(err) => {
                    return Err(err);
                }
            },
            None => {}
        }

        if reader.done {
            break;
        }
    }

    return Ok(ret);
}
