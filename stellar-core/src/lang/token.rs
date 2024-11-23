use lasso::Spur;

use crate::lang::location::{Location, Span, Spanned};

use super::ast::{BinaryOperatorKind, PrefixOperatorKind};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Keyword {
    Play,
    With,
    Wait,
    Sequence,
    LoadSample,
    Let
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Punctuator {
    Exclamation,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Dot,
    Colon,
    Comma,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
    PlusEq,
    MinusEq,
    Star,
    Slash,
    Assign,
    Eq,
    Exclamation
}

impl Operator {
    pub fn into_binary_operator_kind(&self) -> Option<BinaryOperatorKind> {
        match self {
            Self::Plus => Some(BinaryOperatorKind::Plus),
            Self::Minus => Some(BinaryOperatorKind::Minus),
            Self::Star => Some(BinaryOperatorKind::Star),
            Self::Slash => Some(BinaryOperatorKind::Slash),
            Self::Assign => Some(BinaryOperatorKind::Assign),
            _ => None,
        }
    }

    pub fn into_prefix_operator_kind(&self) -> Option<PrefixOperatorKind> {
        match self {
            Self::Exclamation => Some(PrefixOperatorKind::Exclamation),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Identifier {
    name: Spur,
    span: Span,
}

impl Identifier {
    pub fn new(name: Spur, span: Span) -> Self {
        Self { name, span }
    }

    pub fn name(&self) -> Spur {
        self.name
    }
}

impl Spanned for Identifier {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Keyword {
        keyword: Keyword,
        span: Span,
    },
    Identifier(Identifier),
    Operator {
        operator: Operator,
        span: Span,
    },
    Punctuator {
        punctuator: Punctuator,
        span: Span,
    },
    Float {
        value: f64,
        span: Span,
    },
    Integer {
        value: i64,
        span: Span,
    },
    Bool {
        value: bool,
        span: Span,
    },
    EndOfLine {
        span: Span,
    },
    EndOfFile {
        /// Location of the last byte in the source file.
        location: Location,
    },
}

impl Token {
    pub fn is_end_of_file(&self) -> bool {
        matches!(self, Self::EndOfFile { .. })
    }

    pub fn is_end_of_line(&self) -> bool {
        matches!(self, Self::EndOfLine { .. })
    }

    pub fn is_keyword(&self, keyword: Keyword) -> bool {
        match self {
            Self::Keyword {
                keyword: my_keyword,
                ..
            } => keyword == *my_keyword,
            _ => false,
        }
    }

    pub fn is_punctuator(&self, punctuator: Punctuator) -> bool {
        match self {
            Self::Punctuator {
                punctuator: my_punctuator,
                ..
            } => punctuator == *my_punctuator,
            _ => false,
        }
    }

    pub fn is_operator(&self, operator: Operator) -> bool {
        match self {
            Self::Operator {
                operator: my_operator,
                ..
            } => operator == *my_operator,
            _ => false,
        }
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier { .. })
    }
}

impl Spanned for Token {
    fn span(&self) -> Span {
        match self {
            Self::EndOfFile { location } => Span::new(
                *location,
                Location::new(location.line(), location.column() + 1, location.index() + 1),
            ),
            Self::Identifier(Identifier { span, .. })
            | Self::Punctuator { span, .. }
            | Self::Operator { span, .. }
            | Self::Keyword { span, .. }
            | Self::Integer { span, .. }
            | Self::Float { span, .. }
            | Self::Bool { span, .. }
            | Self::EndOfLine { span } => *span,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TokenStream(Vec<Token>);

impl TokenStream {
    /// Creates a new empty token stream.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Appends a token to the stream.
    pub fn push(&mut self, token: Token) {
        self.0.push(token);
    }

    /// Returns token with a specified index in the stream. In case index
    /// is out of bounds, EOF token (End Of File) is returned.
    fn get(&self, index: usize) -> Token {
        if index > self.0.len() {
            self.0.last().copied().unwrap_or(Token::EndOfFile {
                location: Location::sof(),
            })
        } else {
            self.0[index]
        }
    }

    /// Returns a cursor over the token stream. See [`TokenStreamCursor`] for more details.
    pub fn into_cursor(self) -> Option<TokenStreamCursor> {
        // Ensure last token is EOF.
        if self.0.last().map_or(true, |maybe_eof| !maybe_eof.is_end_of_file()) {
            return None;
        }

        Some(TokenStreamCursor::new(self))
    }
}

/// A cursor for navigating through a stream of tokens.
///
/// This struct provides functionality to sequentially traverse
/// a [`TokenStream`], allowing you to retrieve tokens one at a time
/// or peek at the upcoming token without advancing the cursor.
/// It tracks the current position in the stream and ensures
/// that an **EOF (End Of File) token is returned when no more
/// tokens are available**.
pub struct TokenStreamCursor {
    stream: TokenStream,
    location: usize,
}

impl TokenStreamCursor {
    fn new(stream: TokenStream) -> Self {
        Self {
            stream,
            location: 0,
        }
    }

    /// Retrieves the next token from the stream, advancing the cursor
    /// to the subsequent position. If no more tokens are available,
    /// an EOF (End Of File) token is returned.
    pub fn next(&mut self) -> Token {
        self.location += 1;

        self.stream.get(self.location - 1)
    }

    /// Provides a glimpse of the next token without advancing the cursor
    /// (compared to [`TokenStreamCursor::next`]). If no more tokens are
    /// available, an EOF (End Of File) token is returned.
    pub fn peek(&mut self) -> Token {
        self.stream.get(self.location)
    }
}

impl IntoIterator for TokenStream {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = Token;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
