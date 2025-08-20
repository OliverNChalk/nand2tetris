// TODO: Remove once we start on the AST.
#![allow(dead_code)]

use std::io::Write;

use strum::IntoStaticStr;

pub(crate) struct Tokenizer<'a> {
    source: &'a str,
    errored: bool,
}

impl<'a> Tokenizer<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self { source, errored: false }
    }

    fn try_parse_symbol(&mut self) -> Option<SourceToken<'a>> {
        debug_assert!(self.source.as_bytes()[0] != b' ');

        let symbol = match self.source.as_bytes().first()? {
            b'{' => Symbol::LeftBrace,
            b'}' => Symbol::RightBrace,
            b'(' => Symbol::LeftParen,
            b')' => Symbol::RightParen,
            b'[' => Symbol::LeftBracket,
            b']' => Symbol::RightBracket,
            b'.' => Symbol::Dot,
            b',' => Symbol::Comma,
            b';' => Symbol::Semicolon,
            b'+' => Symbol::Plus,
            b'-' => Symbol::Minus,
            b'*' => Symbol::Asterisk,
            b'/' => Symbol::ForwardSlash,
            b'&' => Symbol::Ampersand,
            b'|' => Symbol::Pipe,
            b'<' => Symbol::LeftAngleBracket,
            b'>' => Symbol::RightAngleBracket,
            b'=' => Symbol::Equals,
            b'~' => Symbol::Tilde,
            _ => return None,
        };

        // Construct our token.
        let token = SourceToken { source: &self.source[..1], token: Token::Symbol(symbol) };

        // Advance our position in the source.
        self.source = &self.source[1..];

        Some(token)
    }

    fn try_parse_keyword(&mut self) -> Option<SourceToken<'a>> {
        debug_assert!(self.source.as_bytes()[0] != b' ');

        // Keywords contain only alphabetical characters.
        let word = match self
            .source
            .as_bytes()
            .iter()
            .position(|byte| !byte.is_ascii_alphabetic())
        {
            Some(end) => &self.source[..end],
            None => self.source,
        };

        let keyword = match word {
            "class" => Keyword::Class,
            "constructor" => Keyword::Constructor,
            "function" => Keyword::Function,
            "method" => Keyword::Method,
            "field" => Keyword::Field,
            "static" => Keyword::Static,
            "var" => Keyword::Var,
            "int" => Keyword::Int,
            "char" => Keyword::Char,
            "boolean" => Keyword::Boolean,
            "void" => Keyword::Void,
            "true" => Keyword::True,
            "false" => Keyword::False,
            "null" => Keyword::Null,
            "this" => Keyword::This,
            "let" => Keyword::Let,
            "do" => Keyword::Do,
            "if" => Keyword::If,
            "else" => Keyword::Else,
            "while" => Keyword::While,
            "return" => Keyword::Return,
            _ => return None,
        };

        // Construct our token.
        let token =
            SourceToken { source: &self.source[..word.len()], token: Token::Keyword(keyword) };

        // Advance our position in the source.
        self.source = &self.source[word.len()..];

        Some(token)
    }

    fn try_parse_identifier(&mut self) -> Option<SourceToken<'a>> {
        debug_assert!(self.source.as_bytes()[0] != b' ');

        // Identifiers cannot start with a digit.
        if self.source.as_bytes()[0].is_ascii_digit() {
            return None;
        }

        // Identifiers can be terminated by various symbols.
        let identifier = match self
            .source
            .as_bytes()
            .iter()
            .position(|byte| !byte.is_ascii_alphanumeric() && byte != &b'_')
        {
            Some(end) => &self.source[..end],
            None => self.source,
        };

        // Bail if we could not extract a valid identifier.
        if identifier.is_empty() {
            return None;
        }

        // Construct our token.
        let token = SourceToken { source: identifier, token: Token::Identifier };

        // SAFETY: As we know `word` to be valid UTF8, we can safely trim the entire
        // word without splitting a UTF-8 char boundary.
        self.source =
            unsafe { core::str::from_utf8_unchecked(&self.source.as_bytes()[identifier.len()..]) };

        Some(token)
    }

    fn try_parse_integer_literal(&mut self) -> Option<SourceToken<'a>> {
        debug_assert!(self.source.as_bytes()[0] != b' ');

        // Integer literals must contain only digits.
        let literal_s = match self
            .source
            .as_bytes()
            .iter()
            .position(|byte| !byte.is_ascii_digit())
        {
            // SAFETY: As no ASCII characters overlap with UTF8 multi byte characters, we can
            // safely assume that if we find a space and then index that space, we will not be
            // splitting any UTF-8 chars.
            Some(end) => unsafe { core::str::from_utf8_unchecked(&self.source.as_bytes()[..end]) },
            None => self.source,
        };

        // If we have no digits then this cannot be an integer literal.
        if literal_s.is_empty() {
            return None;
        }

        // Jack integers are 16bit signed values but only the 0 & positive integers are
        // usable, thus the range is 0..2**15.
        let Ok(literal) = literal_s.parse::<i16>() else {
            todo!("Invalid jack integer literal");
        };

        // Jack integers literals cannot be negative.
        assert!(!literal.is_negative());

        // Construct our token.
        let token = SourceToken { source: literal_s, token: Token::IntegerLiteral(literal) };

        // SAFETY: As we know `word` to be valid UTF8, we can safely trim the entire
        // word without splitting a UTF-8 char boundary.
        self.source =
            unsafe { core::str::from_utf8_unchecked(&self.source.as_bytes()[literal_s.len()..]) };

        Some(token)
    }

    fn try_parse_string_literal(&mut self) -> Option<SourceToken<'a>> {
        debug_assert!(self.source.as_bytes()[0] != b' ');

        // String literals must start with a double quote.
        if self.source.as_bytes()[0] != b'"' {
            return None;
        }

        // String literals are terminated by a double quote.
        let literal = match self
            .source
            .as_bytes()
            .iter()
            .skip(1)
            .position(|byte| byte == &b'"')
        {
            // SAFETY: As no ASCII characters overlap with UTF8 multi byte characters, we can
            // safely assume that if we find a space and then index that space, we will not be
            // splitting any UTF-8 chars.
            Some(end) => unsafe {
                core::str::from_utf8_unchecked(&self.source.as_bytes()[..(end + 2)])
            },
            None => self.source,
        };
        assert!(!literal.is_empty());

        if literal.bytes().any(|byte| byte == b'\n') {
            todo!("Jack specification forbids newlines in string literals");
        }

        // Construct our token.
        let token = SourceToken { source: literal, token: Token::StringLiteral };

        // SAFETY: As we know `word` to be valid UTF8, we can safely trim the entire
        // word without splitting a UTF-8 char boundary.
        self.source =
            unsafe { core::str::from_utf8_unchecked(&self.source.as_bytes()[literal.len()..]) };

        Some(token)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<SourceToken<'a>, TokenizeError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Fuse ourselves if we have errored to prevent callers from suppressing errors
        // and infinite looping.
        if self.errored {
            return None;
        }

        // Loop until we are done or find a valid token.
        loop {
            // Strip any whitespace left over after parsing the previous iteration.
            self.source = self.source.trim_ascii_start();

            // If we have no source left we are done.
            if self.source.is_empty() {
                return None;
            }

            // If this is an empty line, skip it.
            let source = self.source.as_bytes();
            if source[0] == b'\n' {
                // SAFETY: As we are trimming an ASCII byte, there is no possibility of UTF-8
                // char splitting.
                self.source =
                    unsafe { core::str::from_utf8_unchecked(&self.source.as_bytes()[1..]) };

                continue;
            }

            // If this is a single line comment, skip it.
            if source.get(0..2).is_some_and(|chars| chars == b"//") {
                self.source = source.iter().position(|byte| byte == &b'\n').map_or(
                    &self.source[0..0],
                    |pos| {
                        // SAFETY: As we have found a valid ASCII byte, we can be sure this byte
                        // does not belong to some multi-byte UTF-8 char.
                        unsafe { core::str::from_utf8_unchecked(&self.source.as_bytes()[pos..]) }
                    },
                );

                continue;
            }

            // If this is a doc comment comment, skip it.
            if source.get(0..3).is_some_and(|chars| chars == b"/**") {
                let Some(end) = source[3..].windows(2).position(|window| window == b"*/") else {
                    self.errored = true;

                    return Some(Err(TokenizeError::UnclosedComment));
                };

                // SAFETY: This is safe because the end character is in ASCII which cannot be
                // present in a multi-byte UTF-8 character, thus no character splitting can
                // occur.
                self.source = unsafe {
                    core::str::from_utf8_unchecked(&self.source.as_bytes()[(3 + end + 2)..])
                };

                continue;
            }

            // Try eat a symbol.
            if let Some(token) = self.try_parse_symbol() {
                return Some(Ok(token));
            }

            // Try eat a keyword.
            if let Some(token) = self.try_parse_keyword() {
                return Some(Ok(token));
            }

            // Try eat an identifier.
            if let Some(token) = self.try_parse_identifier() {
                return Some(Ok(token));
            }

            // Try eat an integer literal.
            if let Some(token) = self.try_parse_integer_literal() {
                return Some(Ok(token));
            }

            // Try eat a string literal.
            if let Some(token) = self.try_parse_string_literal() {
                return Some(Ok(token));
            }

            todo!("Could not parse a token");
        }
    }
}

