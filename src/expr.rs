//! Expression AST for S-expressions
//!
//! This module defines the AST for parsed S-expressions and how they map
//! to the runtime evaluation system.

use crate::{StringId, Value};

/// Represents a parsed S-expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    /// Literal value (string, integer, float)
    Literal(Value),
    
    /// Variable reference
    Variable(StringId),
    
    /// Function call
    Call {
        /// Function name (interned)
        function: StringId,
        /// Function arguments
        args: Vec<Expr>,
    },
    
    /// List literal
    List(Vec<Expr>),
}

impl Expr {
    /// Check if this expression is a literal value
    pub fn is_literal(&self) -> bool {
        matches!(self, Expr::Literal(_))
    }
    
    /// Check if this expression is a variable reference
    pub fn is_variable(&self) -> bool {
        matches!(self, Expr::Variable(_))
    }
    
    /// Check if this expression is a function call
    pub fn is_call(&self) -> bool {
        matches!(self, Expr::Call { .. })
    }
    
    /// Check if this expression is a list
    pub fn is_list(&self) -> bool {
        matches!(self, Expr::List(_))
    }
}

/// Built-in functions supported by the expression engine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinFunction {
    // Boolean operators
    And,
    Or,
    Not,
    
    // Comparison operators
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    
    // List operations
    In,
    NotIn,
    OneOf,
    AllOf,
    NoneOf,
    
    // Geo functions
    GeoWithinRadius,
}

impl BuiltinFunction {
    /// Get the string representation of this function
    pub fn as_str(&self) -> &'static str {
        match self {
            BuiltinFunction::And => "and",
            BuiltinFunction::Or => "or",
            BuiltinFunction::Not => "not",
            BuiltinFunction::Equal => "=",
            BuiltinFunction::NotEqual => "!=",
            BuiltinFunction::LessThan => "<",
            BuiltinFunction::LessThanOrEqual => "<=",
            BuiltinFunction::GreaterThan => ">",
            BuiltinFunction::GreaterThanOrEqual => ">=",
            BuiltinFunction::In => "in",
            BuiltinFunction::NotIn => "not-in",
            BuiltinFunction::OneOf => "one-of",
            BuiltinFunction::AllOf => "all-of",
            BuiltinFunction::NoneOf => "none-of",
            BuiltinFunction::GeoWithinRadius => "geo_within_radius",
        }
    }
    
    /// Try to parse a function from its string representation
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "and" => Some(BuiltinFunction::And),
            "or" => Some(BuiltinFunction::Or),
            "not" => Some(BuiltinFunction::Not),
            "=" => Some(BuiltinFunction::Equal),
            "!=" => Some(BuiltinFunction::NotEqual),
            "<" => Some(BuiltinFunction::LessThan),
            "<=" => Some(BuiltinFunction::LessThanOrEqual),
            ">" => Some(BuiltinFunction::GreaterThan),
            ">=" => Some(BuiltinFunction::GreaterThanOrEqual),
            "in" => Some(BuiltinFunction::In),
            "not-in" => Some(BuiltinFunction::NotIn),
            "one-of" => Some(BuiltinFunction::OneOf),
            "all-of" => Some(BuiltinFunction::AllOf),
            "none-of" => Some(BuiltinFunction::NoneOf),
            "geo_within_radius" => Some(BuiltinFunction::GeoWithinRadius),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builtin_functions() {
        assert_eq!(BuiltinFunction::from_str("and"), Some(BuiltinFunction::And));
        assert_eq!(BuiltinFunction::from_str("="), Some(BuiltinFunction::Equal));
        assert_eq!(BuiltinFunction::from_str("one-of"), Some(BuiltinFunction::OneOf));
        assert_eq!(BuiltinFunction::from_str("unknown"), None);
        
        assert_eq!(BuiltinFunction::And.as_str(), "and");
        assert_eq!(BuiltinFunction::Equal.as_str(), "=");
    }
}