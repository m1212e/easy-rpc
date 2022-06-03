use super::disposeable_comment::DisposeableComment;

pub enum Token {
    DisposeableComment(DisposeableComment),
}

impl From<DisposeableComment> for Token {
    fn from(el: DisposeableComment) -> Self {
        Token::DisposeableComment(el)
    }
}