#[derive(Debug)]
pub(crate) enum TokenizeError {
    UnclosedComment,
}

pub(crate) struct SourceToken<'a> {
    pub(crate) source: &'a str,
    pub(crate) token: Token,
}

impl<'a> SourceToken<'a> {
    pub(crate) fn write_xml(&self, wx: &mut impl Write) {
        let Self { source, token } = self;

        match token {
            Token::Keyword(_) => {
                write!(wx, "<keyword> {source} </keyword>").unwrap();
            }
            Token::Symbol(symbol) => {
                write!(wx, "<symbol> ").unwrap();
                match symbol {
                    Symbol::Ampersand => write!(wx, "&amp;"),
                    Symbol::LeftAngleBracket => write!(wx, "&lt;"),
                    Symbol::RightAngleBracket => write!(wx, "&gt;"),
                    _ => write!(wx, "{source}"),
                }
                .unwrap();
                write!(wx, " </symbol>").unwrap();
            }
            Token::Identifier => {
                write!(wx, "<identifier> {source} </identifier>").unwrap();
            }
            Token::IntegerLiteral(_) => {
                write!(wx, "<integerConstant> {source} </integerConstant>").unwrap();
            }
            Token::StringLiteral => {
                write!(wx, "<stringConstant> {} </stringConstant>", &source[1..source.len() - 1])
                    .unwrap();
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier,
    IntegerLiteral(i16),
    StringLiteral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoStaticStr)]
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

impl Keyword {
    fn write_xml(&self, wx: &mut impl Write) {
        wx.write_all(<&str>::from(self).as_bytes()).unwrap();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoStaticStr)]
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

impl Symbol {
    fn write_xml(&self, wx: &mut impl Write) {
        wx.write_all(<&str>::from(self).as_bytes()).unwrap();
    }
}
