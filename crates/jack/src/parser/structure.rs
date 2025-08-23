use crate::parser::error::ParserError;
use crate::parser::statement::Statement;
use crate::parser::utils::{check_next, eat, peek};
use crate::tokenizer::{Keyword, SourceToken, Symbol, Token, Tokenizer};

#[derive(Debug)]
pub(crate) struct Class<'a> {
    pub(crate) name: String,
    pub(crate) variables: Vec<ClassVariableDeclaration<'a>>,
    pub(crate) subroutines: Vec<SubroutineDeclaration<'a>>,
}

impl<'a> Class<'a> {
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError<'a>> {
        // All Jack files must contain exactly one class, so lets start by eating the
        // beginning of the class declaration.
        eat!(tokenizer, Token::Keyword(Keyword::Class))?;
        let class_name = eat!(tokenizer, Token::Identifier)?;
        eat!(tokenizer, Token::Symbol(Symbol::LeftBrace))?;

        // Eat all the class variables.
        let mut variables = Vec::default();
        while matches!(peek(tokenizer), Some(Token::Keyword(Keyword::Static | Keyword::Field))) {
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
            variables.push(ClassVariableDeclaration { modifier, var_type, name });

            // Eat remaining the variable declarations.
            while check_next(tokenizer, Token::Symbol(Symbol::Comma)) {
                // Eat the comma.
                eat!(tokenizer, Token::Symbol(Symbol::Comma))?;

                // Eat the next variable name.
                let name = eat!(tokenizer, Token::Identifier)?;

                variables.push(ClassVariableDeclaration { modifier, var_type, name })
            }
            eat!(tokenizer, Token::Symbol(Symbol::Semicolon))?;
        }

        // Eat all the subroutines.
        let mut subroutines = Vec::default();
        while matches!(
            peek(tokenizer),
            Some(Token::Keyword(Keyword::Constructor | Keyword::Function | Keyword::Method))
        ) {
            // Parse the subroutine body.
            subroutines.push(SubroutineDeclaration::parse(tokenizer)?);
        }

        // Next we eat the body of the class.
        let class = Class { name: class_name.to_string(), variables, subroutines };

        // Finally we finish up the class declaration.
        eat!(tokenizer, Token::Symbol(Symbol::RightBrace))?;
        assert!(tokenizer.next().is_none());

        Ok(class)
    }
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

impl<'a> SubroutineDeclaration<'a> {
    fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError<'a>> {
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

        let body = SubroutineBody::parse(tokenizer)?;

        Ok(SubroutineDeclaration { subroutine_type, return_type, name, parameters, body })
    }
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
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError<'a>> {
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
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError<'a>> {
        eat!(tokenizer, Token::Symbol(Symbol::LeftBrace))?;

        // Eat all variable declarations.
        let mut variables = Vec::default();
        while check_next(tokenizer, Token::Keyword(Keyword::Var)) {
            // Eat the first variable.
            eat!(tokenizer, Token::Keyword(Keyword::Var))?;
            let var_type = Type::parse(tokenizer)?;
            let name = eat!(tokenizer, Token::Identifier)?;

            // Eat the remaining variables.
            variables.push(SubroutineVariableDeclaration { var_type, name });
            while check_next(tokenizer, Token::Symbol(Symbol::Comma)) {
                eat!(tokenizer, Token::Symbol(Symbol::Comma))?;
                variables.push(SubroutineVariableDeclaration {
                    var_type,
                    name: eat!(tokenizer, Token::Identifier)?,
                });
            }

            eat!(tokenizer, Token::Symbol(Symbol::Semicolon))?;
        }

        // Eat all statements.
        let mut statements = Vec::default();
        while !check_next(tokenizer, Token::Symbol(Symbol::RightBrace)) {
            statements.push(Statement::parse(tokenizer)?);
        }
        eat!(tokenizer, Token::Symbol(Symbol::RightBrace))?;

        Ok(SubroutineBody { variables, statements })
    }

    pub(crate) fn compile(&self) -> Vec<String> {
        self.statements
            .iter()
            .flat_map(|statement| statement.compile())
            .collect()
    }
}

#[derive(Debug)]
pub(crate) struct SubroutineVariableDeclaration<'a> {
    pub(crate) var_type: Type<'a>,
    pub(crate) name: &'a str,
}
