use thiserror::Error;

use crate::tokenizer::{SourceToken, TokenizeError};

#[derive(Debug, Error)]
pub(crate) enum ParseError<'a> {
    #[error("Invalid token; err={0}")]
    InvalidToken(#[from] TokenizeError),
    #[error("Unexpected token; token={0:?}")]
    UnexpectedToken(SourceToken<'a>),
    #[error("Unexpected eof")]
    UnexpectedEof,
    #[error("Trailing comma")]
    TrailingComma,
}
