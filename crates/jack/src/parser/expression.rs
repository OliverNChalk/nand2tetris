use std::iter::Peekable;

use crate::parser::utils::{eat, peek_token};
use crate::parser::ParserError;
use crate::tokenizer::{Keyword, SourceToken, Symbol, Token, Tokenizer};

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
            Token::Keyword(Keyword::True) => Term::True,
            Token::Keyword(Keyword::False) => Term::False,
            Token::Keyword(Keyword::Null) => Term::Null,
            Token::Keyword(Keyword::This) => Term::This,
            _ => todo!("Term; {token:?}"),
        })
    }
}

#[derive(Debug)]
pub(crate) struct SubroutineCall<'a> {
    var: Option<&'a str>,
    subroutine: &'a str,
    arguments: Vec<Expression<'a>>,
}

impl<'a> SubroutineCall<'a> {
    pub(crate) fn parse(
        tokenizer: &mut Peekable<&mut Tokenizer<'a>>,
    ) -> Result<Self, ParserError<'a>> {
        // No matter what, a subroutine call begins with an identifier (class, variable,
        // or subroutine).
        let first_identifier = eat!(tokenizer, Token::Identifier)?;

        // If the next variable is a `.` then we have a class/variable identifier, else
        // we have a subroutine identifier.
        let (var, subroutine) = match peek_token(tokenizer, Token::Symbol(Symbol::Dot)) {
            true => {
                eat!(tokenizer, Token::Symbol(Symbol::Dot))?;
                let subroutine = eat!(tokenizer, Token::Identifier)?;

                (Some(first_identifier), subroutine)
            }
            false => (None, first_identifier),
        };

        // Eat all the arguments.
        eat!(tokenizer, Token::Symbol(Symbol::LeftParen))?;
        let mut arguments = Vec::default();
        while !peek_token(tokenizer, Token::Symbol(Symbol::RightParen)) {
            arguments.push(Expression::parse(tokenizer)?);
            if peek_token(tokenizer, Token::Symbol(Symbol::Comma)) {
                eat!(tokenizer, Token::Symbol(Symbol::Comma))?;
            }
        }
        eat!(tokenizer, Token::Symbol(Symbol::RightParen))?;
        eat!(tokenizer, Token::Symbol(Symbol::Semicolon))?;

        Ok(SubroutineCall { var, subroutine, arguments })
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
