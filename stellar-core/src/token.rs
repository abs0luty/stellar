use lasso::Spur;

use crate::location::{ByteLocation, Span, Spanned};

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
        last_byte_location: ByteLocation,
    },
}

impl Spanned for Token {
    fn span(&self) -> Span {
        match self {
            Self::EOF { last_byte_location} => Span {
                start: *last_byte_location,
                end: last_byte_location.next_byte_location(),
            },
            Self::Identifier { span, .. }
            | Self::Keyword { span, .. } => span.to_owned(),
        }
    }
}
