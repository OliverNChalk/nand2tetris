use std::iter::Peekable;

use crate::parser::expression::Expression;
use crate::parser::utils::{self, eat, peek_token};
use crate::parser::ParserError;
use crate::tokenizer::{Keyword, Symbol, Token, Tokenizer};

#[derive(Debug)]
pub(crate) enum Statement<'a> {
    Let(LetStatement<'a>),
}

impl<'a> Statement<'a> {
    pub(crate) fn parse(tokenizer: &mut Peekable<Tokenizer<'a>>) -> Result<Self, ParserError<'a>> {
        let st = tokenizer.next().ok_or(ParserError::UnexpectedEof)??;
        match st.token {
            Token::Keyword(Keyword::Let) => LetStatement::parse(tokenizer).map(Self::Let),
            Token::Keyword(Keyword::If) => todo!(),
            Token::Keyword(Keyword::While) => todo!(),
            Token::Keyword(Keyword::Do) => todo!(),
            Token::Keyword(Keyword::Return) => todo!(),
            _ => Err(ParserError::UnexpectedToken(st)),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LetStatement<'a> {
    pub(crate) var_name: &'a str,
    pub(crate) index: Option<Expression>,
    pub(crate) expression: Expression,
}

impl<'a> LetStatement<'a> {
    pub(crate) fn parse(tokenizer: &mut Peekable<Tokenizer<'a>>) -> Result<Self, ParserError<'a>> {
        eat!(tokenizer, Token::Keyword(Keyword::Let))?;
        let var_name = eat!(tokenizer, Token::Identifier)?;

        // Handle index case.
        let index = match peek_token(tokenizer, Token::Symbol(Symbol::LeftBracket)) {
            true => {
                eat!(tokenizer, Token::Symbol(Symbol::LeftBracket))?;

                Some(Expression::parse(tokenizer)?)
            }
            false => None,
        };

        // Eat the assignment.
        eat!(tokenizer, Token::Symbol(Symbol::Equals))?;
        let expression = Expression::parse(tokenizer)?;

        Ok(LetStatement { var_name, index, expression })
    }
}
