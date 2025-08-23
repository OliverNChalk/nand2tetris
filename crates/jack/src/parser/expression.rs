use hashbrown::HashMap;

use crate::code_gen::{ClassContext, CompileError, SymbolCategory, SymbolEntry};
use crate::parser::error::ParseError;
use crate::parser::structure::Type;
use crate::parser::utils::{check_next, eat, peek};
use crate::tokenizer::{Keyword, Symbol, Token, Tokenizer};

#[derive(Debug)]
pub(crate) struct Expression<'a> {
    term: Box<Term<'a>>,
    op: Option<(Op, Box<Term<'a>>)>,
}

impl<'a> Expression<'a> {
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParseError<'a>> {
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
            )) => Some((Op::parse(tokenizer)?, Box::new(Term::parse(tokenizer)?))),
            _ => None,
        };

        Ok(Expression { term, op })
    }

    pub(crate) fn compile(
        &self,
        class: &ClassContext,
        subroutine: &HashMap<&str, SymbolEntry>,
    ) -> Result<Vec<String>, CompileError<'a>> {
        let mut code = self.term.compile(class, subroutine)?;
        if let Some((op, term)) = &self.op {
            code.extend(term.compile(class, subroutine)?);
            code.push(op.compile());
        }

        Ok(code)
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
    fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParseError<'a>> {
        let st = tokenizer.peek_0().ok_or(ParseError::UnexpectedEof)??;
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
                let next = tokenizer.peek_1().ok_or(ParseError::UnexpectedEof)??.token;
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
            _ => return Err(ParseError::UnexpectedToken(tokenizer.next().unwrap().unwrap())),
        })
    }

    pub(crate) fn compile(
        &self,
        class: &ClassContext,
        subroutine: &HashMap<&str, SymbolEntry>,
    ) -> Result<Vec<String>, CompileError<'a>> {
        match self {
            Self::IntegerConstant(integer) => Ok(vec![format!("push constant {integer}")]),
            Self::Expression(expression) => expression.compile(class, subroutine),
            Self::UnaryOp { op, term } => {
                let mut code = term.compile(class, subroutine)?;
                code.push(op.compile());

                Ok(code)
            }
            Self::True => Ok(vec!["push constant 1".to_string(), "neg".to_string()]),
            Self::False => Ok(vec!["push constant 0".to_string()]),
            Self::Null => Ok(vec!["push constant 0".to_string()]),
            Self::This => Ok(vec!["push pointer 0".to_string()]),
            Self::Variable(var) => Ok(vec![subroutine
                .get(var)
                .or_else(|| class.symbols.get(var))
                .ok_or(CompileError::UnknownSymbol(var))?
                .compile_push()]),
            Self::SubroutineCall(call) => call.compile(class, subroutine),
            _ => todo!("{self:?}"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct VariableIndex<'a> {
    var: &'a str,
    index: i16,
}

impl<'a> VariableIndex<'a> {
    fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParseError<'a>> {
        let var = eat!(tokenizer, Token::Identifier)?;
        eat!(tokenizer, Token::Symbol(Symbol::LeftBracket))?;
        let index = tokenizer.next().ok_or(ParseError::UnexpectedEof)??;
        eat!(tokenizer, Token::Symbol(Symbol::RightBracket))?;
        let Token::IntegerConstant(index) = index.token else {
            return Err(ParseError::UnexpectedToken(index));
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
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParseError<'a>> {
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

    pub(crate) fn compile(
        &self,
        class: &ClassContext,
        subroutine: &HashMap<&str, SymbolEntry>,
    ) -> Result<Vec<String>, CompileError<'a>> {
        // Push the object being operated on if necessary.
        let (class_name, push_this) = match self.var {
            Some(var) => {
                let push_this = subroutine.get(var).or_else(|| {
                    class
                        .symbols
                        .get(var)
                        .filter(|symbol| symbol.category == SymbolCategory::Field)
                });

                match push_this {
                    Some(symbol) => {
                        let class_name = match symbol.symbol_type {
                            Type::Class(name) => name,
                            _ => return Err(CompileError::InvalidCallee(var)),
                        };

                        (class_name, Some(symbol.compile_push()))
                    }
                    None => (var, None),
                }
            }
            None => (class.name, Some("push pointer 0".to_string())),
        };

        // Push all the arguments.
        let method = push_this.is_some();
        let mut code = Vec::from_iter(push_this);
        for arg in &self.arguments {
            code.extend(arg.compile(class, subroutine)?);
        }

        // Append the function call.
        code.push(format!(
            "call {class_name}.{} {}",
            self.subroutine,
            self.arguments.len() + usize::from(method)
        ));

        Ok(code)
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
    pub(crate) fn parse<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParseError<'a>> {
        let st = tokenizer.next().ok_or(ParseError::UnexpectedEof)??;
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
            _ => Err(ParseError::UnexpectedToken(st)),
        }
    }

    pub(crate) fn compile(&self) -> String {
        match self {
            Self::Plus => "add",
            Self::Minus => "sub",
            Self::Multiply => "call Math.multiply 2",
            Self::Divide => "call Math.divide 2",
            Self::BitAnd => "and",
            Self::BitOr => "or",
            Self::Lt => "lt",
            Self::Gt => "gt",
            Self::Equals => "eq",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub(crate) enum UnaryOp {
    Negate,
    Not,
}

impl UnaryOp {
    fn compile(&self) -> String {
        match self {
            Self::Negate => "neg".to_string(),
            Self::Not => "not".to_string(),
        }
    }
}
