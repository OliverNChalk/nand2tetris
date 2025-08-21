use std::iter::Peekable;

use crate::tokenizer::{Token, Tokenizer};

pub(crate) fn peek_token(tokenizer: &mut Peekable<&mut Tokenizer>, expected: Token) -> bool {
    let Some(Ok(st)) = tokenizer.peek() else {
        return false;
    };

    st.token == expected
}

macro_rules! eat {
    ($tokenizer:expr, $expected:pat) => {{
        let $crate::tokenizer::SourceToken { source, token } = $tokenizer.next().unwrap()?;
        if !matches!(token, $expected) {
            return Err(ParserError::UnexpectedToken($crate::tokenizer::SourceToken {
                source,
                token,
            }));
        }

        Ok::<_, ParserError>(source)
    }};
}

pub(crate) use eat;
