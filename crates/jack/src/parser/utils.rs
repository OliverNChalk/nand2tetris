use crate::parser::error::ParserError;
use crate::tokenizer::{Token, Tokenizer};

pub(crate) fn next<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Token, ParserError<'a>> {
    Ok(tokenizer.next().ok_or(ParserError::UnexpectedEof)??.token)
}

pub(crate) fn peek(tokenizer: &mut Tokenizer) -> Option<Token> {
    tokenizer
        .peek_0()
        .and_then(|res| res.ok().map(|st| st.token))
}

pub(crate) fn check_next(tokenizer: &mut Tokenizer, expected: Token) -> bool {
    let Some(Ok(st)) = tokenizer.peek_0() else {
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
