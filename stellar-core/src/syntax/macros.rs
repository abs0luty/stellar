// All heavy looking macros moved here.

#[cfg(test)]
#[macro_export]
macro_rules! test_parse {
    ($(($name:ident, $source:expr)),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                let token_stream = scan($source).expect("Scanning failed");
                assert_debug_snapshot!(parse(token_stream));
            }
        )*
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! test_scan {
    ($(($name:ident, $source:expr)),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                assert_debug_snapshot!(scan($source));
            }
        )*
    };
}

#[macro_export]
macro_rules! match_single_and_two_character_tokens {
    ($char:expr, $cursor:expr, $start:expr,
        { $($single_punctuator:pat => $single_punctuator_token:expr,)+ },
        { $($single_operator:pat => $single_operator_token:expr,)+ },
        { $($first:pat,$second:pat => $pair_token:expr,)+ }) => {{
        match ($char, $cursor.peek()) {
            // Handle two-character operators.
            $(
                ($first, Some($second)) => {
                    $cursor.next();

                    Ok(Token::Operator {
                        operator: $pair_token,
                        span: Span::new($start, $cursor.location()),
                    })
                },
            )+
            // Handle single-character operators.
            $(
                ($single_operator, _) => Ok(Token::Operator {
                    operator: $single_operator_token,
                    span: Span::new($start, $cursor.location()),
                }),
            )+
            // Handle punctuators.
            $(
                ($single_punctuator, _) => Ok(Token::Punctuator {
                    punctuator: $single_punctuator_token,
                    span: Span::new($start, $cursor.location()),
                }),
            )+
            // Handle unexpected character.
            _ => Err(ScanError::UnexpectedCharacter {
                character: $char,
                span: Span::new($start, $cursor.location()),
            }),
        }
    }};
}
