use lasso::Spur;

use crate::location::{Location, Span, Spanned};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Keyword {
    WithFx,
    WithSynth,
    Wait
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword {
        keyword: Keyword,
        span: Span,
    },
    Identifier {
        name: Spur,
        span: Span,
    },
    EOF {
        /// Location of the last byte in the source file.
        location: Location,
    },
}

impl Spanned for Token {
    fn span(&self) -> Span {
        match self {
            Self::EOF { location} => Span {
                start: *location,
                end: Location::new(location.line, location.column + 1, location.index + 1)
            },
            Self::Identifier { span, .. }
            | Self::Keyword { span, .. } => *span,
        }
    }
}
