use std::fmt::Display;

/// ObjId is a handle that represents a location in the managed
/// garbage collector heap. ObjId is used instead of pointers
/// for memory safety.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjId(pub usize);

impl Display for ObjId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// HeapObj represents all lox object types
/// that are heap allocated.
pub struct HeapObj {
    pub is_marked: bool,
    pub kind: ObjKind,
}

impl HeapObj {
    /// size calculates the total allocation size
    /// for each heap object. This is  used
    /// for bookkeeping in the heap.
    pub fn size(&self) -> usize {
        let base_size = std::mem::size_of::<Self>();
        let variant_size = match &self.kind {
            ObjKind::String(s) => s.capacity(),
        };

        base_size + variant_size
    }
}

pub enum ObjKind {
    String(String),
    // Function(ObjFunction),
    // Closure(ObjClosure),
    // Class(ObjClass),
    // Instance(ObjInstance),
    // BoundMethod(ObjBoundMethod),
}
