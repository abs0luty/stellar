use lasso::Spur;

use crate::lang::location::{Location, Span, Spanned};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Keyword {
    With,
    Wait,
    Sequence,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Punctuation {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Plus,
    Minus,
    PlusEq,
    MinusEq,
    Dot,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Keyword {
        keyword: Keyword,
        span: Span,
    },
    Identifier {
        name: Spur,
        span: Span,
    },
    Punctuation {
        punctuation: Punctuation,
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
    EOF {
        /// Location of the last byte in the source file.
        location: Location,
    },
}

impl Token {
    pub fn is_eof(&self) -> bool {
        matches!(self, Self::EOF { .. })
    }

    pub fn is_punctuation(&self, punctuation: Punctuation) -> bool {
        match self {
            Self::Punctuation {
                punctuation: my_punctuation,
                ..
            } => punctuation == *my_punctuation,
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
            Self::EOF { location } => Span::new(
                *location,
                Location::new(location.line(), location.column() + 1, location.index() + 1),
            ),
            Self::Identifier { span, .. }
            | Self::Punctuation { span, .. }
            | Self::Keyword { span, .. }
            | Self::Integer { span, .. }
            | Self::Float { span, .. } => *span,
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
    /// is out of bounds, EOF token (end of file) is returned.
    pub fn get(&self, index: usize) -> Token {
        if index > self.0.len() {
            self.0.last().copied().unwrap_or(Token::EOF {
                location: Location::sof(),
            })
        } else {
            self.0[index]
        }
    }

    /// Returns a cursor over the token stream. See [`TokenStreamCursor`] for more details.
    pub fn into_cursor(self) -> Option<TokenStreamCursor> {
        // Ensure last token is EOF.
        if self.0.last().map_or(true, |maybe_eof| !maybe_eof.is_eof()) {
            return None;
        }

        Some(TokenStreamCursor::new(self))
    }
}

pub struct TokenStreamCursor {
    stream: TokenStream,
    location: usize,
}

impl TokenStreamCursor {
    pub fn new(stream: TokenStream) -> Self {
        Self {
            stream,
            location: 0,
        }
    }

    /// Returns a new token in the token stream and advances the cursor
    /// position to the next one. In case, no new tokens are present, EOF
    /// token (End Of File) is returned.
    pub fn next(&mut self) -> Token {
        self.location += 1;

        self.stream.get(self.location - 1)
    }

    pub fn peek(&mut self) -> Token {
        self.stream.get(self.location)
    }
}