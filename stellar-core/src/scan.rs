use lasso::Rodeo;

use crate::{
    cursor::Cursor,
    location::{Span, Spanned},
    token::{Keyword, Punctuation, Token},
};

/// Scans a given Stellar source text and converts into a vector of tokens.
///
/// # Errors
/// In case any obvious syntax errors, which affected the scanning process
/// were found, [`ScannerError`] will be returned.
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

/// Scans the next token in the source text and advances position of the [`Cursor`].
fn scan_next_token(cursor: &mut Cursor, rodeo: &mut Rodeo) -> Result<Token, ScannerError> {
    while let Some(c) = cursor.peek() {
        if c.is_whitespace() {
            cursor.next();
            continue;
        }

        macro_rules! single_char_punctuation {
            ($p:expr) => {{
                let start = cursor.location();
                cursor.next();

                return Ok(Token::Punctuation {
                    punctuation: $p,
                    span: Span {
                        start,
                        end: cursor.location(),
                    },
                });
            }};
        }

        match c {
            c if c.is_alphabetic() || c == '_' => return Ok(scan_name(cursor, rodeo)),
            '{' => single_char_punctuation!(Punctuation::LeftBrace),
            '}' => single_char_punctuation!(Punctuation::RightBrace),
            _ => {
                let start = cursor.location();
                cursor.next();

                return Err(ScannerError::UnexpectedCharacter {
                    character: c,
                    span: Span {
                        start,
                        end: cursor.location(),
                    },
                });
            }
        }
    }

    Ok(Token::EOF {
        location: cursor.location(),
    })
}

/// Scans the next candidate for identifier token in the source text and if
/// its name matches any known keywords returns keyword token.
fn scan_name(cursor: &mut Cursor, rodeo: &mut Rodeo) -> Token {
    let mut name = String::new();
    let start = cursor.location();

    while let Some(c) = cursor.peek() {
        if !c.is_alphanumeric() || c == '_' {
            break;
        }

        name.push(c);
        cursor.next();
    }

    let span = Span::new(start, cursor.location());

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

#[derive(Debug, PartialEq)]
pub enum ScannerError {
    UnexpectedCharacter { character: char, span: Span },
}

impl Spanned for ScannerError {
    fn span(&self) -> Span {
        match self {
            Self::UnexpectedCharacter { span, .. } => *span,
        }
    }
}

#[cfg(test)]
mod tests {
    use lasso::Rodeo;

    use crate::{
        location::{Location, Span},
        scan::{scan, ScannerError},
        token::{Keyword, Punctuation, Token},
    };

    #[test]
    fn test_eof() {
        let mut rodeo = Rodeo::new();
        let tokens = scan("", &mut rodeo).unwrap();

        assert_eq!(
            tokens,
            vec![Token::EOF {
                location: Location::sof()
            }]
        )
    }

    #[test]
    fn test_unexpected_character() {
        let mut rodeo = Rodeo::new();
        assert_eq!(
            scan("!", &mut rodeo),
            Err(ScannerError::UnexpectedCharacter {
                character: '!',
                span: Span::new(Location::sof(), Location::new(1, 1, 1))
            })
        );
    }

    #[test]
    fn test_punctuation() {
        let mut rodeo = Rodeo::new();
        let tokens = scan("{", &mut rodeo).expect("Scanning should not fail");

        assert_eq!(
            tokens,
            vec![
                Token::Punctuation {
                    punctuation: Punctuation::LeftBrace,
                    span: Span::new(Location::sof(), Location::new(1, 1, 1))
                },
                Token::EOF {
                    location: Location::new(1, 1, 1)
                }
            ]
        );
    }

    #[test]
    fn test_identifier_and_keyword() {
        let mut rodeo = Rodeo::new();
        let tokens = scan("wait time", &mut rodeo).expect("Scanning should not fail");

        assert_eq!(
            tokens,
            vec![
                Token::Keyword {
                    keyword: Keyword::Wait,
                    span: Span::new(Location::sof(), Location::new(1, 4, 4))
                },
                Token::Identifier {
                    name: rodeo.get_or_intern("time"),
                    span: Span::new(Location::new(1, 5, 5), Location::new(1, 9, 9))
                },
                Token::EOF {
                    location: Location::new(1, 9, 9)
                }
            ]
        )
    }
}
