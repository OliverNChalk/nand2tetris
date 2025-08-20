use std::iter::Peekable;

use crate::tokenizer::{Keyword, SourceToken, Symbol, Token, TokenizeError, Tokenizer};

pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse(tokenizer: Tokenizer) -> Result<Class, TokenizeError> {
        let mut tokenizer = tokenizer.peekable();

        // All Jack files must contain exactly one class, so lets start by eating the
        // beginning of the class declaration.
        let _ = Self::eat(&mut tokenizer, Token::Keyword(Keyword::Class))?;
        let class_name = Self::eat(&mut tokenizer, Token::Identifier)?;
        let _ = Self::eat(&mut tokenizer, Token::Symbol(Symbol::LeftBrace))?;

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
    ) -> Result<&'a str, TokenizeError> {
        let SourceToken { source, token } = tokenizer.next().unwrap()?;
        assert_eq!(token, expected);

        Ok(source)
    }

    fn eat_multiple<T>(
        tokenizer: &mut Peekable<Tokenizer>,
        try_eat: impl Fn(&mut Peekable<Tokenizer>) -> Result<Option<T>, TokenizeError>,
    ) -> Result<Vec<T>, TokenizeError> {
        std::iter::from_fn(|| try_eat(tokenizer).transpose()).try_fold(
            Vec::default(),
            |mut variables, res| {
                variables.push(res?);

                Ok(variables)
            },
        )
    }

    fn try_eat_class_variable(
        tokenizer: &mut Peekable<Tokenizer>,
    ) -> Result<Option<VariableDeclaration>, TokenizeError> {
        todo!()
    }

    fn try_eat_class_subroutine(
        tokenizer: &mut Peekable<Tokenizer>,
    ) -> Result<Option<SubroutineDeclaration>, TokenizeError> {
        todo!()
    }

    fn parse_keyword() -> () {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct Class {
    pub(crate) name: String,
    pub(crate) variables: Vec<VariableDeclaration>,
    pub(crate) subroutines: Vec<SubroutineDeclaration>,
}

#[derive(Debug)]
pub(crate) struct VariableDeclaration {}

#[derive(Debug)]
pub(crate) struct SubroutineDeclaration {}
