use std::collections::HashMap;

use crate::objects::{HeapObj, ObjId, ObjKind};

/// Heap represents a garbage collector primitive
/// used to manage objects that have dynamic memory size.
pub struct Heap {
    /// strings is a collection of interned strings that
    /// allow us to do string comparisons using only the
    /// ObjId instead of character by character.
    strings: HashMap<String, ObjId>,
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

const INITIAL_CAPACITY: usize = 256;

impl Heap {
    /// new creates a new Heap.
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
            objects: Vec::with_capacity(INITIAL_CAPACITY),
            free: Vec::new(),
            bytes_allocated: 0,
        }
    }

    /// intern_string de-duplicates strings in the heap
    /// to save space and to make string comparison easier.
    pub fn intern_string(&mut self, s: String) -> ObjId {
        if let Some(&id) = self.strings.get(&s) {
            return id;
        }

        let key = s.clone();
        let id = self.alloc(ObjKind::String(s));
        self.strings.insert(key, id);
        id
    }

    /// alloc reserves space on the heap for the
    /// new object and returns a new ObjId handle. String allocations
    /// should be done through intern_string.
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

    /// free_objects frees every object in the backing
    /// storage.
    pub fn free_objects(&mut self) {
        self.strings = HashMap::new();
        self.objects = Vec::with_capacity(INITIAL_CAPACITY);
        self.free = Vec::new();
        self.bytes_allocated = 0;
    }

    /// free_object frees a single object from the heap
    /// and places its empty slot in the free-list.
    pub fn free_object(&mut self, id: ObjId) {
        let idx = id.0;
        let obj = self.objects[idx].take();
        if let Some(o) = obj {
            let size = o.size();
            self.bytes_allocated -= size;
            self.free.push(idx);
        }
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
