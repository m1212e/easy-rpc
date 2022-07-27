use self::{disposeable_comment::DisposeableComment, endpoint::Endpoint};

use super::{lexer::TokenReader, CodePosition};

mod disposeable_comment;
mod endpoint;
mod tests;
pub mod field_type;
pub mod custom_type;

#[derive(Debug)]
pub struct ParseError {
    pub start: CodePosition,
    pub end: CodePosition,
    pub message: String,
}

pub struct Parser;

pub struct ParseResult {
    pub endpoints: Vec<Endpoint>
}

impl Parser {
    fn run(reader: &mut TokenReader) -> Result<ParseResult, ParseError> {
        let mut ret = ParseResult{
            endpoints: Vec::new()
        };
        loop {
            if reader.done {
                break;
            }

            if DisposeableComment::skip_disposeable_comment(reader).is_some() {
                continue;
            }

            let ep = Endpoint::parse_endpoint(reader);
            if ep.is_some() {
                let ep = ep.unwrap();
                if ep.is_ok() {
                    ret.endpoints.push(ep.unwrap());
                } else {
                    return Err(ep.unwrap_err());
                }
                continue;
            };

            if reader.done {
                break;
            }
        }

        return Ok(ret);
    }
}
