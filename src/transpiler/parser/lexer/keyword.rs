use strum::IntoEnumIterator;
use strum_macros::{self, Display, EnumIter};

use std::io::Read;

use crate::transpiler::parser::{
    input_reader::{InputReader, InputReaderError},
    CodeArea, CodePosition,
};

/**
    Keywords are predefined words which the parser knows.
 */
#[derive(Display, EnumIter)]
pub enum KeywordType {
    Type,
    Import,
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

pub struct Keyword {
    keyword_type: KeywordType,
    start: CodePosition,
    end: CodePosition,
}

impl Keyword {
    pub fn lex_keyword<T: Read>(
        reader: &mut InputReader<T>,
    ) -> Result<Option<Keyword>, InputReaderError> {
        for keyword_type in KeywordType::iter() {
            let peeked = reader.peek(keyword_type.to_string().len())?;

            if peeked == keyword_type.to_string().to_lowercase() {
                let start = reader.get_current_position().clone();
                reader.consume(peeked.len())?;
                let end = reader.get_current_position().clone();

                return Ok(Some(Keyword {
                    keyword_type,
                    start: start,
                    end: end,
                }));
            }
        }
        Ok(None)
    }

    pub fn get_type(&self) -> &KeywordType {
        return &self.keyword_type;
    }
}

impl CodeArea for Keyword {
    fn get_start(&self) -> &CodePosition {
        &self.start
    }

    fn get_end(&self) -> &CodePosition {
        &self.end
    }
}
