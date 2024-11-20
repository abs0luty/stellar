use lasso::Rodeo;

use crate::{
    cursor::Cursor,
    location::{Span, Spanned},
    token::{Keyword, Punctuation, Token, TokenStream},
};

/// Scans a given Stellar source text and converts into a vector of tokens.
///
/// # Errors
/// In case any obvious syntax errors, which affected the scanning process
/// were found, [`ScannerError`] will be returned.
pub fn scan(source: &str, rodeo: &mut Rodeo) -> Result<TokenStream, ScanError> {
    let mut cursor = Cursor::new(source);
    let mut stream = TokenStream::new();

    loop {
        let token = scan_next_token(&mut cursor, rodeo)?;

        if let Token::EOF { .. } = token {
            stream.push(token);

            break;
        }

        stream.push(token);
    }

    Ok(stream)
}

/// Scans the next token in the source text and advances position of the [`Cursor`].
fn scan_next_token(cursor: &mut Cursor, rodeo: &mut Rodeo) -> Result<Token, ScanError> {
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
                    span: Span::new(start, cursor.location()),
                });
            }};
        }

        match c {
            c if c.is_alphabetic() || c == '_' => return Ok(scan_name(cursor, rodeo)),
            c if c.is_numeric() || c == '.' => return Ok(scan_number_or_dot(cursor)),
            '{' => single_char_punctuation!(Punctuation::LeftBrace),
            '}' => single_char_punctuation!(Punctuation::RightBrace),
            '[' => single_char_punctuation!(Punctuation::LeftBracket),
            ']' => single_char_punctuation!(Punctuation::RightBracket),
            _ => {
                let start = cursor.location();
                cursor.next();

                return Err(ScanError::UnexpectedCharacter {
                    character: c,
                    span: Span::new(start, cursor.location()),
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
        "with_channel" => Token::Keyword {
            keyword: Keyword::WithChannel,
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
        "sequence" => Token::Keyword {
            keyword: Keyword::Sequence,
            span,
        },
        _ => Token::Identifier {
            name: rodeo.get_or_intern(name),
            span,
        },
    }
}

/// Scans a number or a dot (`.`) from the source text.
fn scan_number_or_dot(cursor: &mut Cursor) -> Token {
    let start = cursor.location();
    let mut has_dot = false;

    while let Some(c) = cursor.peek() {
        if c.is_numeric() {
            cursor.next();
        } else if c == '.' && !has_dot {
            has_dot = true;
            cursor.next();
        } else {
            break;
        }
    }

    let end = cursor.location();

    if end.index - start.index == 1 && has_dot {
        return Token::Punctuation {
            punctuation: Punctuation::Dot,
            span: Span::new(start, end),
        };
    }

    let lexeme = &cursor.source()[(start.index as usize)..(end.index as usize)];

    if has_dot {
        Token::Float {
            value: lexeme.parse::<f64>().unwrap(),
            span: Span::new(start, end),
        }
    } else {
        Token::Integer {
            value: lexeme.parse::<i64>().unwrap(),
            span: Span::new(start, end),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ScanError {
    UnexpectedCharacter { character: char, span: Span },
}

impl Spanned for ScanError {
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
        scan::{scan, ScanError},
        token::{Keyword, Punctuation, Token},
    };

    #[test]
    fn test_eof() {
        let mut rodeo = Rodeo::new();
        let mut cursor = scan("", &mut rodeo).unwrap().cursor();

        assert_eq!(
            cursor.next(),
            Token::EOF {
                location: Location::sof()
            }
        )
    }

    #[test]
    fn test_unexpected_character() {
        let mut rodeo = Rodeo::new();

        assert_eq!(
            scan("!", &mut rodeo),
            Err(ScanError::UnexpectedCharacter {
                character: '!',
                span: Span::new(Location::sof(), Location::new(1, 1, 1))
            })
        );
    }

    #[test]
    fn test_punctuation() {
        let mut rodeo = Rodeo::new();
        let mut cursor = scan("{", &mut rodeo).unwrap().cursor();

        assert_eq!(
            cursor.next(),
            Token::Punctuation {
                punctuation: Punctuation::LeftBrace,
                span: Span::new(Location::sof(), Location::new(1, 1, 1))
            }
        );
        assert_eq!(
            cursor.next(),
            Token::EOF {
                location: Location::new(1, 1, 1)
            }
        );
    }

    #[test]
    fn test_number_and_dot() {
        let mut rodeo = Rodeo::new();
        let mut cursor = scan("3 3.2.", &mut rodeo).unwrap().cursor();

        assert_eq!(
            cursor.next(),
            Token::Integer {
                value: 3,
                span: Span::new(Location::sof(), Location::new(1, 1, 1))
            },
        );
        assert_eq!(
            cursor.next(),
            Token::Float {
                value: 3.2,
                span: Span::new(Location::new(1, 2, 2), Location::new(1, 5, 5))
            },
        );
        assert_eq!(
            cursor.next(),
            Token::Punctuation {
                punctuation: Punctuation::Dot,
                span: Span::new(Location::new(1, 5, 5), Location::new(1, 6, 6))
            },
        );
        assert_eq!(
            cursor.next(),
            Token::EOF {
                location: Location::new(1, 6, 6)
            }
        );
    }

    #[test]
    fn test_identifier_and_keyword() {
        let mut rodeo = Rodeo::new();
        let mut cursor = scan("wait time", &mut rodeo).unwrap().cursor();

        assert_eq!(
            cursor.next(),
            Token::Keyword {
                keyword: Keyword::Wait,
                span: Span::new(Location::sof(), Location::new(1, 4, 4))
            },
        );
        assert_eq!(
            cursor.next(),
            Token::Identifier {
                name: rodeo.get_or_intern("time"),
                span: Span::new(Location::new(1, 5, 5), Location::new(1, 9, 9))
            },
        );
        assert_eq!(
            cursor.next(),
            Token::EOF {
                location: Location::new(1, 9, 9)
            }
        );
    }
}
