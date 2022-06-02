use std::{io::{BufReader, BufRead, Error}, fs::File};

use crate::transpiler::parser::CodePosition;

struct DisposeableComment {
    content: String,
    start: CodePosition,
    end: CodePosition
}

fn lex_disposeable_comment(reader: &mut BufReader<File>) -> Result<Option<DisposeableComment>, Error> {

    Ok(None)
}