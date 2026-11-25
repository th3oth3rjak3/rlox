use crate::heap::Heap;
use crate::objects::ObjId;

/// Value represents a lox runtime value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Nil,
    Number(f64),
    Bool(bool),
    Obj(ObjId),
}

impl Value {
    /// is_falsey determines if a value has
    /// false-like properties. Nil is falsey, false is falsey,
    /// everything else is considered to be truthy.
    pub fn is_falsey(&self) -> bool {
        match self {
            Self::Nil => true,
            Self::Bool(b) => !b,
            _ => false,
        }
    }

    /// is_bool checks to see if the value
    /// is a boolean variant.
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// as_bool returns the boolean variant
    /// from the value. This function panics
    /// when the variant doesn't match.
    pub fn as_bool(&self) -> bool {
        match self {
            Self::Bool(b) => *b,
            _ => unreachable!("expected boolean"),
        }
    }

    /// is_number checks to see if the value
    /// is a number variant.
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// as_number produces the number
    /// from the value when it's the correct variant.
    /// This function panics when the value is not
    /// a number.
    pub fn as_number(&self) -> f64 {
        match self {
            Self::Number(n) => *n,
            _ => unreachable!("expected number"),
        }
    }

    /// is_obj checks to see if the current
    /// value is an object type.
    pub fn is_obj(&self) -> bool {
        matches!(self, Value::Obj(_))
    }

    /// as_obj produces the object ID handle
    /// when the value is an object. This function
    /// panics when the value is not an object.
    pub fn as_obj(&self) -> ObjId {
        match self {
            Self::Obj(id) => *id,
            _ => unreachable!("expected object"),
        }
    }

    /// is_string checks to see if the current value
    /// is a heap string.
    pub fn is_string(&self, heap: &Heap) -> bool {
        self.is_obj() && heap.is_string(self.as_obj())
    }
}
