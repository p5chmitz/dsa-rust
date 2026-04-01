/*! A naive bump arena

# About
A simple bump arena (allocator) that allocates objects sequentially from a single contiguous memory region. Allocation is performed by pointer arithmetic and does not support individual deallocation; instead, all objects are dropped and the backing memory is released when the arena itself is dropped.

Objects allocated from the arena have stable addresses for the lifetime of the arena, making this structure well suited for data structures that rely on pointer stability, such as positional lists or graph nodes. The contiguous layout provides array-like cache locality, while avoiding the overhead of per-allocation heap management or reference counting.

This arena was written to support a simple positional list abstraction, but the same allocation strategy applies naturally to graph representations. In practice, higher-level abstractions such as maps or indexed containers are often layered on top of arenas to provide more flexible access patterns.

# Design
In contrast with a simple [pointer bag]() (typcally as a `Vec` of pointers), this design represents a real minimal bump allocator.

# Example

*/

use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{self, NonNull};

pub struct BumpArena<T> {
    ptr: NonNull<T>,
    capacity: usize,
    len: usize,
}

impl<T> BumpArena<T> {
    /// Self contains a pointer to the first slot,
    /// a capacity mirroring the input, and a length
    /// as the number of slots used in the arena
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);

        let layout = Layout::array::<T>(capacity).unwrap();
        let raw = unsafe { alloc(layout) as *mut T };

        let ptr = NonNull::new(raw).expect("allocation failed");

        Self {
            ptr,
            capacity,
            len: 0,
        }
    }

    /// This
    fn alloc(&mut self, value: T) -> *mut T {
        assert!(self.len < self.capacity, "arena exhausted");

        let slot = unsafe { self.ptr.as_ptr().add(self.len) };
        unsafe { ptr::write(slot, value) };

        self.len += 1;
        slot
    }
}

impl<T> Drop for BumpArena<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                ptr::drop_in_place(self.ptr.as_ptr().add(i));
            }

            let layout = Layout::array::<T>(self.capacity).unwrap();
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

#[allow(unused)]
struct Test {
    str: String,
    size: usize,
}
#[test]
fn arena_test() {
    let _ = BumpArena::<usize>::new(10);

    let test = Test {
        str: "Hello, world!".to_string(),
        size: 42,
    };

    // LHS annotation
    let mut v: Vec<&Test> = Vec::with_capacity(64);
    v.push(&test);

    // Turbofish after function
    //let mut v = Vec::with_capacity::<&Test>(64);
    //v.push(&test);

    // Turbofish after type
    let mut v = Vec::<&Test>::with_capacity(64);
    v.push(&test);

    // No generic specifier
    let mut v = Vec::with_capacity(64);
    v.push(&test);

    // Works with new too
    let mut v = Vec::new();
    v.push(&test);
}
