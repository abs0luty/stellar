use lasso::Spur;

use crate::location::Span;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Sequence {
        name: Name
    },
    WithChannel {
        channel: Expression,
        block: Block,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Float { value: f64, span: Span },
    Integer { value: i64, span: Span },
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span
}

#[derive(Debug, PartialEq)]
pub struct Name {
    pub name: Spur,
    pub span: Span,
}
