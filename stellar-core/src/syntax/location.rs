/// Represents location of an individual byte in a source file.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location {
    line: u32,
    column: u32,
    index: u32,
}

impl Location {
    /// Creates a new location object.
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

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn column(&self) -> u32 {
        self.column
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

/// Represents range of bytes in a source file.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    start: Location,
    end: Location,
}

impl Span {
    /// Creates a byte span.
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    /// Returns length of the span in bytes.
    pub fn len(&self) -> u32 {
        self.start.index - self.end.index
    }

    /// Returns location of the first byte in the span.
    pub fn start(&self) -> Location {
        self.start
    }

    /// Returns location of the last byte in the span.
    pub fn end(&self) -> Location {
        self.end
    }
}

/// Represents any object localized in a specific byte span 
/// of a source file.
pub trait Spanned {
    /// Returns a byte span associated with the object.
    fn span(&self) -> Span;
}
