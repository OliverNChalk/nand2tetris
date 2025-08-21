mod expression;
mod statement;
mod structure;
mod utils;

use thiserror::Error;

use crate::parser::structure::{
    Class, ClassVariableDeclaration, FieldModifier, ParameterDeclaration, ReturnType,
    SubroutineBody, SubroutineDeclaration, SubroutineType, Type,
};
use crate::parser::utils::{check_next, eat};
use crate::tokenizer::{Keyword, SourceToken, Symbol, Token, TokenizeError, Tokenizer};

pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Class<'a>, ParserError<'a>> {
        // All Jack files must contain exactly one class, so lets start by eating the
        // beginning of the class declaration.
        eat!(tokenizer, Token::Keyword(Keyword::Class))?;
        let class_name = eat!(tokenizer, Token::Identifier)?;
        eat!(tokenizer, Token::Symbol(Symbol::LeftBrace))?;

        // Next we eat the body of the class.
        let class = Class {
            name: class_name.to_string(),
            // PERF: These temporary vector allocations are annoying.
            variables: Self::eat_multiple(tokenizer, Self::try_eat_class_variables)?
                .into_iter()
                .flatten()
                .collect(),
            subroutines: Self::eat_multiple(tokenizer, Self::try_eat_class_subroutine)?,
        };

        // Finally we finish up the class declaration.
        Self::eat(tokenizer, Token::Symbol(Symbol::RightBrace))?;
        assert!(tokenizer.next().is_none());

        Ok(class)
    }

    fn eat<'a>(tokenizer: &mut Tokenizer<'a>, expected: Token) -> Result<&'a str, ParserError<'a>> {
        let SourceToken { source, token } = tokenizer.next().unwrap()?;
        assert_eq!(token, expected);

        Ok(source)
    }

    fn eat_multiple<'a, T>(
        tokenizer: &mut Tokenizer<'a>,
        try_eat: impl Fn(&mut Tokenizer<'a>) -> Result<Option<T>, ParserError<'a>>,
    ) -> Result<Vec<T>, ParserError<'a>> {
        std::iter::from_fn(|| try_eat(tokenizer).transpose()).try_fold(
            Vec::default(),
            |mut variables, res| {
                variables.push(res?);

                Ok(variables)
            },
        )
    }

    fn try_eat_class_variables<'a>(
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<Option<Vec<ClassVariableDeclaration<'a>>>, ParserError<'a>> {
        let Some(Ok(peek)) = tokenizer.peek_0() else { return Ok(None) };

        if !matches!(peek.token, Token::Keyword(Keyword::Static | Keyword::Field)) {
            return Ok(None);
        }

        // Eat the modifier.
        let modifier = tokenizer.next().unwrap().unwrap();
        let modifier = match modifier.token {
            Token::Keyword(Keyword::Static) => FieldModifier::Static,
            Token::Keyword(Keyword::Field) => FieldModifier::Field,
            _ => unreachable!(),
        };

        // Eat the type.
        let SourceToken { source, token } =
            tokenizer.next().ok_or(ParserError::UnexpectedEof)??;
        let var_type = match token {
            Token::Keyword(Keyword::Int) => Type::Int,
            Token::Keyword(Keyword::Char) => Type::Char,
            Token::Keyword(Keyword::Boolean) => Type::Boolean,
            Token::Identifier => Type::Class(source),
            _ => return Err(ParserError::UnexpectedToken(SourceToken { source, token })),
        };

        // Eat the first variable name.
        let name = eat!(tokenizer, Token::Identifier)?;

        // Eat remaining the variable declarations.
        let mut vars = vec![ClassVariableDeclaration { modifier, var_type, name }];
        while check_next(tokenizer, Token::Symbol(Symbol::Comma)) {
            // Eat the comma.
            eat!(tokenizer, Token::Symbol(Symbol::Comma))?;

            // Eat the next variable name.
            let name = eat!(tokenizer, Token::Identifier)?;

            vars.push(ClassVariableDeclaration { modifier, var_type, name })
        }
        eat!(tokenizer, Token::Symbol(Symbol::Semicolon))?;

        Ok(Some(vars))
    }

    fn try_eat_class_subroutine<'a>(
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<Option<SubroutineDeclaration<'a>>, ParserError<'a>> {
        let Some(Ok(peek)) = tokenizer.peek_0() else { return Ok(None) };

        if !matches!(
            peek.token,
            Token::Keyword(Keyword::Constructor | Keyword::Function | Keyword::Method)
        ) {
            return Ok(None);
        }

        // Eat the function category.
        let subroutine_type = tokenizer.next().unwrap()?;
        let subroutine_type = match subroutine_type.token {
            Token::Keyword(Keyword::Constructor) => SubroutineType::Constructor,
            Token::Keyword(Keyword::Function) => SubroutineType::Function,
            Token::Keyword(Keyword::Method) => SubroutineType::Method,
            _ => return Err(ParserError::UnexpectedToken(subroutine_type)),
        };

        // Eat the return type.
        let SourceToken { source, token } =
            tokenizer.next().ok_or(ParserError::UnexpectedEof)??;
        let return_type = match token {
            Token::Keyword(Keyword::Void) => ReturnType::Void,
            Token::Identifier => ReturnType::Class(source),
            _ => return Err(ParserError::UnexpectedToken(SourceToken { source, token })),
        };

        // Eat the subroutine name.
        let name = eat!(tokenizer, Token::Identifier)?;

        // Eat any parameter declarations.
        eat!(tokenizer, Token::Symbol(Symbol::LeftParen))?;
        let mut parameters = Vec::default();
        let mut more = false;
        loop {
            // If this is not a parameter declaration, we are done.
            if !matches!(
                tokenizer.peek_0(),
                Some(Ok(SourceToken {
                    token: Token::Keyword(Keyword::Int)
                        | Token::Keyword(Keyword::Char)
                        | Token::Keyword(Keyword::Boolean)
                        | Token::Identifier,
                    ..
                }))
            ) {
                break;
            }

            // Eat the parameter type.
            let parameter_type = Type::parse(tokenizer)?;

            // Eat the parameter name.
            let name = eat!(tokenizer, Token::Identifier)?;

            // Maybe eat a comma.
            let has_comma = matches!(
                tokenizer.peek_0(),
                Some(Ok(SourceToken { token: Token::Symbol(Symbol::Comma), .. }))
            );
            if has_comma {
                tokenizer.next().unwrap().unwrap();
            }
            more = has_comma;

            parameters.push(ParameterDeclaration { parameter_type, name })
        }
        if more {
            return Err(ParserError::TrailingComma);
        }
        eat!(tokenizer, Token::Symbol(Symbol::RightParen))?;

        // Parse the subroutine body.
        let body = SubroutineBody::parse(tokenizer)?;

        Ok(Some(SubroutineDeclaration { subroutine_type, return_type, name, parameters, body }))
    }
}

#[derive(Debug, Error)]
pub(crate) enum ParserError<'a> {
    #[error("Invalid token; err={0}")]
    InvalidToken(#[from] TokenizeError),
    #[error("Unexpected token; token={0:?}")]
    UnexpectedToken(SourceToken<'a>),
    #[error("Unexpected eof")]
    UnexpectedEof,
    #[error("Trailing comma")]
    TrailingComma,
}
