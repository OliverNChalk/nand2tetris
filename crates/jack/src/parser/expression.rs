use std::iter::Peekable;

use crate::parser::ParserError;
use crate::tokenizer::Tokenizer;

#[derive(Debug)]
pub(crate) struct Expression;

impl Expression {
    pub(crate) fn parse<'a>(
        tokenizer: &mut Peekable<Tokenizer<'a>>,
    ) -> Result<Self, ParserError<'a>> {
        todo!()
    }
}
