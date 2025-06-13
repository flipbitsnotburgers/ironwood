//! Value types for Ironwood S-expression engine

use crate::StringId;


/// Core value types that can be stored and evaluated
#[derive(Debug, Clone)]
pub enum Value {
    /// Interned symbol identifier
    Symbol(StringId),
    /// Text literal value
    String(StringId),
    /// Integer value
    Integer(i64),
    /// Float value
    Float(f64),
    /// List of strings
    StringList(Vec<StringId>),
    /// List of integers
    IntegerList(Vec<i64>),
}

/// Manual PartialEq implementation to handle float comparison
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a.to_bits() == b.to_bits(),
            (Value::StringList(a), Value::StringList(b)) => a == b,
            (Value::IntegerList(a), Value::IntegerList(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Symbol(id) => {
                0u8.hash(state);
                id.hash(state);
            }
            Value::String(id) => {
                1u8.hash(state);
                id.hash(state);
            }
            Value::Integer(i) => {
                2u8.hash(state);
                i.hash(state);
            }
            Value::Float(f) => {
                3u8.hash(state);
                f.to_bits().hash(state);
            }
            Value::StringList(list) => {
                4u8.hash(state);
                list.hash(state);
            }
            Value::IntegerList(list) => {
                5u8.hash(state);
                list.hash(state);
            }
        }
    }
}

/// Value type enumeration for type checking and domain validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueType {
    Symbol,
    String,
    Integer,
    Float,
    StringList,
    IntegerList,
}

impl Value {
    /// Get the type of this value
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Symbol(_) => ValueType::Symbol,
            Value::String(_) => ValueType::String,
            Value::Integer(_) => ValueType::Integer,
            Value::Float(_) => ValueType::Float,
            Value::StringList(_) => ValueType::StringList,
            Value::IntegerList(_) => ValueType::IntegerList,
        }
    }

    /// Check if value is a symbol
    pub fn is_symbol(&self) -> bool {
        matches!(self, Value::Symbol(_))
    }

    /// Check if value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Check if value is an integer
    pub fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    /// Check if value is a float
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// Check if value is a string list
    pub fn is_string_list(&self) -> bool {
        matches!(self, Value::StringList(_))
    }

    /// Check if value is an integer list
    pub fn is_integer_list(&self) -> bool {
        matches!(self, Value::IntegerList(_))
    }

    /// Try to get symbol ID
    pub fn as_symbol(&self) -> Option<StringId> {
        match self {
            Value::Symbol(id) => Some(*id),
            _ => None,
        }
    }

    /// Try to get string ID
    pub fn as_string(&self) -> Option<StringId> {
        match self {
            Value::String(id) => Some(*id),
            _ => None,
        }
    }

    /// Try to get integer value
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to get float value
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Try to get string list
    pub fn as_string_list(&self) -> Option<&Vec<StringId>> {
        match self {
            Value::StringList(list) => Some(list),
            _ => None,
        }
    }

    /// Try to get integer list
    pub fn as_integer_list(&self) -> Option<&Vec<i64>> {
        match self {
            Value::IntegerList(list) => Some(list),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_types() {
        // Symbol
        let sym = Value::Symbol(StringId::new(0));
        assert_eq!(sym.value_type(), ValueType::Symbol);
        assert!(sym.is_symbol());
        assert!(!sym.is_string());
        assert_eq!(sym.as_symbol(), Some(StringId::new(0)));
        assert_eq!(sym.as_string(), None);

        // String
        let s = Value::String(StringId::new(1));
        assert_eq!(s.value_type(), ValueType::String);
        assert!(s.is_string());
        assert!(!s.is_symbol());

        // Integer
        let i = Value::Integer(42);
        assert_eq!(i.value_type(), ValueType::Integer);
        assert!(i.is_integer());
        assert_eq!(i.as_integer(), Some(42));

        // Float
        let f = Value::Float(40.5);
        assert_eq!(f.value_type(), ValueType::Float);
        assert!(f.is_float());
        assert_eq!(f.as_float(), Some(40.5));

        // String list
        let sl = vec![StringId::new(0), StringId::new(1)];
        let string_list = Value::StringList(sl.clone());
        assert_eq!(string_list.value_type(), ValueType::StringList);
        assert!(string_list.is_string_list());
        assert_eq!(string_list.as_string_list(), Some(&sl));

        // Integer list
        let il = vec![1, 2, 3];
        let int_list = Value::IntegerList(il.clone());
        assert_eq!(int_list.value_type(), ValueType::IntegerList);
        assert!(int_list.is_integer_list());
        assert_eq!(int_list.as_integer_list(), Some(&il));
    }
}