#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location {
    pub line: u32,
    pub column: u32,
    pub index: u32,
}

impl Location {
    pub fn new(line: u32, column: u32, index: u32) -> Self {
        Self {
            line,
            column,
            index,
        }
    }

    /// Returns the location of the first byte in the file - Start Of File (SOF).
    pub fn sof() -> Self {
        Self {
            line: 1,
            column: 0,
            index: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    start: Location,
    end: Location,
}

impl Span {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> u32 {
        self.start.index - self.end.index
    }

    pub fn start(self) -> Location {
        self.start
    }

    pub fn end(self) -> Location {
        self.end
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}
