use std::iter::Peekable;

use crate::parser::expression::Expression;
use crate::parser::utils::{self, eat, peek_token};
use crate::parser::ParserError;
use crate::tokenizer::{Keyword, Symbol, Token, Tokenizer};

#[derive(Debug)]
pub(crate) enum Statement<'a> {
    Let(LetStatement<'a>),
    If(IfStatement<'a>),
    While(WhileStatement<'a>),
    Do(DoStatement),
    Return(ReturnStatement<'a>),
}

impl<'a> Statement<'a> {
    pub(crate) fn parse(
        tokenizer: &mut Peekable<&mut Tokenizer<'a>>,
    ) -> Result<Self, ParserError<'a>> {
        let st = match tokenizer.peek().ok_or(ParserError::UnexpectedEof)? {
            Ok(token) => token,
            Err(err) => return Err(ParserError::InvalidToken(*err)),
        };
        match st.token {
            Token::Keyword(Keyword::Let) => LetStatement::parse(tokenizer).map(Self::Let),
            Token::Keyword(Keyword::If) => IfStatement::parse(tokenizer).map(Self::If),
            Token::Keyword(Keyword::While) => WhileStatement::parse(tokenizer).map(Self::While),
            Token::Keyword(Keyword::Do) => DoStatement::parse(tokenizer).map(Self::Do),
            Token::Keyword(Keyword::Return) => ReturnStatement::parse(tokenizer).map(Self::Return),
            _ => Err(ParserError::UnexpectedToken(*st)),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LetStatement<'a> {
    pub(crate) var_name: &'a str,
    pub(crate) index: Option<Expression<'a>>,
    pub(crate) expression: Expression<'a>,
}

impl<'a> LetStatement<'a> {
    pub(crate) fn parse(
        tokenizer: &mut Peekable<&mut Tokenizer<'a>>,
    ) -> Result<Self, ParserError<'a>> {
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

#[derive(Debug)]
pub(crate) struct IfStatement<'a> {
    pub(crate) expression: Expression<'a>,
    pub(crate) if_statements: Vec<Statement<'a>>,
    pub(crate) else_statement: Vec<Statement<'a>>,
}

impl<'a> IfStatement<'a> {
    pub(crate) fn parse(
        tokenizer: &mut Peekable<&mut Tokenizer<'a>>,
    ) -> Result<Self, ParserError<'a>> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct WhileStatement<'a> {
    pub(crate) expression: Expression<'a>,
    pub(crate) statements: Vec<Statement<'a>>,
}

impl<'a> WhileStatement<'a> {
    pub(crate) fn parse(
        tokenizer: &mut Peekable<&mut Tokenizer<'a>>,
    ) -> Result<Self, ParserError<'a>> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct DoStatement {
    pub(crate) subroutine_call: (),
}

impl DoStatement {
    pub(crate) fn parse<'a>(
        tokenizer: &mut Peekable<&mut Tokenizer<'a>>,
    ) -> Result<Self, ParserError<'a>> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct ReturnStatement<'a> {
    pub(crate) return_value: Option<Expression<'a>>,
}

impl<'a> ReturnStatement<'a> {
    pub(crate) fn parse(
        tokenizer: &mut Peekable<&mut Tokenizer<'a>>,
    ) -> Result<Self, ParserError<'a>> {
        todo!()
    }
}
