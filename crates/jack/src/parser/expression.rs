use crate::parser::utils::{check_next, eat, peek};
use crate::parser::ParserError;
use crate::tokenizer::{Keyword, SourceToken, Symbol, Token, Tokenizer};

#[derive(Debug)]
pub(crate) struct Expression<'a> {
    term: Box<Term<'a>>,
    op: Option<Box<(Op, Term<'a>)>>,
}

impl<'a> Expression<'a> {
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError<'a>> {
        let term = Box::new(Term::parse(tokenizer)?);

        // Eat the op if one exists.
        let op = match peek(tokenizer) {
            Some(Token::Symbol(
                Symbol::Plus
                | Symbol::Minus
                | Symbol::Asterisk
                | Symbol::ForwardSlash
                | Symbol::Ampersand
                | Symbol::Pipe
                | Symbol::LeftAngleBracket
                | Symbol::RightAngleBracket
                | Symbol::Equals,
            )) => Some(Box::new((Op::parse(tokenizer)?, Term::parse(tokenizer)?))),
            _ => None,
        };

        Ok(Expression { term, op })
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
    SubroutineCall(SubroutineCall<'a>),
}

impl<'a> Term<'a> {
    fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError<'a>> {
        let SourceToken { source, token } =
            tokenizer.next().ok_or(ParserError::UnexpectedEof)??;
        Ok(match token {
            Token::IntegerConstant(integer) => Term::IntegerConstant(integer),
            Token::StringConstant => Term::StringConstant(source),
            Token::Keyword(Keyword::True) => Term::True,
            Token::Keyword(Keyword::False) => Term::False,
            Token::Keyword(Keyword::Null) => Term::Null,
            Token::Keyword(Keyword::This) => Term::This,
            Token::Identifier => match peek(tokenizer).ok_or(ParserError::UnexpectedEof)? {
                Token::Symbol(Symbol::LeftBracket) => todo!("Index expression"),
                Token::Symbol(Symbol::LeftParen) => {
                    Term::SubroutineCall(SubroutineCall::parse(tokenizer)?)
                }
                _ => Term::VarName(source),
            },
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
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError<'a>> {
        // No matter what, a subroutine call begins with an identifier (class, variable,
        // or subroutine).
        let first_identifier = eat!(tokenizer, Token::Identifier)?;

        // If the next variable is a `.` then we have a class/variable identifier, else
        // we have a subroutine identifier.
        let (var, subroutine) = match check_next(tokenizer, Token::Symbol(Symbol::Dot)) {
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
        while !check_next(tokenizer, Token::Symbol(Symbol::RightParen)) {
            arguments.push(Expression::parse(tokenizer)?);
            if check_next(tokenizer, Token::Symbol(Symbol::Comma)) {
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

impl Op {
    pub(crate) fn parse<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError<'a>> {
        let st = tokenizer.next().ok_or(ParserError::UnexpectedEof)??;
        match st.token {
            Token::Symbol(Symbol::Plus) => Ok(Self::Plus),
            Token::Symbol(Symbol::Minus) => Ok(Self::Minus),
            Token::Symbol(Symbol::Asterisk) => Ok(Self::Multiply),
            Token::Symbol(Symbol::ForwardSlash) => Ok(Self::Divide),
            Token::Symbol(Symbol::Ampersand) => Ok(Self::BitAnd),
            Token::Symbol(Symbol::Pipe) => Ok(Self::BitOr),
            Token::Symbol(Symbol::LeftAngleBracket) => Ok(Self::Lt),
            Token::Symbol(Symbol::RightAngleBracket) => Ok(Self::Gt),
            Token::Symbol(Symbol::Equals) => Ok(Self::Equals),
            _ => Err(ParserError::UnexpectedToken(st)),
        }
    }
}

#[derive(Debug)]
pub(crate) enum UnaryOp {
    Negate,
    Not,
}
