use lasso::Rodeo;

use crate::{
    cursor::Cursor,
    location::Span,
    token::{Keyword, Token},
};

pub fn scan(source: &str, rodeo: &mut Rodeo) -> Result<Vec<Token>, ScannerError> {
    let mut cursor = Cursor::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = scan_next_token(&mut cursor, rodeo)?;

        if let Token::EOF { .. } = token {
            tokens.push(token);

            break;
        }

        tokens.push(token);
    }

    Ok(tokens)
}

#[derive(Debug, PartialEq)]
pub enum ScannerError {
    UnexpectedCharacter(char),
}

fn scan_next_token(cursor: &mut Cursor, rodeo: &mut Rodeo) -> Result<Token, ScannerError> {
    while let Some(c) = cursor.peek() {
        if c.is_whitespace() {
            cursor.next();
            continue;
        }

        match c {
            c if c.is_alphabetic() || c == '_' => {
                return Ok(scan_identifier_or_keyword(cursor, rodeo))
            }
            _ => return Err(ScannerError::UnexpectedCharacter(c)),
        }
    }

    Ok(Token::EOF {
        last_byte_location: cursor.location(),
    })
}

fn scan_identifier_or_keyword(cursor: &mut Cursor, rodeo: &mut Rodeo) -> Token {
    let mut name = String::new();
    let start_location = cursor.location();

    while let Some(c) = cursor.peek() {
        if !c.is_alphanumeric() || c == '_' {
            break;
        }

        name.push(c);
        cursor.next();
    }

    let span = Span::new(start_location, cursor.location());

    match name.as_str() {
        "with_fx" => Token::Keyword {
            keyword: Keyword::WithFx,
            span,
        },
        "with_synth" => Token::Keyword {
            keyword: Keyword::WithSynth,
            span,
        },
        "wait" => Token::Keyword {
            keyword: Keyword::Wait,
            span,
        },
        _ => Token::Identifier {
            name: rodeo.get_or_intern(name),
            span,
        },
    }
}

#[cfg(test)]
mod tests {
    use lasso::Rodeo;

    use crate::{
        location::{ByteLocation, Span},
        scan::scan,
        token::{Keyword, Token},
    };

    #[test]
    fn test_eof() {
        let mut rodeo = Rodeo::new();
        let tokens = scan("", &mut rodeo).unwrap();

        assert_eq!(
            tokens,
            vec![Token::EOF {
                last_byte_location: ByteLocation::start_of_file()
            }]
        )
    }

    #[test]
    fn test_identifier_and_keyword() {
        let mut rodeo = Rodeo::new();
        let tokens = scan("test wait", &mut rodeo).expect("Scanning should not fail");

        assert_eq!(
            tokens,
            vec![
                Token::Identifier {
                    name: rodeo.get_or_intern("test"),
                    span: Span::new(ByteLocation::start_of_file(), ByteLocation::new(1, 4, 4))
                },
                Token::Keyword {
                    keyword: Keyword::Wait,
                    span: Span::new(ByteLocation::new(1, 5, 5), ByteLocation::new(1, 9, 9))
                },
                Token::EOF {
                    last_byte_location: ByteLocation::new(1, 9, 9)
                }
            ]
        )
    }
}
