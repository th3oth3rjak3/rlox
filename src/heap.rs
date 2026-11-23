use crate::objects::{HeapObj, ObjId, ObjKind};

/// Heap represents a garbage collector primitive
/// used to manage objects that have dynamic memory size.
pub struct Heap {
    /// objects is the actual storage location for all
    /// heap managed objects.
    objects: Vec<Option<HeapObj>>,
    /// free is a free list where unused heap indexes live.
    /// When allocating a new object, reuse any existing
    /// free indexes to minimize fragmentation before
    /// pushing to the end of the objects list.
    free: Vec<usize>,
    /// bytes_allocated is used to measure the amount of
    /// storage used by the heap to determine when to next
    /// collect garbage.
    bytes_allocated: usize,
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

impl Heap {
    /// new creates a new Heap.
    pub fn new() -> Self {
        Self {
            objects: Vec::with_capacity(256),
            free: Vec::new(),
            bytes_allocated: 0,
        }
    }

    /// alloc reserves space on the heap for the
    /// new object and returns a new ObjId handle.
    pub fn alloc(&mut self, kind: ObjKind) -> ObjId {
        let obj = HeapObj {
            is_marked: false,
            kind,
        };

        self.bytes_allocated += obj.size();

        if let Some(idx) = self.free.pop() {
            self.objects[idx] = Some(obj);
            return ObjId(idx);
        }

        let idx = self.objects.len();
        let id = ObjId(idx);

        self.objects.push(Some(obj));

        id
    }

    /// is_string determines if the object at the provided
    /// id handle is a string.
    pub fn is_string(&self, id: ObjId) -> bool {
        let obj = self.get_object(id);
        matches!(obj.kind, ObjKind::String(_))
    }

    /// as_string returns a string slice from the
    /// object. This method panics when the object
    /// is not a string.
    pub fn as_string(&self, id: ObjId) -> &str {
        let obj = self.get_object(id);
        match &obj.kind {
            ObjKind::String(s) => s,
        }
    }

    /// get_object retrieves the internal heap object
    /// represented by the id.
    fn get_object(&self, id: ObjId) -> &HeapObj {
        self.objects[id.0]
            .as_ref()
            .expect("heap object handle to be valid")
    }
}
