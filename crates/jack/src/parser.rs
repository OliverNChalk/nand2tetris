use std::iter::Peekable;

use thiserror::Error;

use crate::tokenizer::{Keyword, SourceToken, Symbol, Token, TokenizeError, Tokenizer};

macro_rules! eat {
    ($tokenizer:expr, $expected:pat) => {{
        let SourceToken { source, token } = $tokenizer.next().unwrap()?;
        assert!(matches!(token, $expected));

        source
    }};
}

pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse(tokenizer: Tokenizer) -> Result<Class, ParserError> {
        let mut tokenizer = tokenizer.peekable();

        // All Jack files must contain exactly one class, so lets start by eating the
        // beginning of the class declaration.
        let _ = eat!(&mut tokenizer, Token::Keyword(Keyword::Class));
        let class_name = eat!(&mut tokenizer, Token::Identifier);
        let _ = eat!(&mut tokenizer, Token::Symbol(Symbol::LeftBrace));

        // Next we eat the body of the class.
        let class = Class {
            name: class_name.to_string(),
            variables: Self::eat_multiple(&mut tokenizer, Self::try_eat_class_variable)?,
            subroutines: Self::eat_multiple(&mut tokenizer, Self::try_eat_class_subroutine)?,
        };

        // Finally we finish up the class declaration.
        Self::eat(&mut tokenizer, Token::Symbol(Symbol::RightBrace))?;
        assert!(tokenizer.next().is_none());

        Ok(class)
    }

    fn eat<'a>(
        tokenizer: &mut Peekable<Tokenizer<'a>>,
        expected: Token,
    ) -> Result<&'a str, ParserError<'a>> {
        let SourceToken { source, token } = tokenizer.next().unwrap()?;
        assert_eq!(token, expected);

        Ok(source)
    }

    fn eat_multiple<'a, T>(
        tokenizer: &mut Peekable<Tokenizer<'a>>,
        try_eat: impl Fn(&mut Peekable<Tokenizer<'a>>) -> Result<Option<T>, ParserError<'a>>,
    ) -> Result<Vec<T>, ParserError<'a>> {
        std::iter::from_fn(|| try_eat(tokenizer).transpose()).try_fold(
            Vec::default(),
            |mut variables, res| {
                variables.push(res?);

                Ok(variables)
            },
        )
    }

    fn try_eat_class_variable<'a>(
        tokenizer: &mut Peekable<Tokenizer<'a>>,
    ) -> Result<Option<VariableDeclaration<'a>>, ParserError<'a>> {
        let Some(Ok(peek)) = tokenizer.peek() else { return Ok(None) };

        if !matches!(peek.token, Token::Keyword(Keyword::Static | Keyword::Field)) {
            return Ok(None);
        }

        // Extract the modifier.
        let modifier = tokenizer.next().unwrap().unwrap();
        let modifier = match modifier.token {
            Token::Keyword(Keyword::Static) => Modifier::Static,
            Token::Keyword(Keyword::Field) => Modifier::Field,
            _ => unreachable!(),
        };

        // Extract the type.
        let SourceToken { source, token } = tokenizer.next().unwrap()?;
        let var_type = match token {
            Token::Keyword(Keyword::Int) => Type::Int,
            Token::Keyword(Keyword::Char) => Type::Char,
            Token::Keyword(Keyword::Boolean) => Type::Boolean,
            Token::Identifier => Type::Class(source),
            _ => return Err(ParserError::UnexpectedToken(SourceToken { source, token })),
        };

        todo!()
    }

    fn try_eat_class_subroutine<'a>(
        tokenizer: &mut Peekable<Tokenizer<'a>>,
    ) -> Result<Option<SubroutineDeclaration>, ParserError<'a>> {
        todo!()
    }

    fn parse_keyword() -> () {
        todo!()
    }
}

#[derive(Debug, Error)]
pub(crate) enum ParserError<'a> {
    #[error("Invalid token; err={0}")]
    InvalidToken(#[from] TokenizeError),
    #[error("Unexpected token; token={0:?}")]
    UnexpectedToken(SourceToken<'a>),
}

#[derive(Debug)]
pub(crate) struct Class<'a> {
    pub(crate) name: String,
    pub(crate) variables: Vec<VariableDeclaration<'a>>,
    pub(crate) subroutines: Vec<SubroutineDeclaration>,
}

#[derive(Debug)]
pub(crate) struct VariableDeclaration<'a> {
    pub(crate) modifier: Modifier,
    pub(crate) var_type: Type<'a>,
}

#[derive(Debug)]
pub(crate) enum Modifier {
    Static,
    Field,
}

#[derive(Debug)]
pub(crate) enum Type<'a> {
    Int,
    Char,
    Boolean,
    Class(&'a str),
}

#[derive(Debug)]
pub(crate) struct SubroutineDeclaration {}
