use crate::lang::{
    ast::{Block, Expression, Statement},
    location::{Span, Spanned},
    token::{Keyword, Punctuator, Token, TokenStream, TokenStreamCursor},
};

use super::{
    ast::{BinaryOperator, PrefixOperator, Property},
    token::{Identifier, Operator},
};

/// Parses a given token stream into Abstract Syntax Tree (AST).
pub fn parse(stream: TokenStream) -> Result<Vec<Statement>, ParseError> {
    let Some(mut cursor) = stream.into_cursor() else {
        return Err(ParseError::InvalidTokenStream);
    };
    let mut statements = Vec::new();

    loop {
        skip_end_of_lines(&mut cursor);

        if cursor.peek().is_eof() {
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
                expression: parse_expression(cursor, 0)?,
            })
        }
        token if token.is_keyword(Keyword::Wait) => {
            cursor.next(); // 'wait' keyword

            Ok(Statement::Wait {
                expression: parse_expression(cursor, 0)?,
            })
        }
        token if token.is_keyword(Keyword::Sequence) => parse_sequence_statement(cursor),
        token if token.is_keyword(Keyword::With) => parse_with_statement(cursor),
        token if token.is_keyword(Keyword::Let) => parse_let_statement(cursor),
        _ => parse_expression(cursor, 0).map(|e| Statement::Expression(e)),
    }
}

fn parse_property(cursor: &mut TokenStreamCursor) -> Result<Property, ParseError> {
    let name = parse_identifier(cursor)?; // name
    parse_punctuator(cursor, Punctuator::Colon)?; // ':'

    let value = parse_expression(cursor, 0)?;

    Ok(Property { name, value })
}

fn parse_let_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    cursor.next(); // 'let'

    let name = parse_identifier(cursor)?;

    skip_end_of_lines(cursor);
    parse_operator(cursor, Operator::Assign)?;
    skip_end_of_lines(cursor);

    let value = parse_expression(cursor, 0)?;

    Ok(Statement::Let { name, value })
}

fn parse_with_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    cursor.next(); // 'with'

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

fn parse_expression(
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

        let right = parse_expression(cursor, binary_operator_precedence + 1)?;
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

fn parse_prefix_expression(cursor: &mut TokenStreamCursor) -> Result<Expression, ParseError> {
    match cursor.next() {
        Token::Integer { value, span } => Ok(Expression::Integer { value, span }),
        Token::Float { value, span } => Ok(Expression::Float { value, span }),
        Token::Identifier(identifier) => Ok(Expression::Identifier(identifier)),
        token if token.is_keyword(Keyword::LoadSample) => {
            dbg!("Detected load_sample");
            let sample = parse_expression(cursor, 0)?;

            Ok(Expression::LoadSample {
                span: Span::new(token.span().start(), sample.span().end()),
                sample: Box::new(sample),
            })
        }
        token if token.is_punctuator(Punctuator::LeftParen) => {
            let expression = parse_expression(cursor, 0)?;

            parse_punctuator(cursor, Punctuator::RightParen)?; // ')'

            Ok(expression)
        }
        token if token.is_punctuator(Punctuator::LeftBracket) => {
            let mut expressions = Vec::new();

            skip_end_of_lines(cursor);

            if !cursor.peek().is_punctuator(Punctuator::RightBracket) {
                expressions.push(parse_expression(cursor, 0)?);

                skip_end_of_lines(cursor);

                while cursor.peek().is_punctuator(Punctuator::Comma) {
                    cursor.next();

                    skip_end_of_lines(cursor);

                    if cursor.peek().is_punctuator(Punctuator::RightBracket) {
                        break; // [a, b,] - still counts
                    }

                    expressions.push(parse_expression(cursor, 0)?)
                }
            }

            let end = cursor.next().span().end();

            Ok(Expression::List {
                expressions,
                span: Span::new(token.span().start(), end),
            })
        }
        token @ Token::Operator {
            operator,
            span: operator_span,
        } => {
            let Some(prefix_operator_kind) = operator.into_prefix_operator_kind() else {
                return Err(ParseError::ExpectedExpression { got: token });
            };

            let operand = parse_expression(cursor, usize::MAX)?;

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

fn parse_identifier(cursor: &mut TokenStreamCursor) -> Result<Identifier, ParseError> {
    let got = cursor.next();
    let Token::Identifier(identifier) = got else {
        return Err(ParseError::ExpectedIdentifier { got });
    };

    Ok(identifier)
}

fn parse_punctuator(
    cursor: &mut TokenStreamCursor,
    punctuation: Punctuator,
) -> Result<Token, ParseError> {
    let got = cursor.next();
    if !got.is_punctuator(punctuation) {
        return Err(ParseError::ExpectedPunctuation {
            expected: punctuation,
            got,
        });
    }

    Ok(got)
}

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

fn skip_end_of_lines(cursor: &mut TokenStreamCursor) {
    while cursor.peek().is_eol() {
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
        (
            binary_expr,
            "a +
2 * (3 + b) - 3"
        ),
        (play_and_wait, "play c4 wait 1"),
        (
            list,
            "[1, 2] [1,
2]
[
1, 
2]
[1,
2,]"
        ),
        (let_stmt, "let a = 3 + 2")
    );
}
