use std::iter::Peekable;

pub(crate) struct Tokenizer<'a> {
    source: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self { source }
    }

    fn try_parse_symbol(&mut self) -> Option<Symbol> {
        let next = self.source.split_ascii_whitespace().next()?;
        let symbol = match next.chars().next().unwrap() {
            '{' => Symbol::LeftBrace,
            '}' => Symbol::RightBrace,
            '(' => Symbol::LeftParen,
            ')' => Symbol::RightParen,
            '[' => Symbol::LeftBracket,
            ']' => Symbol::RightBracket,
            '.' => Symbol::Dot,
            ',' => Symbol::Comma,
            ';' => Symbol::Semicolon,
            '+' => Symbol::Plus,
            '-' => Symbol::Minus,
            '*' => Symbol::Asterisk,
            '/' => Symbol::ForwardSlash,
            '&' => Symbol::Ampersand,
            '|' => Symbol::Pipe,
            '<' => Symbol::LeftAngleBracket,
            '>' => Symbol::RightAngleBracket,
            '=' => Symbol::Equals,
            '~' => Symbol::Tilde,
            _ => return None,
        };

        // Advance our position in the source.
        self.source = &self.source[1..];

        Some(symbol)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizeError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.source.is_empty() {
                return None;
            }

            // If this line is a comment, skip it.
            if self
                .source
                .as_bytes()
                .trim_ascii_start()
                .get(0..2)
                .is_some_and(|chars| chars == b"//")
            {
                self.source = self
                    .source
                    .trim_ascii_start()
                    .bytes()
                    .position(|byte| byte == b'\n')
                    .map_or(&self.source[0..0], |pos| {
                        let rem = &self.source.as_bytes()[pos..];

                        unsafe { core::str::from_utf8_unchecked(rem) }
                    });

                continue;
            }

            // Try eat a symbol.
            if let Some(symbol) = self.try_parse_symbol() {
                return Some(Ok(Token::Symbol(symbol)));
            }

            // Try eat a keyword.

            todo!()
        }
    }
}

pub(crate) enum TokenizeError {}

pub(crate) enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
}

pub(crate) enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

pub(crate) enum Symbol {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    Ampersand,
    Pipe,
    LeftAngleBracket,
    RightAngleBracket,
    Equals,
    Tilde,
}
