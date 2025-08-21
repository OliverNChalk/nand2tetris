use crate::parser::utils::{check_next, eat, peek};
use crate::parser::ParserError;
use crate::tokenizer::{Keyword, Symbol, Token, Tokenizer};

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
    Variable(&'a str),
    VariableIndex(VariableIndex<'a>),
    Expression(Expression<'a>),
    UnaryOp { op: UnaryOp, term: Box<Self> },
    SubroutineCall(SubroutineCall<'a>),
}

impl<'a> Term<'a> {
    fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError<'a>> {
        let st = tokenizer.peek_0().ok_or(ParserError::UnexpectedEof)??;
        let mut simple_term = |term: Term<'a>| -> Term<'a> {
            tokenizer.next().unwrap().unwrap();

            term
        };

        Ok(match st.token {
            Token::IntegerConstant(integer) => simple_term(Term::IntegerConstant(integer)),
            Token::StringConstant => simple_term(Term::StringConstant(st.source)),
            Token::Keyword(Keyword::True) => simple_term(Term::True),
            Token::Keyword(Keyword::False) => simple_term(Term::False),
            Token::Keyword(Keyword::Null) => simple_term(Term::Null),
            Token::Keyword(Keyword::This) => simple_term(Term::This),
            Token::Symbol(Symbol::Minus) => {
                eat!(tokenizer, Token::Symbol(Symbol::Minus))?;
                let term = Box::new(Term::parse(tokenizer)?);

                Term::UnaryOp { op: UnaryOp::Negate, term }
            }
            Token::Symbol(Symbol::Tilde) => {
                eat!(tokenizer, Token::Symbol(Symbol::Tilde))?;
                let term = Box::new(Term::parse(tokenizer)?);

                Term::UnaryOp { op: UnaryOp::Not, term }
            }
            Token::Symbol(Symbol::LeftParen) => {
                eat!(tokenizer, Token::Symbol(Symbol::LeftParen))?;
                let expression = Expression::parse(tokenizer)?;
                eat!(tokenizer, Token::Symbol(Symbol::RightParen))?;

                Term::Expression(expression)
            }
            Token::Identifier => {
                let next = tokenizer.peek_1().ok_or(ParserError::UnexpectedEof)??.token;
                match next {
                    Token::Symbol(Symbol::LeftBracket) => {
                        Term::VariableIndex(VariableIndex::parse(tokenizer)?)
                    }
                    Token::Symbol(Symbol::LeftParen | Symbol::Dot) => {
                        Term::SubroutineCall(SubroutineCall::parse(tokenizer)?)
                    }
                    _ => {
                        tokenizer.next().unwrap().unwrap();

                        Term::Variable(st.source)
                    }
                }
            }
            _ => return Err(ParserError::UnexpectedToken(tokenizer.next().unwrap().unwrap())),
        })
    }
}

#[derive(Debug)]
pub(crate) struct VariableIndex<'a> {
    var: &'a str,
    index: i16,
}

impl<'a> VariableIndex<'a> {
    fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError<'a>> {
        let var = eat!(tokenizer, Token::Identifier)?;
        eat!(tokenizer, Token::Symbol(Symbol::LeftBracket))?;
        let index = tokenizer.next().ok_or(ParserError::UnexpectedEof)??;
        eat!(tokenizer, Token::Symbol(Symbol::RightBracket))?;
        let Token::IntegerConstant(index) = index.token else {
            return Err(ParserError::UnexpectedToken(index));
        };

        Ok(Self { var, index })
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
