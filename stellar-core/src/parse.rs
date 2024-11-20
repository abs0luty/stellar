use crate::{
    ast::{Name, Statement},
    token::{Keyword, Token, TokenStream, TokenStreamCursor},
};

pub fn parse(stream: TokenStream) -> Result<Vec<Statement>, ParseError> {
    let mut cursor = stream.cursor();
    let mut statements = Vec::new();

    while !cursor.peek().is_eof() {
        statements.push(parse_statement(&mut cursor)?);
    }

    Ok(statements)
}

fn parse_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    match cursor.peek() {
        Token::Keyword {
            keyword: Keyword::Sequence,
            ..
        } => parse_sequence_statement(cursor),
        token => Err(ParseError::ExpectedStatement { token }),
    }
}

fn parse_sequence_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    cursor.next(); // sequence keyword

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
