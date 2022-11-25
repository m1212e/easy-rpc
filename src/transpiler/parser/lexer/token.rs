use tower_lsp::lsp_types::Range;

use super::{
    disposeable_comment::DisposeableComment, documentational_comment::DocumentationalComment,
    identifier::Identifier, invalid_characters::InvalidCharacters, keyword::Keyword,
    line_break::LineBreak, literal::Literal, operator::Operator,
};

/**
   A token is a lexer result element and represents a specific piece of code which has a meaning.
*/
#[derive(Clone, Debug)]
pub enum Token {
    DisposeableComment(DisposeableComment),
    DocumentationalComment(DocumentationalComment),
    Identifier(Identifier),
    InvalidCharacters(InvalidCharacters),
    Keyword(Keyword),
    LineBreak(LineBreak),
    Literal(Literal),
    Operator(Operator),
}

impl From<DisposeableComment> for Token {
    fn from(el: DisposeableComment) -> Self {
        Token::DisposeableComment(el)
    }
}

impl From<DocumentationalComment> for Token {
    fn from(el: DocumentationalComment) -> Self {
        Token::DocumentationalComment(el)
    }
}

impl From<Identifier> for Token {
    fn from(el: Identifier) -> Self {
        Token::Identifier(el)
    }
}

impl From<InvalidCharacters> for Token {
    fn from(el: InvalidCharacters) -> Self {
        Token::InvalidCharacters(el)
    }
}

impl From<Keyword> for Token {
    fn from(el: Keyword) -> Self {
        Token::Keyword(el)
    }
}

impl From<LineBreak> for Token {
    fn from(el: LineBreak) -> Self {
        Token::LineBreak(el)
    }
}

impl From<Literal> for Token {
    fn from(el: Literal) -> Self {
        Token::Literal(el)
    }
}

impl From<Operator> for Token {
    fn from(el: Operator) -> Self {
        Token::Operator(el)
    }
}

impl Token {
    pub fn range(&self) -> Range {
        match self {
            Token::DisposeableComment(value) => value.range,
            Token::DocumentationalComment(value) => value.range,
            Token::Identifier(value) => value.range,
            Token::InvalidCharacters(value) => value.range,
            Token::Keyword(value) => value.range,
            Token::LineBreak(value) => value.range,
            Token::Literal(value) => value.range,
            Token::Operator(value) => value.range,
        }
    }
}
