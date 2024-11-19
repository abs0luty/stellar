use std::{iter::Peekable, str::Chars};

use crate::location::ByteLocation;

pub struct Cursor<'a> {
    input: Peekable<Chars<'a>>,
    location: ByteLocation,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            input: source.chars().peekable(),
            location: ByteLocation::start_of_file(),
        }
    }

    pub fn location(&self) -> ByteLocation {
        return self.location;
    }

    pub fn peek(&mut self) -> Option<char> {
        return self.input.peek().copied();
    }

    pub fn next(&mut self) -> Option<char> {
        if let Some(&next_char) = self.input.peek() {
            if next_char == '\n' {
                self.location.line += 1;
                self.location.column = 0;
            } else {
                self.location.column += 1;
            }

            self.location.index += next_char.len_utf8() as u32;
        }

        return self.input.next();
    }
}
