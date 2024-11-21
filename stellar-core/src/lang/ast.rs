use lasso::Spur;

use crate::lang::location::Span;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Sequence {
        name: Name,
        block: Block
    },
    With {
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
