use std::{iter::Peekable, str::Chars};

use crate::lang::location::Location;

pub struct Cursor<'a> {
    source: &'a str,
    input: Peekable<Chars<'a>>,
    location: Location,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            input: source.chars().peekable(),
            location: Location::sof(),
        }
    }

    pub fn source(&self) -> &'a str {
        return self.source;
    }

    pub fn location(&self) -> Location {
        return self.location;
    }

    pub fn peek(&mut self) -> Option<char> {
        return self.input.peek().copied();
    }

    pub fn next(&mut self) -> Option<char> {
        if let Some(&next_char) = self.input.peek() {
            self.location = if next_char == '\n' {
                Location::new(
                    self.location.line() + 1,
                    0,
                    self.location.index() + next_char.len_utf8() as u32,
                )
            } else {
                Location::new(
                    self.location.line(),
                    self.location.column() + 1,
                    self.location.index() + next_char.len_utf8() as u32,
                )
            };
        }

        return self.input.next();
    }
}
