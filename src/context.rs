use crate::intern::{StringId, StringInterner};

#[derive(Debug)]
pub struct Context {
    interner: StringInterner,
}

impl Context {
    pub fn new() -> Self {
        Self {
            interner: StringInterner::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> StringId {
        self.interner.intern(s)
    }

    pub fn resolve(&self, id: StringId) -> Option<&str> {
        self.interner.resolve(id)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}