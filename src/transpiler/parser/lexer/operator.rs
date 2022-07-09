use strum::IntoEnumIterator;
use strum_macros::{self, Display, EnumIter};

use std::io::Read;

use crate::{
    transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        CodePosition,
    },
    unwrap_result_option,
};

/**
   Operators are mostly single chars inside the source code used to syntactically indicate various things.
*/
#[derive(Display, EnumIter, Clone)]
pub enum OperatorType {
    #[strum(serialize = "|")]
    Pipe,
    #[strum(serialize = "{")]
    CurlyOpenBracket,
    #[strum(serialize = "}")]
    CurlyCloseBracket,
    #[strum(serialize = "(")]
    OpenBracket,
    #[strum(serialize = ")")]
    CloseBracket,
    #[strum(serialize = "[")]
    SquareOpenBracket,
    #[strum(serialize = "]")]
    SquareCloseBracket,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = "?")]
    QuestionMark,
}
#[derive(Clone)]
pub struct Operator {
    pub operator_type: OperatorType,
    pub start: CodePosition,
    pub end: CodePosition,
}

impl Operator {
    pub fn lex_operator<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<Operator>, InputReaderError> {
        for operator_type in OperatorType::iter() {
            let peeked = reader.peek(operator_type.to_string().len())?;

            if peeked.is_none() {
                continue;
            }

            let peeked = peeked.unwrap();

            if peeked == operator_type.to_string() {
                let start = reader.current_position.clone();
                reader.consume(peeked.len())?;
                let end = reader.current_position.clone();

                return Ok(Some(Operator {
                    operator_type,
                    start,
                    end,
                }));
            }
        }
        Ok(None)
    }
}
