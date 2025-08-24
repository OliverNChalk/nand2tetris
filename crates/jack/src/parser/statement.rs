use hashbrown::HashMap;

use crate::code_gen::{ClassContext, CompileError, SymbolEntry};
use crate::parser::error::ParseError;
use crate::parser::expression::{Expression, SubroutineCall};
use crate::parser::utils::{check_next, eat};
use crate::tokenizer::{Keyword, Symbol, Token, Tokenizer};

#[derive(Debug)]
pub(crate) enum Statement<'a> {
    Let(LetStatement<'a>),
    If(IfStatement<'a>),
    While(WhileStatement<'a>),
    Do(DoStatement<'a>),
    Return(ReturnStatement<'a>),
}

impl<'a> Statement<'a> {
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParseError<'a>> {
        let st = tokenizer.peek_0().ok_or(ParseError::UnexpectedEof)??;
        match st.token {
            Token::Keyword(Keyword::Let) => LetStatement::parse(tokenizer).map(Self::Let),
            Token::Keyword(Keyword::If) => IfStatement::parse(tokenizer).map(Self::If),
            Token::Keyword(Keyword::While) => WhileStatement::parse(tokenizer).map(Self::While),
            Token::Keyword(Keyword::Do) => DoStatement::parse(tokenizer).map(Self::Do),
            Token::Keyword(Keyword::Return) => ReturnStatement::parse(tokenizer).map(Self::Return),
            _ => Err(ParseError::UnexpectedToken(st)),
        }
    }

    pub(crate) fn compile(
        &self,
        class: &ClassContext,
        subroutine: &HashMap<&str, SymbolEntry>,
    ) -> Result<Vec<String>, CompileError<'a>> {
        match self {
            Self::Let(stmt) => stmt.compile(class, subroutine),
            Self::If(stmt) => stmt.compile(class, subroutine),
            Self::While(stmt) => stmt.compile(class, subroutine),
            Self::Do(stmt) => stmt.compile(class, subroutine),
            Self::Return(stmt) => stmt.compile(class, subroutine),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LetStatement<'a> {
    pub(crate) var_name: &'a str,
    pub(crate) index: Option<Expression<'a>>,
    pub(crate) expression: Expression<'a>,
}

impl<'a> LetStatement<'a> {
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParseError<'a>> {
        eat!(tokenizer, Token::Keyword(Keyword::Let))?;
        let var_name = eat!(tokenizer, Token::Identifier)?;

        // Handle index case.
        let index = match check_next(tokenizer, Token::Symbol(Symbol::LeftBracket)) {
            true => {
                eat!(tokenizer, Token::Symbol(Symbol::LeftBracket))?;
                let expression = Expression::parse(tokenizer)?;
                eat!(tokenizer, Token::Symbol(Symbol::RightBracket))?;

                Some(expression)
            }
            false => None,
        };

        // Eat the assignment.
        eat!(tokenizer, Token::Symbol(Symbol::Equals))?;
        let expression = Expression::parse(tokenizer)?;
        eat!(tokenizer, Token::Symbol(Symbol::Semicolon))?;

        Ok(LetStatement { var_name, index, expression })
    }

    pub(crate) fn compile(
        &self,
        class: &ClassContext,
        subroutine: &HashMap<&str, SymbolEntry>,
    ) -> Result<Vec<String>, CompileError<'a>> {
        // Compute the right hand side of the assignment.
        //
        // [RHS]
        let mut code = self.expression.compile(class, subroutine)?;

        // Compute the region in memory to store the expression result.
        let symbol = subroutine
            .get(self.var_name)
            .or_else(|| class.symbols.get(self.var_name))
            .ok_or(CompileError::UnknownSymbol(self.var_name))?;
        match &self.index {
            Some(expression) => {
                // [RHS, symbol]
                code.push(symbol.compile_push());
                // [RHS, symbol, expression]
                code.extend(expression.compile(class, subroutine)?);
                // [RHS, symbol[expression]]
                code.push("add".to_string());
                // At this stage we have that configured accurately.
                //
                // [RHS]
                code.push("pop pointer 1".to_string());
                // []
                code.push("pop that 0".to_string());
            }
            // []
            None => code.push(symbol.compile_pop()),
        }

        Ok(code)
    }
}

#[derive(Debug)]
pub(crate) struct IfStatement<'a> {
    pub(crate) condition: Expression<'a>,
    pub(crate) if_statements: Vec<Statement<'a>>,
    pub(crate) else_statements: Vec<Statement<'a>>,
}

impl<'a> IfStatement<'a> {
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParseError<'a>> {
        // Eat the condition expression.
        eat!(tokenizer, Token::Keyword(Keyword::If))?;
        eat!(tokenizer, Token::Symbol(Symbol::LeftParen))?;
        let condition = Expression::parse(tokenizer)?;
        eat!(tokenizer, Token::Symbol(Symbol::RightParen))?;

        // Eat the braces & all statements
        eat!(tokenizer, Token::Symbol(Symbol::LeftBrace))?;
        let mut if_statements = Vec::default();
        while !check_next(tokenizer, Token::Symbol(Symbol::RightBrace)) {
            if_statements.push(Statement::parse(tokenizer)?);
        }
        eat!(tokenizer, Token::Symbol(Symbol::RightBrace))?;

        // Maybe eat the else statements.
        let mut else_statements = Vec::default();
        if check_next(tokenizer, Token::Keyword(Keyword::Else)) {
            eat!(tokenizer, Token::Keyword(Keyword::Else))?;
            eat!(tokenizer, Token::Symbol(Symbol::LeftBrace))?;
            while !check_next(tokenizer, Token::Symbol(Symbol::RightBrace)) {
                else_statements.push(Statement::parse(tokenizer)?);
            }
            eat!(tokenizer, Token::Symbol(Symbol::RightBrace))?;
        }

        Ok(IfStatement { condition, if_statements, else_statements })
    }

    pub(crate) fn compile(
        &self,
        class: &ClassContext,
        subroutine: &HashMap<&str, SymbolEntry>,
    ) -> Result<Vec<String>, CompileError<'a>> {
        let label0 = class.next_label();
        let label1 = class.next_label();

        let mut code = self.condition.compile(class, subroutine)?;
        code.push("not".to_string());
        code.push(format!("if-goto L{label0}"));
        for stmt in &self.if_statements {
            code.extend(stmt.compile(class, subroutine)?);
        }
        code.push(format!("goto L{label1}"));
        code.push(format!("label L{label0}"));
        for stmt in &self.else_statements {
            code.extend(stmt.compile(class, subroutine)?);
        }
        code.push(format!("label L{label1}"));

        Ok(code)
    }
}

