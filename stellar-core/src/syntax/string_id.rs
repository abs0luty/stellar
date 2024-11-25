use std::{fmt::Debug, sync::{Arc, LazyLock}};

use lasso::{Spur, ThreadedRodeo};

static STRING_INTERNER: LazyLock<Arc<ThreadedRodeo>> =
    LazyLock::new(|| Arc::new(ThreadedRodeo::new()));

/// Represents a globally interned string ID.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringId(Spur);

impl StringId {
    /// Interns a string using the global interner and returns its `StringId`.
    pub fn new(string: impl AsRef<str>) -> Self {
        StringId(STRING_INTERNER.get_or_intern(string))
    }

    /// Resolves the string associated with this `StringId`.
    pub fn resolve(self) -> String {
        STRING_INTERNER.resolve(&self.0).to_string()
    }
}

impl Debug for StringId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StringId(\"{}\")", self.resolve())
    }
}

#[cfg(test)]
mod tests {
    use super::StringId;

    #[test]
    fn intern() {
        let a = StringId::new("test");
        let b = StringId::new("test");
        let c = StringId::new("test2");

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }

    #[test]
    fn resolve() {
        let a = StringId::new("test");
        let b = StringId::new("test2");

        assert_eq!(a.resolve(), "test");
        assert_eq!(b.resolve(), "test2");
    }
}