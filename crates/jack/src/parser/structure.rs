use std::iter::Peekable;

use thiserror::Error;

use crate::parser::statement::Statement;
use crate::parser::utils::{eat, peek_token};
use crate::parser::ParserError;
use crate::tokenizer::{Keyword, SourceToken, Symbol, Token, TokenizeError, Tokenizer};

#[derive(Debug)]
pub(crate) struct Class<'a> {
    pub(crate) name: String,
    pub(crate) variables: Vec<ClassVariableDeclaration<'a>>,
    pub(crate) subroutines: Vec<SubroutineDeclaration<'a>>,
}

#[derive(Debug)]
pub(crate) struct ClassVariableDeclaration<'a> {
    pub(crate) modifier: FieldModifier,
    pub(crate) var_type: Type<'a>,
    pub(crate) name: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum FieldModifier {
    Static,
    Field,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Type<'a> {
    Int,
    Char,
    Boolean,
    Class(&'a str),
}

#[derive(Debug)]
pub(crate) struct SubroutineDeclaration<'a> {
    pub(crate) subroutine_type: SubroutineType,
    pub(crate) return_type: ReturnType<'a>,
    pub(crate) name: &'a str,
    pub(crate) parameters: Vec<ParameterDeclaration<'a>>,
    pub(crate) body: SubroutineBody<'a>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum SubroutineType {
    Constructor,
    Function,
    Method,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ReturnType<'a> {
    Void,
    Class(&'a str),
}

#[derive(Debug)]
pub(crate) struct ParameterDeclaration<'a> {
    pub(crate) parameter_type: Type<'a>,
    pub(crate) name: &'a str,
}

impl<'a> Type<'a> {
    pub(crate) fn parse(tokenizer: &mut Peekable<Tokenizer<'a>>) -> Result<Self, ParserError<'a>> {
        let st = tokenizer.next().ok_or(ParserError::UnexpectedEof)??;

        match st.token {
            Token::Keyword(Keyword::Int) => Ok(Self::Int),
            Token::Keyword(Keyword::Char) => Ok(Self::Char),
            Token::Keyword(Keyword::Boolean) => Ok(Self::Boolean),
            Token::Identifier => Ok(Self::Class(st.source)),
            _ => Err(ParserError::UnexpectedToken(st)),
        }
    }
}

#[derive(Debug)]
pub(crate) struct SubroutineBody<'a> {
    pub(crate) variables: Vec<SubroutineVariableDeclaration<'a>>,
    pub(crate) statements: Vec<Statement<'a>>,
}

impl<'a> SubroutineBody<'a> {
    pub(crate) fn parse(tokenizer: &mut Peekable<Tokenizer<'a>>) -> Result<Self, ParserError<'a>> {
        eat!(tokenizer, Token::Symbol(Symbol::LeftBrace))?;

        // Eat all variable declarations.
        let mut variables = Vec::default();
        while peek_token(tokenizer, Token::Keyword(Keyword::Var)) {
            // Eat the first variable.
            eat!(tokenizer, Token::Keyword(Keyword::Var))?;
            let var_type = Type::parse(tokenizer)?;
            let name = eat!(tokenizer, Token::Identifier)?;

            // Eat the remaining variables.
            variables.push(SubroutineVariableDeclaration { var_type, name });
            while peek_token(tokenizer, Token::Symbol(Symbol::Comma)) {
                eat!(tokenizer, Token::Symbol(Symbol::Comma))?;
                variables.push(SubroutineVariableDeclaration {
                    var_type,
                    name: eat!(tokenizer, Token::Identifier)?,
                });
            }
        }

        // Eat all statements.
        let mut statements = Vec::default();
        while !peek_token(tokenizer, Token::Symbol(Symbol::RightBrace)) {
            statements.push(Statement::parse(tokenizer)?);
        }

        Ok(SubroutineBody { variables, statements })
    }
}

#[derive(Debug)]
pub(crate) struct SubroutineVariableDeclaration<'a> {
    pub(crate) var_type: Type<'a>,
    pub(crate) name: &'a str,
}
