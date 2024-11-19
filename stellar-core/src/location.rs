#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ByteLocation {
    pub line: u32,
    pub column: u32,
    pub index: u32,
}

impl ByteLocation {
    pub fn new(line: u32, column: u32, index: u32) -> Self {
        Self {
            line,
            column,
            index,
        }
    }

    pub fn start_of_file() -> Self {
        Self {
            line: 1,
            column: 0,
            index: 0,
        }
    }

    pub fn next_byte_location(self) -> Self {
        Self {
            line: self.line,
            column: self.column + 1,
            index: self.index + 1,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    pub start: ByteLocation,
    pub end: ByteLocation
}

impl Span {
    pub fn new(start: ByteLocation, end: ByteLocation) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> u32 {
        self.start.index - self.end.index
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}
