use crate::lang::{
    ast::{Block, Expression, Statement},
    location::{Span, Spanned},
    token::{Keyword, Punctuator, Token, TokenStream, TokenStreamCursor},
};

use super::{
    ast::{BinaryOperator, PrefixOperator, Property},
    token::{Identifier, Operator},
};

/// Processes a given token stream and converts into an Abstract Syntax Tree.
pub fn parse(stream: TokenStream) -> Result<Vec<Statement>, ParseError> {
    let Some(mut cursor) = stream.into_cursor() else {
        return Err(ParseError::InvalidTokenStream);
    };
    let mut statements = Vec::new();

    loop {
        skip_end_of_lines(&mut cursor);

        if cursor.peek().is_end_of_file() {
            break;
        }

        statements.push(parse_statement(&mut cursor)?);
    }

    Ok(statements)
}

fn parse_block(cursor: &mut TokenStreamCursor) -> Result<Block, ParseError> {
    let start = parse_punctuator(cursor, Punctuator::LeftBrace)?
        .span()
        .start(); // '{'
    let mut statements = Vec::new();

    loop {
        skip_end_of_lines(cursor);

        if cursor.peek().is_punctuator(Punctuator::RightBrace) {
            break;
        }

        statements.push(parse_statement(cursor)?);
    }

    let end = cursor.next().span().end(); // '}'

    Ok(Block {
        statements,
        span: Span::new(start, end),
    })
}

fn parse_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    match cursor.peek() {
        token if token.is_keyword(Keyword::Play) => {
            cursor.next(); // 'play' keyword

            Ok(Statement::Play {
                expression: parse_expression(cursor)?,
            })
        }
        token if token.is_keyword(Keyword::Wait) => {
            cursor.next(); // 'wait' keyword

            Ok(Statement::Wait {
                expression: parse_expression(cursor)?,
            })
        }
        token if token.is_keyword(Keyword::Sequence) => parse_sequence_statement(cursor),
        token if token.is_keyword(Keyword::With) => parse_with_statement(cursor),
        token if token.is_keyword(Keyword::Let) => parse_let_statement(cursor),
        _ => parse_expression(cursor).map(|e| Statement::Expression(e)),
    }
}

fn parse_expression(cursor: &mut TokenStreamCursor) -> Result<Expression, ParseError> {
    parse_expression_with_precedence(cursor, 0)
}

fn parse_expression_with_precedence(
    cursor: &mut TokenStreamCursor,
    precedence: usize,
) -> Result<Expression, ParseError> {
    let mut left = parse_prefix_expression(cursor)?;

    while let Token::Operator {
        operator,
        span: operator_span,
    } = cursor.peek()
    {
        let Some(binary_operator_kind) = operator.into_binary_operator_kind() else {
            continue;
        };

        let binary_operator_precedence = binary_operator_kind.precedence();
        if binary_operator_precedence < precedence {
            break;
        }

        cursor.next();

        // 1 |  3 +
        // 2 |  2 # Expression is continued on the new line
        skip_end_of_lines(cursor);

        let right = parse_expression_with_precedence(cursor, binary_operator_precedence + 1)?;
        left = Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator {
                kind: binary_operator_kind,
                span: operator_span,
            },
            right: Box::new(right),
        };
    }

    Ok(left)
}

/// Parses a left hand side of the potential binary expression.
fn parse_prefix_expression(cursor: &mut TokenStreamCursor) -> Result<Expression, ParseError> {
    match cursor.next() {
        // Literals.
        Token::Integer { value, span } => Ok(Expression::Integer { value, span }),
        Token::Float { value, span } => Ok(Expression::Float { value, span }),
        Token::Identifier(identifier) => Ok(Expression::Identifier(identifier)),
        // Parenthesized expression.
        token if token.is_punctuator(Punctuator::LeftParen) => {
            let expression = parse_expression(cursor)?;

            parse_punctuator(cursor, Punctuator::RightParen)?; // ')'

            Ok(expression)
        }
        // List expression.
        token if token.is_punctuator(Punctuator::LeftBracket) => {
            let mut expressions = Vec::new();

            skip_end_of_lines(cursor);

            if !cursor.peek().is_punctuator(Punctuator::RightBracket) {
                expressions.push(parse_expression(cursor)?);

                skip_end_of_lines(cursor);

                while cursor.peek().is_punctuator(Punctuator::Comma) {
                    cursor.next();

                    skip_end_of_lines(cursor);

                    if cursor.peek().is_punctuator(Punctuator::RightBracket) {
                        break; // [a, b,] - still counts
                    }

                    expressions.push(parse_expression(cursor)?)
                }
            }

            let end = cursor.next().span().end();

            Ok(Expression::List {
                expressions,
                span: Span::new(token.span().start(), end),
            })
        }
        token if token.is_keyword(Keyword::LoadSample) => {
            let sample = parse_expression(cursor)?;

            Ok(Expression::LoadSample {
                span: Span::new(token.span().start(), sample.span().end()),
                sample: Box::new(sample),
            })
        }
        token @ Token::Operator {
            operator,
            span: operator_span,
        } => {
            let Some(prefix_operator_kind) = operator.into_prefix_operator_kind() else {
                return Err(ParseError::ExpectedExpression { got: token });
            };

            let operand = parse_prefix_expression(cursor)?;

            Ok(Expression::Prefix {
                operator: PrefixOperator {
                    kind: prefix_operator_kind,
                    span: operator_span,
                },
                operand: Box::new(operand),
            })
        }
        token => Err(ParseError::ExpectedExpression { got: token }),
    }
}

