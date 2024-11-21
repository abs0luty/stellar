use crate::lang::{
    ast::{Block, Expression, Name, Statement},
    location::{Span, Spanned},
    token::{Keyword, Punctuation, Token, TokenStream, TokenStreamCursor},
};

/// Parses a given token stream into Abstract Syntax Tree (AST).
pub fn parse(stream: TokenStream) -> Result<Vec<Statement>, ParseError> {
    let Some(mut cursor) = stream.into_cursor() else {
        return Err(ParseError::InvalidTokenStream);
    };
    let mut statements = Vec::new();

    while !cursor.peek().is_eof() {
        statements.push(parse_statement(&mut cursor)?);
    }

    Ok(statements)
}

fn force_next_punctuation(
    cursor: &mut TokenStreamCursor,
    punctuation: Punctuation,
) -> Result<Token, ParseError> {
    let got = cursor.next();
    if !got.is_punctuation(punctuation) {
        return Err(ParseError::ExpectedPunctuation {
            expected: punctuation,
            got,
        });
    }

    Ok(got)
}

fn parse_block(cursor: &mut TokenStreamCursor) -> Result<Block, ParseError> {
    if !cursor.peek().is_punctuation(Punctuation::LeftBrace) {
        return Err(ParseError::ExpectedPunctuation {
            expected: Punctuation::LeftBrace,
            got: cursor.peek(),
        });
    }

    let start = cursor.next().span().start(); // '{'
    let mut statements = Vec::new();

    while !cursor.peek().is_punctuation(Punctuation::RightBrace) {
        if cursor.peek().is_eof() {
            return Err(ParseError::ExpectedPunctuation {
                expected: Punctuation::RightBrace,
                got: cursor.peek(),
            });
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
        token if token.is_keyword(Keyword::Sequence) => parse_sequence_statement(cursor),
        token if token.is_keyword(Keyword::With) => parse_with_statement(cursor),
        _ => parse_expression(cursor, 0).map(|e| Statement::Expression(e)),
    }
}

fn parse_with_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    cursor.next(); // 'with'

    let block = parse_block(cursor)?;

    Ok(Statement::With { block })
}

fn parse_expression(
    cursor: &mut TokenStreamCursor,
    precedence: usize,
) -> Result<Expression, ParseError> {
    let mut left = parse_prefix_expression(cursor)?;

    while let Some(operator) = cursor.peek().into_binary_operator() {
        let operator_precedence = operator.precedence();

        if operator_precedence < precedence {
            break;
        }

        let right = parse_expression(cursor, operator_precedence + 1)?;
        left = Expression::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_prefix_expression(cursor: &mut TokenStreamCursor) -> Result<Expression, ParseError> {
    match cursor.peek() {
        Token::Integer { value, span } => {
            cursor.next();

            Ok(Expression::Integer { value, span })
        }
        Token::Float { value, span } => {
            cursor.next();

            Ok(Expression::Float { value, span })
        }
        token if token.is_punctuation(Punctuation::LeftParen) => {
            cursor.next(); // '('

            let expression = parse_expression(cursor, 0)?;

            force_next_punctuation(cursor, Punctuation::RightParen)?; // ')'

            Ok(expression)
        }
        token => {
            let Some(prefix_operator) = token.into_prefix_operator() else {
                return Err(ParseError::ExpectedExpression { token });
            };

            let operand = parse_expression(cursor, usize::MAX)?;

            Ok(Expression::Prefix {
                operator: prefix_operator,
                operand: Box::new(operand),
            })
        }
    }
}

fn parse_sequence_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    cursor.next(); // 'sequence' keyword

    let Token::Identifier { name, span } = cursor.peek() else {
        return Err(ParseError::ExpectedIdentifier {
            token: cursor.peek(),
        });
    };
    cursor.next();

    let block = parse_block(cursor)?;

    Ok(Statement::Sequence {
        name: Name { name, span },
        block,
    })
}

#[derive(Debug)]
pub enum ParseError {
    InvalidTokenStream,
    ExpectedExpression { token: Token },
    ExpectedIdentifier { token: Token },
    ExpectedPunctuation { expected: Punctuation, got: Token },
}

#[cfg(test)]
mod tests {
    use lasso::Rodeo;

    use crate::lang::{
        ast::{Block, Name, Statement},
        location::{Location, Span},
        scan::scan,
    };

    use super::parse;

    #[test]
    fn test_empty() {
        let mut rodeo = Rodeo::new();
        let token_stream = scan("", &mut rodeo).unwrap();
        let statements = parse(token_stream).unwrap();

        assert!(statements.is_empty());
    }

    #[test]
    fn test_sequence() {
        let mut rodeo = Rodeo::new();
        let token_stream = scan("sequence name {}", &mut rodeo).unwrap();
        let statements = parse(token_stream).unwrap();

        assert_eq!(
            statements.first().unwrap(),
            &Statement::Sequence {
                name: Name {
                    name: rodeo.get_or_intern("name"),
                    span: Span::new(Location::new(1, 9, 9), Location::new(1, 13, 13))
                },
                block: Block {
                    statements: Vec::new(),
                    span: Span::new(Location::new(1, 14, 14), Location::new(1, 16, 16))
                }
            }
        );
    }
}
