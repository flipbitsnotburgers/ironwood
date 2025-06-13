//! Ironwood - Efficient S-expression evaluation engine

pub mod intern;
pub mod value;

pub use intern::{StringInterner, StringId};
pub use value::{Value, ValueType};