fn parse_sequence_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    cursor.next(); // 'sequence' keyword

    let name = parse_identifier(cursor)?;
    let block = parse_block(cursor)?;

    Ok(Statement::Sequence { name, block })
}

fn parse_let_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    cursor.next(); // 'let'

    let name = parse_identifier(cursor)?;

    skip_end_of_lines(cursor);
    parse_operator(cursor, Operator::Assign)?;
    skip_end_of_lines(cursor);

    let value = parse_expression(cursor)?;

    Ok(Statement::Let { name, value })
}

fn parse_with_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    cursor.next(); // 'with' keyword

    // <property> ::= <name> ':' <value>
    fn parse_property(cursor: &mut TokenStreamCursor) -> Result<Property, ParseError> {
        let name = parse_identifier(cursor)?;
        parse_punctuator(cursor, Punctuator::Colon)?;

        let value = parse_expression(cursor)?;

        Ok(Property { name, value })
    }

    let mut properties = Vec::new();
    properties.push(parse_property(cursor)?);

    while cursor.peek().is_punctuator(Punctuator::Comma) {
        cursor.next();

        if cursor.peek().is_punctuator(Punctuator::LeftBrace) {
            break; // with a: test, b: 3, { } - still counts
        }

        properties.push(parse_property(cursor)?)
    }

    let block = parse_block(cursor)?;

    Ok(Statement::With { properties, block })
}

/// Checks if the next token in cursor is an identifier:
/// - If it is, returns an [`Identifier`] object.
/// - If it is not, returns a [`ParseError`].
///
/// In both cases, cursor is moved to the next token.
fn parse_identifier(cursor: &mut TokenStreamCursor) -> Result<Identifier, ParseError> {
    let got = cursor.next();
    let Token::Identifier(identifier) = got else {
        return Err(ParseError::ExpectedIdentifier { got });
    };

    Ok(identifier)
}

/// Checks if the next token in cursor is a punctuator given in a function argument `punctuator`:
/// - If it is, returns a [`Token`] object.
/// - If it is not, returns a [`ParseError`].
///
/// In both cases, cursor is moved to the next token.
fn parse_punctuator(
    cursor: &mut TokenStreamCursor,
    punctuator: Punctuator,
) -> Result<Token, ParseError> {
    let got = cursor.next();
    if !got.is_punctuator(punctuator) {
        return Err(ParseError::ExpectedPunctuation {
            expected: punctuator,
            got,
        });
    }

    Ok(got)
}

/// Checks if the next token in cursor is an operator given in a function argument `operator`:
/// - If it is, returns a [`Token`] object.
/// - If it is not, returns a [`ParseError`].
///
/// In both cases, cursor is moved to the next token.
fn parse_operator(cursor: &mut TokenStreamCursor, operator: Operator) -> Result<Token, ParseError> {
    let got = cursor.next();
    if !got.is_operator(operator) {
        return Err(ParseError::ExpectedOperator {
            expected: operator,
            got,
        });
    }

    Ok(got)
}

/// Moves cursor to the next non-EOL token.
fn skip_end_of_lines(cursor: &mut TokenStreamCursor) {
    while cursor.peek().is_end_of_line() {
        cursor.next();
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidTokenStream,
    ExpectedExpression { got: Token },
    ExpectedIdentifier { got: Token },
    ExpectedPunctuation { expected: Punctuator, got: Token },
    ExpectedOperator { expected: Operator, got: Token },
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use lasso::Rodeo;

    use crate::{lang::scan::scan, test_parse};

    use super::parse;

    test_parse!(
        (empty, ""),
        (with, "with a: 3, b: 4, {}"),
        (sequence, "sequence test {}"),
        (binary_expr, "a + \n 2 * (3 + b) - 3"),
        (play_and_wait, "play c4 wait 1"),
        (list, "[1, 2]\n[1, \n2]\n[\n1, \n2]\n[1,\n2,]"),
        (let_stmt, "let a = 3 + 2")
    );
}
