use crate::transpiler::parser::{CodePosition};

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
    pub fn start(&self) -> CodePosition {
        match self {
            Token::DisposeableComment(value) => value.start,
            Token::DocumentationalComment(value) => value.start,
            Token::Identifier(value) => value.start,
            Token::InvalidCharacters(value) => value.start,
            Token::Keyword(value) => value.start,
            Token::LineBreak(value) => value.start,
            Token::Literal(value) => value.start,
            Token::Operator(value) => value.start,
        }
    }

    pub fn end(&self) -> CodePosition {
        match self {
            Token::DisposeableComment(value) => value.end,
            Token::DocumentationalComment(value) => value.end,
            Token::Identifier(value) => value.end,
            Token::InvalidCharacters(value) => value.end,
            Token::Keyword(value) => value.end,
            Token::LineBreak(value) => value.end,
            Token::Literal(value) => value.end,
            Token::Operator(value) => value.end,
        }
    }
}