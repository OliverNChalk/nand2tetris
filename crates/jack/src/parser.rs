use std::iter::Peekable;

use thiserror::Error;

use crate::tokenizer::{Keyword, SourceToken, Symbol, Token, TokenizeError, Tokenizer};

macro_rules! eat {
    ($tokenizer:expr, $expected:pat) => {{
        let SourceToken { source, token } = $tokenizer.next().unwrap()?;
        if !matches!(token, $expected) {
            return Err(ParserError::UnexpectedToken(SourceToken { source, token }));
        }

        Ok::<_, ParserError>(source)
    }};
}

pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse(tokenizer: Tokenizer) -> Result<Class, ParserError> {
        let mut tokenizer = tokenizer.peekable();

        // All Jack files must contain exactly one class, so lets start by eating the
        // beginning of the class declaration.
        eat!(&mut tokenizer, Token::Keyword(Keyword::Class))?;
        let class_name = eat!(&mut tokenizer, Token::Identifier)?;
        eat!(&mut tokenizer, Token::Symbol(Symbol::LeftBrace))?;

        // Next we eat the body of the class.
        let class = Class {
            name: class_name.to_string(),
            // PERF: These temporary vector allocations are annoying.
            variables: Self::eat_multiple(&mut tokenizer, Self::try_eat_class_variable)?
                .into_iter()
                .flatten()
                .collect(),
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
    ) -> Result<Option<Vec<VariableDeclaration<'a>>>, ParserError<'a>> {
        let Some(Ok(peek)) = tokenizer.peek() else { return Ok(None) };

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
        let SourceToken { source: name, token } =
            tokenizer.next().ok_or(ParserError::UnexpectedEof)??;
        if token != Token::Identifier {
            return Err(ParserError::UnexpectedToken(SourceToken { source: name, token }));
        }

        // Eat remaining the variable declarations.
        let mut vars = vec![VariableDeclaration { modifier, var_type, name }];
        loop {
            let Some(Ok(token)) = tokenizer.peek() else {
                break;
            };

            // Following the prior variable declaration should be a comma if we have more
            // variable declarations.
            if token.token != Token::Symbol(Symbol::Comma) {
                break;
            }

            // Eat the comma.
            eat!(tokenizer, Token::Symbol(Symbol::Comma))?;

            // Eat the next variable name.
            let name = eat!(tokenizer, Token::Symbol(Symbol::Comma))?;

            vars.push(VariableDeclaration { modifier, var_type, name })
        }

        // Eat the semicolon.
        let SourceToken { source, token } =
            tokenizer.next().ok_or(ParserError::UnexpectedEof)??;
        if token != Token::Symbol(Symbol::Semicolon) {
            return Err(ParserError::UnexpectedToken(SourceToken { source, token }));
        }

        Ok(Some(vars))
    }

    fn try_eat_class_subroutine<'a>(
        tokenizer: &mut Peekable<Tokenizer<'a>>,
    ) -> Result<Option<SubroutineDeclaration<'a>>, ParserError<'a>> {
        let Some(Ok(peek)) = tokenizer.peek() else { return Ok(None) };

        if !matches!(
            peek.token,
            Token::Keyword(Keyword::Constructor | Keyword::Function | Keyword::Method)
        ) {
            return Ok(None);
        }

        // Eat the function category.
        let category = tokenizer.next().unwrap()?;
        let category = match category.token {
            Token::Keyword(Keyword::Constructor) => FunctionCategory::Constructor,
            Token::Keyword(Keyword::Function) => FunctionCategory::Function,
            Token::Keyword(Keyword::Method) => FunctionCategory::Method,
            _ => return Err(ParserError::UnexpectedToken(category)),
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

        // TODO: Parameter list.

        // TODO: Function body.

        Ok(Some(SubroutineDeclaration {
            category,
            return_type,
            name,
            parameter_list: vec![],
            body: SubroutineBody {},
        }))
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
    #[error("Unexpected eof")]
    UnexpectedEof,
}

#[derive(Debug)]
pub(crate) struct Class<'a> {
    pub(crate) name: String,
    pub(crate) variables: Vec<VariableDeclaration<'a>>,
    pub(crate) subroutines: Vec<SubroutineDeclaration<'a>>,
}

#[derive(Debug)]
pub(crate) struct VariableDeclaration<'a> {
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
    category: FunctionCategory,
    return_type: ReturnType<'a>,
    name: &'a str,
    parameter_list: Vec<()>,
    body: SubroutineBody,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum FunctionCategory {
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
pub(crate) struct SubroutineBody {}