#[derive(Debug)]
pub(crate) struct WhileStatement<'a> {
    pub(crate) condition: Expression<'a>,
    pub(crate) statements: Vec<Statement<'a>>,
}

impl<'a> WhileStatement<'a> {
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParseError<'a>> {
        // Eat the condition expression.
        eat!(tokenizer, Token::Keyword(Keyword::While))?;
        eat!(tokenizer, Token::Symbol(Symbol::LeftParen))?;
        let condition = Expression::parse(tokenizer)?;
        eat!(tokenizer, Token::Symbol(Symbol::RightParen))?;

        // Eat the brace & all statements.
        eat!(tokenizer, Token::Symbol(Symbol::LeftBrace))?;
        let mut statements = Vec::default();
        while !check_next(tokenizer, Token::Symbol(Symbol::RightBrace)) {
            statements.push(Statement::parse(tokenizer)?);
        }
        eat!(tokenizer, Token::Symbol(Symbol::RightBrace))?;

        Ok(WhileStatement { condition, statements })
    }

    pub(crate) fn compile(
        &self,
        class: &ClassContext,
        subroutine: &HashMap<&str, SymbolEntry>,
    ) -> Result<Vec<String>, CompileError<'a>> {
        let label0 = class.next_label();
        let label1 = class.next_label();

        let mut code = vec![format!("label L{label0}")];
        code.extend(self.condition.compile(class, subroutine)?);
        code.push("not".to_string());
        code.push(format!("if-goto L{label1}"));
        for statement in &self.statements {
            code.extend(statement.compile(class, subroutine)?);
        }
        code.push(format!("goto L{label0}"));
        code.push(format!("label L{label1}"));

        Ok(code)
    }
}

#[derive(Debug)]
pub(crate) struct DoStatement<'a> {
    pub(crate) call: SubroutineCall<'a>,
}

impl<'a> DoStatement<'a> {
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParseError<'a>> {
        eat!(tokenizer, Token::Keyword(Keyword::Do))?;
        let call = SubroutineCall::parse(tokenizer)?;
        eat!(tokenizer, Token::Symbol(Symbol::Semicolon))?;

        Ok(DoStatement { call })
    }

    pub(crate) fn compile(
        &self,
        class: &ClassContext,
        subroutine: &HashMap<&str, SymbolEntry>,
    ) -> Result<Vec<String>, CompileError<'a>> {
        let mut code = self.call.compile(class, subroutine)?;
        code.push("pop temp 0".to_string());

        Ok(code)
    }
}

#[derive(Debug)]
pub(crate) struct ReturnStatement<'a> {
    pub(crate) return_value: Option<Expression<'a>>,
}

impl<'a> ReturnStatement<'a> {
    pub(crate) fn parse(tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParseError<'a>> {
        eat!(tokenizer, Token::Keyword(Keyword::Return))?;
        let return_value = match check_next(tokenizer, Token::Symbol(Symbol::Semicolon)) {
            true => None,
            false => Some(Expression::parse(tokenizer)?),
        };
        eat!(tokenizer, Token::Symbol(Symbol::Semicolon))?;

        Ok(ReturnStatement { return_value })
    }

    pub(crate) fn compile(
        &self,
        class: &ClassContext,
        subroutine: &HashMap<&str, SymbolEntry>,
    ) -> Result<Vec<String>, CompileError<'a>> {
        let mut code = match &self.return_value {
            Some(expression) => expression.compile(class, subroutine)?,
            None => vec!["push constant 0".to_string()],
        };
        code.push("return".to_string());

        Ok(code)
    }
}
