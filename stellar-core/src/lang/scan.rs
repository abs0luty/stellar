use lasso::Rodeo;

use crate::{lang::{
    cursor::Cursor,
    location::{Span, Spanned},
    token::{Keyword, Operator, Punctuator, Token, TokenStream},
}, match_single_and_two_character_tokens};

use super::token::Identifier;

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
        match c {
            // Skip whitespace (except line breaks).
            c if c.is_whitespace() && c != '\n' => {
                cursor.next();
            }
            // Skip single-line comments starting with '#'.
            '#' => {
                while let Some(c) = cursor.next() {
                    if c == '\n' {
                        break; // Stop skipping at the end of the line.
                    }
                }
            }
            _ => break, // Stop skipping when non-whitespace and non-comment are found.
        }
    }

    let Some(c) = cursor.peek() else {
        return Ok(Token::EOF {
            location: cursor.location(),
        });
    };

    match c {
        '\n' => {
            let location = cursor.location();
            cursor.next();

            Ok(Token::EOL {
                span: Span::new(location, cursor.location()),
            })
        }
        c if c.is_alphabetic() || c == '_' => Ok(scan_name(cursor, rodeo)),
        c if c.is_numeric() || c == '.' => Ok(scan_number_or_dot(cursor)),
        _ => {
            let start = cursor.location();
            cursor.next();

            match_single_and_two_character_tokens!(
                c,
                cursor,
                start,
                {
                    '{' => Punctuator::LeftBrace,
                    '}' => Punctuator::RightBrace,
                    '[' => Punctuator::LeftBracket,
                    ']' => Punctuator::RightBracket,
                    '(' => Punctuator::LeftParen,
                    ')' => Punctuator::RightParen,
                    ':' => Punctuator::Colon,
                    '.' => Punctuator::Dot,
                    ',' => Punctuator::Comma,
                },
                {
                    '-' => Operator::Minus,
                    '+' => Operator::Plus,
                    '*' => Operator::Star,
                    '/' => Operator::Slash,
                    '=' => Operator::Assign,
                },
                {
                    '-', '=' => Operator::MinusEq,
                    '+', '=' => Operator::PlusEq,
                    '=', '=' => Operator::Eq,
                }
            )
        }
    }
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

    fn keyword_from_name(name: &str) -> Option<Keyword> {
        match name {
            "with" => Some(Keyword::With),
            "wait" => Some(Keyword::Wait),
            "sequence" => Some(Keyword::Sequence),
            "play" => Some(Keyword::Play),
            "let" => Some(Keyword::Let),
            _ => None,
        }
    }

    match name.as_str() {
        "true" => Token::Bool { value: true, span },
        "false" => Token::Bool { value: false, span },
        name => {
            if let Some(keyword) = keyword_from_name(name) {
                Token::Keyword { keyword, span }
            } else {
                Token::Identifier(Identifier::new(rodeo.get_or_intern(name), span))
            }
        }
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

    if end.index() - start.index() == 1 && has_dot {
        return Token::Punctuator {
            punctuator: Punctuator::Dot,
            span: Span::new(start, end),
        };
    }

    let lexeme = &cursor.source()[(start.index() as usize)..(end.index() as usize)];

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
    use insta::assert_debug_snapshot;
    use lasso::Rodeo;

    use crate::test_scan;

    use super::scan;

    test_scan!(
        (eof, ""),
        (unexpected_char, "!"),
        (punctuation, "("),
        (number_and_dot, "3 3.2."),
        (name, "wait time"),
    );
}
