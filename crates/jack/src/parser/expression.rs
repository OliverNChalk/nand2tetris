use std::iter::Peekable;

use crate::parser::ParserError;
use crate::tokenizer::Tokenizer;

#[derive(Debug)]
pub(crate) struct Expression<'a> {
    term: Box<Term<'a>>,
    operation: Option<Box<(Op, Term<'a>)>>,
}

impl<'a> Expression<'a> {
    pub(crate) fn parse(
        tokenizer: &mut Peekable<&mut Tokenizer<'a>>,
    ) -> Result<Self, ParserError<'a>> {
        todo!()
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
