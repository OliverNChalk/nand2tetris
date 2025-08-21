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
            variables: Self::eat_multiple(&mut tokenizer, Self::try_eat_class_variables)?
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

    fn try_eat_class_variables<'a>(
        tokenizer: &mut Peekable<Tokenizer<'a>>,
    ) -> Result<Option<Vec<ClassVariableDeclaration<'a>>>, ParserError<'a>> {
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
        let mut vars = vec![ClassVariableDeclaration { modifier, var_type, name }];
        loop {
            let Some(Ok(st)) = tokenizer.peek() else {
                break;
            };

            // Following the prior variable declaration should be a comma if we have more
            // variable declarations.
            if st.token != Token::Symbol(Symbol::Comma) {
                break;
            }

            // Eat the comma.
            eat!(tokenizer, Token::Symbol(Symbol::Comma))?;

            // Eat the next variable name.
            let name = eat!(tokenizer, Token::Symbol(Symbol::Comma))?;

            vars.push(ClassVariableDeclaration { modifier, var_type, name })
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
        let mut parameters = Vec::default();
        let mut more = false;
        loop {
            // If this is not a parameter declaration, we are done.
            if !matches!(
                tokenizer.peek(),
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
                tokenizer.peek(),
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
    subroutine_type: SubroutineType,
    return_type: ReturnType<'a>,
    name: &'a str,
    parameters: Vec<ParameterDeclaration<'a>>,
    body: SubroutineBody<'a>,
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
    parameter_type: Type<'a>,
    name: &'a str,
}

impl<'a> Type<'a> {
    fn parse(tokenizer: &mut Peekable<Tokenizer<'a>>) -> Result<Self, ParserError<'a>> {
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
    variables: Vec<SubroutineVariableDeclaration<'a>>,
    statements: Vec<Statement>,
}

impl<'a> SubroutineBody<'a> {
    fn parse(tokenizer: &mut Peekable<Tokenizer<'a>>) -> Result<Self, ParserError<'a>> {
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
    var_type: Type<'a>,
    name: &'a str,
}

impl<'a> SubroutineVariableDeclaration<'a> {}

#[derive(Debug)]
pub(crate) struct Statement {}

impl Statement {
    fn parse<'a>(tokenizer: &mut Peekable<Tokenizer>) -> Result<Self, ParserError<'a>> {
        todo!()
    }
}

fn peek_token(tokenizer: &mut Peekable<Tokenizer>, expected: Token) -> bool {
    let Some(Ok(st)) = tokenizer.peek() else {
        return false;
    };

    st.token == expected
}
