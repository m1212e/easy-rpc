use strum::IntoEnumIterator;
use strum_macros::{self, Display, EnumIter};

use std::io::Read;

use crate::{
    transpiler::parser::{
        input_reader::{InputReader, InputReaderError},
        CodePosition,
    },
};

/**
   Keywords are predefined words which the parser knows.
*/
#[derive(Display, EnumIter, Clone, Debug)]
pub enum KeywordType {
    Type,
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    String,
    Int,
    Float,
}

#[derive(Clone, Debug)]
pub struct Keyword {
    pub keyword_type: KeywordType,
    pub start: CodePosition,
    pub end: CodePosition,
}

impl Keyword {
    pub fn lex_keyword<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<Keyword>, InputReaderError> {
        for keyword_type in KeywordType::iter() {
            let peeked = reader.peek(keyword_type.to_string().len())?;

            if peeked.is_none() {
                continue;
            }

            let peeked = peeked.unwrap();

            if peeked == keyword_type.to_string().to_lowercase() {
                let start = reader.current_position.clone();
                reader.consume(peeked.len())?;
                let end = reader.current_position.clone();

                return Ok(Some(Keyword {
                    keyword_type,
                    start: start,
                    end: end,
                }));
            }
        }
        Ok(None)
    }

}
