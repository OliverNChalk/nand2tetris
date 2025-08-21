use std::iter::Peekable;

use crate::parser::ParserError;
use crate::tokenizer::{SourceToken, Token, Tokenizer};

#[derive(Debug)]
pub(crate) struct Expression<'a> {
    term: Box<Term<'a>>,
    operation: Option<Box<(Op, Term<'a>)>>,
}

impl<'a> Expression<'a> {
    pub(crate) fn parse(
        tokenizer: &mut Peekable<&mut Tokenizer<'a>>,
    ) -> Result<Self, ParserError<'a>> {
        let term = Box::new(Term::parse(tokenizer)?);

        // TODO: Handle the op case.

        Ok(Expression { term, operation: None })
    }
}

#[derive(Debug)]
pub(crate) enum Term<'a> {
    IntegerConstant(i16),
    StringConstant(&'a str),
    True,
    False,
    Null,
    This,
    VarName(&'a str),
    VarNameIndex(()),
    Expression(Expression<'a>),
    UnaryOp(()),
    SubroutineCall(()),
}

impl<'a> Term<'a> {
    fn parse(tokenizer: &mut Peekable<&mut Tokenizer<'a>>) -> Result<Self, ParserError<'a>> {
        let SourceToken { source, token } =
            tokenizer.next().ok_or(ParserError::UnexpectedEof)??;
        Ok(match token {
            Token::StringConstant => Term::StringConstant(source),
            Token::IntegerConstant(integer) => Term::IntegerConstant(integer),
            Token::Identifier => Term::VarName(source),
            _ => todo!("Term; {token:?}"),
        })
    }
}

#[derive(Debug)]
pub(crate) struct SubroutineCall;

impl SubroutineCall {
    pub(crate) fn parse<'a>(
        tokenizer: &mut Peekable<&mut Tokenizer<'a>>,
    ) -> Result<Self, ParserError<'a>> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
    BitAnd,
    BitOr,
    Lt,
    Gt,
    Equals,
}

#[derive(Debug)]
pub(crate) enum UnaryOp {
    Negate,
    Not,
}
