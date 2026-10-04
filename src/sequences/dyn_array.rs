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
#[allow(unused)] // Temporary shhhhhh
impl<T> DynArray<T> {
    // Utilities
    ////////////

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
        // Will panic on attempts to allocate beyond isize::MAX bytes
        let new_layout = Layout::array::<T>(cap).unwrap();
        // SAFETY:
        let ptr = unsafe { alloc::alloc(new_layout) };
        Self {
            ptr: NonNull::new(ptr as *mut T).expect("Out of memory"),
            len: 0,
            cap,
        }
    }

    fn layouts() {
        let layout = Layout::new::<usize>();
    }

    /// Warning: Unimplemented
    /// Returns the current number of elements in the list.
    pub fn size(&mut self, val: T) {}

    /// Warning: Unimplemented
    /// Returns the current capacity of the list.
    pub fn cap(&mut self, val: T) {}

    /// Warning: Unimplemented
    /// Returns a Boolean indicating whether the list is empty.
    pub fn is_empty(&mut self, val: T) -> bool {
        false
    }

    /// Internal capacity growth utility.
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

    // Mutators
    ///////////

    /// Appends the list with an given element in O(1) time.
    /// If the list is at capacity, this operation automatically
    /// grows the list in O(n) time.
    pub fn push(&mut self, val: T) {
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

    /// Warning: Unimplemented
    /// Removes the last inserted element from the list.
    pub fn pop(&mut self) {}

    /// Warning: Unimplemented
    /// Removes an element at the ith index in the list.
    pub fn remove(&mut self, index: usize) {}

    /// Warning: Unimplemented
    /// Inserts an element at a given index, forcing any remaining
    /// elements between i - list.len() to shift right. If the list
    /// is at capacity, the list automatically grows in O(n) time.
    pub fn insert(&mut self, val: T, index: usize) {}

    /// Warning: Unimplemented
    /// Returns
    pub fn get(&mut self, index: usize) -> Option<T> {
        None
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
    a.push("Hello");
    a.push("world");

    assert_eq!(a.len, 2);
    assert_eq!(a.cap, 4);

    // Accesses the array with index operator
    assert_eq!(a[0], "Hello");
    assert_eq!(a[1], "world");

    a.push("Alpha");
    a.push("Bravo");
    assert_eq!(a.len, 4);
    assert_eq!(a.cap, 4);

    // Grows the array, doubling its capacity
    a.push("Charlie");
    assert_eq!(a.len, 5);
    assert_eq!(a.cap, 8);

    // Illustrates the same logic with a pre-allocated slab
    // which avoids O(n) reallocation with the internal grow()
    let mut a = DynArray::<String>::new_with_capacity(4);
    a.push("Papa".to_string());
    a.push("Echo".to_string());
    a.push("Tango".to_string());
    a.push("Echo".to_string());
    a.push("Romeo".to_string());
    assert_eq!(a.cap, 8);
    assert_eq!(a.len, 5);
}

#[test]
#[should_panic]
fn test_1() {
    let mut a = DynArray::<&str>::new();
    a.push("Hello");
    a.push("world");

    assert_ne!(a[5], "Hello"); // Illegal index
}

#[test]
fn test_2() {
    let mut a = DynArray::<String>::new_with_capacity(4);
    a.push("Papa".to_string());
    a.push("Echo".to_string());
    a.push("Tango".to_string());
    a.push("Echo".to_string());
    a.push("Romeo".to_string());
    assert_eq!(a.cap, 8);
    assert_eq!(a.len, 5);

    //assert!(balance("([][]{([]{()})})")); // Balanced
    //assert!(balance("([][]{[]{()}}()")); // Missing closing symbol
    //assert!(balance("[][]{[]{()}}())")); // Missing opening symbol
}

#[test]
#[should_panic]
fn test_panic() {
    // Cannot allocate beyond isize::MAX, which is aproximately
    // 9,223,372,036,854,775,807, or ~9.22 quintillion
    let _: DynArray<String> = DynArray::new_with_capacity(9_223_372_036_854_775_809);
    // Panic!!
}

mod testing {

    #![allow(
        unused,
        clippy::disallowed_names,
        clippy::needless_lifetimes,
        clippy::extra_unused_lifetimes
    )]

    struct MutStr {}

    fn two<'a, 'long, 'short, T>() {
        // Invariance vs covariance
        ///////////////////////////
        //
        // foo is invariant, and can only take *mut T
        let foo: *mut T;
        // bar is covariant and can take any sub-type over T
        let bar: std::ptr::NonNull<T>;

        // Sub-typing
        let foo: &'static str = "foo"; // Lives longest, most useful
        let bar: &'long str = "bar"; // Lives longer, more useful
        let baz: &'short str = "baz"; // Lives shortest, least useful

        // Allowed because foo lives exactly as long
        // as the specified lifetime
        three(foo);
        // Allowed because is a sub-type of the
        // parameter and lives much longer so it
        // can be coerced into a shorter lifetime
        four(foo);
        // bar is illegal because bar is a supertype
        // and does not live as long as the parameter spec
        //three(bar);

        five(foo, foo);
        five(bar, baz);
    }

    // Example of contravariance
    ////////////////////////////
    //
    // Least useful because a caller must supply a value
    // that lives for the entire program, instead of some
    // shorter-lived binding.
    fn three(foo: &'static str) {} // Stricter, so less useful
                                   // More useful because a caller can supply a slice
                                   // with any lifetime.
    fn four<'a>(foo: &'a str) {} // Less strict, more useful

    fn five<'a, 'b>(foo: &'a str, bar: &'b str) {
        let a: Box<&str> = Box::from(foo);
        println!("{foo} {bar}");
    }

    // Invariance
    //
    // Raw pointer type *mut T is invariant in T which
    // means that there is no sub-typing possible through
    // type constructor parameter, preventing any type coercion.
    fn invariant<'a>(foo: *mut &'a str) {}
    fn demo_invariance<'a, 'b>(bar: *mut &'a str) {
        // Illegal because *mut T is invariant in its pointee type
        // so it is not possible to coerce the lifetime
        //let mut baz: *mut &'b str = bar; // Error!

        // Allowed because the types are invariant,
        // e.g. foo and bar are both *mut &'a T
        invariant(bar);
    }

    // Covariance
    //
    // Shared references (&T) are covariant over their
    // lifetime parameter, which allows coercion from
    // longer-lived references to shorter-lived ones
    //
    // &'static str <: &'a str, so a &'static str can be used
    // where &'a str is expected
    fn covariant<'a>(foo: &'a str) {}
    fn demo_covariance(bar: &'static str) {
        // Allowed via covariant coerction because
        // &'static str is a sub-type of &'a str
        covariant(bar);
    }

    // contravariance
    //
    fn accepts_static(_: fn(&'static str)) {}
    fn takes_any<'a>(_: fn(&'a str)) {}
    fn demo_contravariance<'a>(foo: fn(&'a str)) {
        accepts_static(foo); // OK via contravariance
    }
}
