use crate::lang::{
    ast::{Block, Expression, Statement},
    location::{Span, Spanned},
    token::{Keyword, Punctuation, Token, TokenStream, TokenStreamCursor},
};

use super::{ast::Property, token::Identifier};

/// Parses a given token stream into Abstract Syntax Tree (AST).
pub fn parse(stream: TokenStream) -> Result<Vec<Statement>, ParseError> {
    let Some(mut cursor) = stream.into_cursor() else {
        return Err(ParseError::InvalidTokenStream);
    };
    let mut statements = Vec::new();

    loop {
        skip_eols(&mut cursor);

        if cursor.peek().is_eof() {
            break;
        }

        statements.push(parse_statement(&mut cursor)?);
    }

    Ok(statements)
}

fn parse_block(cursor: &mut TokenStreamCursor) -> Result<Block, ParseError> {
    let start = parse_punctuation(cursor, Punctuation::LeftBrace)?
        .span()
        .start(); // '{'
    let mut statements = Vec::new();

    loop {
        skip_eols(cursor);

        if cursor.peek().is_punctuation(Punctuation::RightBrace) {
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
        token if token.is_keyword(Keyword::Sequence) => parse_sequence_statement(cursor),
        token if token.is_keyword(Keyword::With) => parse_with_statement(cursor),
        _ => parse_expression(cursor, 0).map(|e| Statement::Expression(e)),
    }
}

fn parse_property(cursor: &mut TokenStreamCursor) -> Result<Property, ParseError> {
    let name = parse_identifier(cursor)?; // name
    parse_punctuation(cursor, Punctuation::Colon)?; // ':'

    let value = parse_expression(cursor, 0)?;

    Ok(Property { name, value })
}

fn parse_with_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    cursor.next(); // 'with'

    let mut properties = Vec::new();

    properties.push(parse_property(cursor)?);

    while cursor.peek().is_punctuation(Punctuation::Comma) {
        cursor.next();

        if cursor.peek().is_punctuation(Punctuation::LeftBrace) {
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

    while let Some(operator) = cursor.peek().into_binary_operator() {
        let operator_precedence = operator.precedence();

        if operator_precedence < precedence {
            break;
        }

        cursor.next();

        // 1 |  3 + 
        // 2 |  2 # Expression is continued on the new line  
        skip_eols(cursor);

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

            parse_punctuation(cursor, Punctuation::RightParen)?; // ')'

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

fn parse_punctuation(
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

fn skip_eols(cursor: &mut TokenStreamCursor) {
    while cursor.peek().is_eol() {
        cursor.next();
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidTokenStream,
    ExpectedExpression { token: Token },
    ExpectedIdentifier { got: Token },
    ExpectedPunctuation { expected: Punctuation, got: Token },
}

#[cfg(test)]
mod tests {
    use lasso::Rodeo;

    use crate::lang::{
        ast::{Block, Expression, Property, Statement},
        location::{Location, Span},
        scan::scan,
        token::Identifier,
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
    fn test_with() {
        let mut rodeo = Rodeo::new();
        let token_stream = scan("with a: 3, b: 4, {}", &mut rodeo).unwrap();
        let statements = parse(token_stream).unwrap();

        assert_eq!(
            statements.first().unwrap(),
            &Statement::With {
                properties: vec![
                    Property {
                        name: Identifier::new(
                            rodeo.get_or_intern("a"),
                            Span::new(Location::new(1, 5, 5), Location::new(1, 6, 6))
                        ),
                        value: Expression::Integer {
                            value: 3,
                            span: Span::new(Location::new(1, 8, 8), Location::new(1, 9, 9))
                        }
                    },
                    Property {
                        name: Identifier::new(
                            rodeo.get_or_intern("b"),
                            Span::new(Location::new(1, 11, 11), Location::new(1, 12, 12))
                        ),
                        value: Expression::Integer {
                            value: 4,
                            span: Span::new(Location::new(1, 14, 14), Location::new(1, 15, 15))
                        }
                    }
                ],
                block: Block {
                    statements: vec![],
                    span: Span::new(Location::new(1, 17, 17), Location::new(1, 19, 19))
                }
            }
        );
    }

    #[test]
    fn test_sequence() {
        let mut rodeo = Rodeo::new();
        let token_stream = scan("sequence name {}", &mut rodeo).unwrap();
        let statements = parse(token_stream).unwrap();

        assert_eq!(
            statements.first().unwrap(),
            &Statement::Sequence {
                name: Identifier::new(
                    rodeo.get_or_intern("name"),
                    Span::new(Location::new(1, 9, 9), Location::new(1, 13, 13))
                ),
                block: Block {
                    statements: Vec::new(),
                    span: Span::new(Location::new(1, 14, 14), Location::new(1, 16, 16))
                }
            }
        );
    }
}
