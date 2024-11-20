use crate::{
    ast::{Block, Expression, Name, Statement},
    location::{Span, Spanned},
    token::{Keyword, Punctuation, Token, TokenStream, TokenStreamCursor},
};

pub fn parse(stream: TokenStream) -> Result<Vec<Statement>, ParseError> {
    let mut cursor = stream.cursor();
    let mut statements = Vec::new();

    while !cursor.peek().is_eof() {
        statements.push(parse_statement(&mut cursor)?);
    }

    Ok(statements)
}

fn parse_block(cursor: &mut TokenStreamCursor) -> Result<Block, ParseError> {
    let start = cursor.next().span().start(); // '{'
    let mut statements = Vec::new();

    while !cursor.peek().is_punctuation(Punctuation::RightBrace) {
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
        Token::Keyword {
            keyword: Keyword::Sequence,
            ..
        } => parse_sequence_statement(cursor),
        Token::Keyword {
            keyword: Keyword::WithChannel,
            ..
        } => parse_with_channel_statement(cursor),
        token => Err(ParseError::ExpectedStatement { token }),
    }
}

fn parse_with_channel_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    let start = cursor.next().span().start(); // 'with_channel'

    let channel = parse_expression(cursor)?;
    let block = parse_block(cursor)?;

    Ok(Statement::WithChannel { channel, block })
}

fn parse_expression(cursor: &mut TokenStreamCursor) -> Result<Expression, ParseError> {
    todo!()
}

fn parse_sequence_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    cursor.next(); // 'sequence' keyword

    let Token::Identifier { name, span } = cursor.peek() else {
        return Err(ParseError::ExpectedIdentifier {
            token: cursor.peek(),
        });
    };
    cursor.next();

    Ok(Statement::Sequence {
        name: Name { name, span },
    })
}

#[derive(Debug)]
enum ParseError {
    ExpectedStatement { token: Token },
    ExpectedIdentifier { token: Token },
    ExpectedPunctuation { expected: Punctuation, got: Token },
}

#[cfg(test)]
mod tests {
    use lasso::Rodeo;

    use crate::{
        ast::{Name, Statement},
        location::{Location, Span},
        scan::scan,
    };

    use super::parse;

    #[test]
    fn test_sequence() {
        let mut rodeo = Rodeo::new();
        let token_stream = scan("sequence name", &mut rodeo).unwrap();
        let statements = parse(token_stream).unwrap();

        assert_eq!(
            statements.first().unwrap(),
            &Statement::Sequence {
                name: Name {
                    name: rodeo.get_or_intern("name"),
                    span: Span::new(Location::new(1, 9, 9), Location::new(1, 13, 13))
                }
            }
        );
    }
}
