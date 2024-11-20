use lasso::Spur;

use crate::location::Span;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Sequence {
        name: Name
    }
}

#[derive(Debug, PartialEq)]
pub struct Name {
    pub name: Spur,
    pub span: Span,
}
