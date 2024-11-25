use crate::syntax::location::Span;

use super::{location::Spanned, string_id::StringId, token::Identifier};

#[derive(Debug, PartialEq)]
pub struct Property {
    pub name: Identifier,
    pub value: Expression,
}

impl Spanned for Property {
    fn span(&self) -> Span {
        Span::new(self.name.span().start(), self.value.span().end())
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Wait {
        expression: Expression,
    },
    Play {
        expression: Expression,
    },
    Sequence {
        name: Identifier,
        block: Block,
    },
    With {
        properties: Vec<Property>,
        block: Block,
    },
    Let {
        name: Identifier,
        value: Expression,
    },
    Expression(Expression),
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
    String {
        value: StringId,
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
    List {
        expressions: Vec<Expression>,
        span: Span,
    },
    Identifier(Identifier),
    LoadSample {
        sample: Box<Expression>,
        span: Span,
    },
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Self::Binary { left, right, .. } => Span::new(left.span().start(), right.span().end()),
            Self::Prefix { operator, operand } => {
                Span::new(operator.span().start(), operand.span().end())
            }
            Self::List { span, .. }
            | Self::Bool { span, .. }
            | Self::Float { span, .. }
            | Self::String { span, .. }
            | Self::Integer { span, .. }
            | Self::LoadSample { span, .. } => *span,
            Self::Identifier(identifier) => identifier.span(),
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
    Star,
    Slash,
    Assign,
}

impl BinaryOperatorKind {
    pub fn precedence(&self) -> usize {
        match self {
            Self::Assign => 1,
            Self::Plus | Self::Minus => 2,
            Self::Star | Self::Slash => 3,
        }
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
