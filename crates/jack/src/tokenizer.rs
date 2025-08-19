pub(crate) struct Tokenizer<'a> {
    source: &'a str,
    errored: bool,
}

impl<'a> Tokenizer<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self { source, errored: false }
    }

    fn try_parse_symbol(&mut self) -> Option<Symbol> {
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

        // Advance our position in the source.
        self.source = &self.source[1..];

        Some(symbol)
    }

    fn try_parse_keyword(&mut self) -> Option<Keyword> {
        debug_assert!(self.source.as_bytes()[0] != b' ');

        // Keywords are terminated by a space.
        let word = match self.source.as_bytes().iter().position(|byte| byte == &b' ') {
            // SAFETY: As no ASCII characters overlap with UTF8 multi byte characters, we can
            // safely assume that if we find a space and then index that space, we will not be
            // splitting any UTF-8 chars.
            Some(end) => unsafe { core::str::from_utf8_unchecked(&self.source.as_bytes()[..end]) },
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

        // SAFETY: As we know `word` to be valid UTF8, we can safely trim the entire
        // word without splitting a UTF-8 char boundary.
        self.source =
            unsafe { core::str::from_utf8_unchecked(&self.source.as_bytes()[word.len()..]) };

        Some(keyword)
    }

    fn try_parse_identifier(&mut self) -> Option<&'a str> {
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
            // SAFETY: As no ASCII characters overlap with UTF8 multi byte characters, we can
            // safely assume that if we find a space and then index that space, we will not be
            // splitting any UTF-8 chars.
            Some(end) => unsafe { core::str::from_utf8_unchecked(&self.source.as_bytes()[..end]) },
            None => self.source,
        };

        // Bail if we could not extract a valid identifier.
        if identifier.is_empty() {
            return None;
        }

        // SAFETY: As we know `word` to be valid UTF8, we can safely trim the entire
        // word without splitting a UTF-8 char boundary.
        self.source =
            unsafe { core::str::from_utf8_unchecked(&self.source.as_bytes()[identifier.len()..]) };

        Some(identifier)
    }

    fn try_parse_integer_literal(&mut self) -> Option<i16> {
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

        // SAFETY: As we know `word` to be valid UTF8, we can safely trim the entire
        // word without splitting a UTF-8 char boundary.
        self.source =
            unsafe { core::str::from_utf8_unchecked(&self.source.as_bytes()[literal_s.len()..]) };

        Some(literal)
    }

    fn try_parse_string_literal(&mut self) -> Option<&'a str> {
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

        // SAFETY: As we know `word` to be valid UTF8, we can safely trim the entire
        // word without splitting a UTF-8 char boundary.
        self.source =
            unsafe { core::str::from_utf8_unchecked(&self.source.as_bytes()[literal.len()..]) };

        Some(literal)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, TokenizeError>;

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
            if let Some(symbol) = self.try_parse_symbol() {
                return Some(Ok(Token::Symbol(symbol)));
            }

            // Try eat a keyword.
            if let Some(keyword) = self.try_parse_keyword() {
                return Some(Ok(Token::Keyword(keyword)));
            }

            // Try eat an identifier.
            if let Some(identifier) = self.try_parse_identifier() {
                return Some(Ok(Token::Identifier(identifier)));
            }

            // Try eat an integer literal.
            if let Some(literal) = self.try_parse_integer_literal() {
                return Some(Ok(Token::IntegerLiteral(literal)));
            }

            // Try eat a string literal.
            if let Some(literal) = self.try_parse_string_literal() {
                return Some(Ok(Token::StringLiteral(literal)));
            }

            todo!("Could not parse a token");
        }
    }
}

#[derive(Debug)]
pub(crate) enum TokenizeError {
    UnclosedComment,
}

#[derive(Debug)]
pub(crate) enum Token<'a> {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(&'a str),
    IntegerLiteral(i16),
    StringLiteral(&'a str),
}

#[derive(Debug)]
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

#[derive(Debug)]
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
