///////////////////////////
/** A vector-based stack */
///////////////////////////

// Requires a lifetime specifier on the reference type because the reference 
// doesn't own the data it points to, meaning the data could be mutated, moved, 
// or dropped, potentially invalidating the reference;
// The lifetime guarantees that the referenced data lives at least as long as 
// the `Frame` instance, ensuring the reference remains valid for its lifetime
pub struct Frame<'a> {
    pub term: f64,
    pub operator: &'a str, // Lifetime specifier required for references
}
impl<'a> Frame<'a> {}
pub struct Stack<Frame> {
    data: Vec<Frame>,
    length: usize
}
impl<Frame> Stack<Frame> {}
