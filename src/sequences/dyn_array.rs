/*!

# About

# Design

# Example
*/

use std::alloc::{self, Layout};
use std::ops::{Index, IndexMut};
use std::ptr::{self, NonNull};

#[derive(Debug)]
/// A dynamically sized, contiguous storage buffer with positive
/// (forward) indexing, where elements are stored sequentially in memory.
pub struct DynArray<T> {
    ptr: NonNull<T>, // Pointer to the start of the contiguous region
    len: usize,      // Number of elements currently initialized
    cap: usize,      // Total number of elements the region can hold
}
#[allow(clippy::new_without_default)]
impl<T> DynArray<T> {
    pub fn new() -> Self {
        // Start with zero capacity and a "dangling" pointer to avoid
        // an immediate syscall for an empty array
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    /// Pre-allocate for quicker operations
    pub fn new_with_capacity(cap: usize) -> Self {
        // Will panic on attempts to allocate beyond isize::MAX
        let new_layout = Layout::array::<T>(cap).unwrap();
        // SAFETY:
        let ptr = unsafe { alloc::alloc(new_layout) };
        Self {
            ptr: NonNull::new(ptr as *mut T).expect("Out of memory"),
            len: 0,
            cap,
        }
    }

    fn grow(&mut self) {
        // Double the capacity (or start at 4)
        let new_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
        let new_layout = Layout::array::<T>(new_cap).unwrap();

        // SAFETY:
        unsafe {
            let new_ptr = if self.cap == 0 {
                // First time allocating: ask the retailer for a fresh block
                alloc::alloc(new_layout)
            } else {
                // Already have memory: ask to resize (might move to a new page)
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                alloc::realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size())
            };

            self.ptr = NonNull::new(new_ptr as *mut T).expect("Out of memory");
            self.cap = new_cap;
        }
    }

    pub fn add(&mut self, val: T) {
        if self.len == self.cap {
            self.grow();
        }

        // SAFETY:
        unsafe {
            // Calculate the offset and write the value into the uninitialized slot
            let offset_ptr = self.ptr.as_ptr().add(self.len);
            ptr::write(offset_ptr, val);
            self.len += 1;
        }
    }
}
impl<T> Index<usize> for DynArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!(
                "Index out of bounds: the len is {} but the index is {}",
                self.len, index
            );
        }
        // Uses pointer arithmetic to retireve the [index]th element
        // add takes usize, for negative indexing like in Python's
        // list type, use offset
        // SAFETY:
        unsafe { &*self.ptr.as_ptr().add(index) }
    }
}
impl<T> IndexMut<usize> for DynArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!(
                "Index out of bounds: the len is {} but the index is {}",
                self.len, index
            );
        }
        // SAFETY:
        unsafe { &mut *self.ptr.as_ptr().add(index) }
    }
}
impl<T> Drop for DynArray<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            // SAFETY:
            unsafe {
                // 1. Drop the actual items in the array
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len));

                // 2. Tell the allocator to release the virtual memory region
                let layout = Layout::array::<T>(self.cap).unwrap();
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[test]
fn test_0() {
    let mut a = DynArray::<&str>::new();
    a.add("Hello");
    a.add("world");

    assert_eq!(a.len, 2);
    assert_eq!(a.cap, 4);

    // Accesses the array with index operator
    assert_eq!(a[0], "Hello");
    assert_eq!(a[1], "world");

    a.add("Alpha");
    a.add("Bravo");
    assert_eq!(a.len, 4);
    assert_eq!(a.cap, 4);

    // Grows the array, doubling its capacity
    a.add("Charlie");
    assert_eq!(a.len, 5);
    assert_eq!(a.cap, 8);

    // Illustrates the same logic with a pre-allocated slab
    // which avoids O(n) reallocation with the internal grow()
    let mut a = DynArray::<String>::new_with_capacity(4);
    a.add("Papa".to_string());
    a.add("Echo".to_string());
    a.add("Tango".to_string());
    a.add("Echo".to_string());
    a.add("Romeo".to_string());
    assert_eq!(a.cap, 8);
    assert_eq!(a.len, 5);
}

#[test]
#[should_panic]
fn test_1() {
    let mut a = DynArray::<&str>::new();
    a.add("Hello");
    a.add("world");

    assert_ne!(a[5], "Hello"); // Illegal index
}

#[test]
fn test_2() {
    let mut a = DynArray::<String>::new_with_capacity(4);
    a.add("Papa".to_string());
    a.add("Echo".to_string());
    a.add("Tango".to_string());
    a.add("Echo".to_string());
    a.add("Romeo".to_string());
    assert_eq!(a.cap, 8);
    assert_eq!(a.len, 5);

    //assert!(balance("([][]{([]{()})})")); // Balanced
    //assert!(balance("([][]{[]{()}}()")); // Missing closing symbol
    //assert!(balance("[][]{[]{()}}())")); // Missing opening symbol
}
