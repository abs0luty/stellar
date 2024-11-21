use lasso::Spur;

use crate::lang::location::Span;

use super::location::Spanned;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Sequence { name: Name, block: Block },
    With { block: Block },
    Expression(Expression)
}

#[derive(Debug, PartialEq)]
pub enum Expression {
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
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Prefix {
        operator: PrefixOperator,
        operand: Box<Expression>,
    },
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Self::Binary { left, right, .. } => Span::new(left.span().start(), right.span().end()),
            Self::Prefix { operator, operand } => {
                Span::new(operator.span().start(), operand.span().end())
            }
            Self::Bool { span, .. } | Self::Float { span, .. } | Self::Integer { span, .. } => {
                *span
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PrefixOperatorKind {
    Exclamation,
}

#[derive(Debug, PartialEq)]
pub struct PrefixOperator {
    pub kind: PrefixOperatorKind,
    pub span: Span,
}

impl Spanned for PrefixOperator {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperatorKind {
    Plus,
    Minus,
}

impl BinaryOperatorKind {
    pub fn precedence(&self) -> usize {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct BinaryOperator {
    pub kind: BinaryOperatorKind,
    pub span: Span,
}

impl BinaryOperator {
    pub fn precedence(&self) -> usize {
        self.kind.precedence()
    }
}

impl Spanned for BinaryOperator {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct Name {
    pub name: Spur,
    pub span: Span,
}
