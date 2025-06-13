//! String interning system for efficient storage and comparison
//! See https://en.wikipedia.org/wiki/String_interning

use rustc_hash::FxHashMap;
use std::collections::HashMap;

/// String interning pool that provides efficient storage and lookup of strings
#[derive(Debug, Default)]
pub struct StringInterner {
    /// Map from string to interned ID
    string_to_id: FxHashMap<String, StringId>,
    /// Map from ID to string (for reverse lookup)
    id_to_string: HashMap<StringId, String>,
    /// Next available ID
    next_id: u32,
}

/// Interned string identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StringId(u32);

impl StringInterner {
    /// Create a new string interner
    pub fn new() -> Self {
        Self::default()
    }

    /// Intern a string and return its ID
    pub fn intern(&mut self, s: &str) -> StringId {
        if let Some(&id) = self.string_to_id.get(s) {
            return id;
        }

        let id = StringId(self.next_id);
        self.next_id += 1;
        
        let owned = s.to_string();
        self.string_to_id.insert(owned.clone(), id);
        self.id_to_string.insert(id, owned);
        
        id
    }

    /// Get the string for an interned ID
    pub fn resolve(&self, id: StringId) -> Option<&str> {
        self.id_to_string.get(&id).map(|s| s.as_str())
    }

    /// Get the ID for a string if it exists
    pub fn get_id(&self, s: &str) -> Option<StringId> {
        self.string_to_id.get(s).copied()
    }

    /// Check if a string is interned
    pub fn contains(&self, s: &str) -> bool {
        self.string_to_id.contains_key(s)
    }

    /// Get the number of interned strings
    pub fn len(&self) -> usize {
        self.string_to_id.len()
    }

    /// Check if the interner is empty
    pub fn is_empty(&self) -> bool {
        self.string_to_id.is_empty()
    }
}

impl StringId {
    /// Get the raw ID value
    pub fn raw(self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_interning() {
        let mut interner = StringInterner::new();
        
        let id1 = interner.intern("hello");
        let id2 = interner.intern("world");
        let id3 = interner.intern("hello");
        
        assert_eq!(id1, id3);
        assert_ne!(id1, id2);
        
        assert_eq!(interner.resolve(id1), Some("hello"));
        assert_eq!(interner.resolve(id2), Some("world"));
        assert_eq!(interner.len(), 2);
    }

    #[test]
    fn lookup_operations() {
        let mut interner = StringInterner::new();
        
        let id = interner.intern("test");
        
        assert_eq!(interner.get_id("test"), Some(id));
        assert_eq!(interner.get_id("nonexistent"), None);
        assert!(interner.contains("test"));
        assert!(!interner.contains("nonexistent"));
    }
}