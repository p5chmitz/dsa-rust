#![allow(unused_imports)]

#[test]
/// Illustrates the various ways to get pointers, allocate memory,
/// and initialize values.
fn nonnull_test() {
    use std::boxed::Box;
    use std::ptr::NonNull;

    // Creating a NonNull pointer

    // 1) From a binding
    // 1.1) usize -> *mut usize -> Option<NonNull<usize>>
    // Valid for non-owning pointers
    let mut a: usize = 64;
    let b: *mut usize = &mut a;
    let c: Option<NonNull<usize>> = NonNull::new(b);
    assert_eq!(unsafe { *c.unwrap().as_ptr() }, 64usize);
    unsafe { *c.unwrap().as_ptr() = 34usize };
    assert_eq!(unsafe { *c.unwrap().as_ptr() }, 34usize);
    // 1.2) usize -> Option<NonNull<usize>>
    // Avoid outside specialized use cases
    // This creates a pointer from the numerical value of a,
    // it does not create a pointer to the value
    let a: usize = 64;
    #[allow(unknown_lints)]
    let _b: Option<NonNull<usize>> = NonNull::new(a as *mut usize);
    //assert_eq!(unsafe { *b.unwrap().as_ptr() }, 64usize); // Segfault!

    // 2) From Box
    //////////////

    // 2.1) Box<usize> -> *mut usize -> NonNull<usize>
    // NOTE: *mut T and NonNull<T> are non-owning pointer types.
    // They do not enforce aliasing or ownership rules.
    // The programmer must uphold Rust's aliasing guarantees manually.
    // - Box::new allocates and binds
    // - Box::into_raw converts binding to raw pointer
    // - NonNull::new<T> -> Option<NonNull<T>>
    // - NonNull::new_unchecked<T> -> NonNull<T>
    // - Box::from_raw reclaims ownership
    //
    let a: Box<usize> = Box::new(64);
    let b: *mut usize = Box::into_raw(a);
    // SAFETY: Creating NonNull may actually be Null, so
    // NonNull::new returns Option by default.
    // If you can GUARANTEE that T is non-null, use
    // the unsafe new_unchecked function instead of
    // unwrapping the value.
    //let c: NonNull<usize> = NonNull::new(b).unwrap();
    let c: NonNull<usize> = unsafe { NonNull::new_unchecked(b) };
    let d = c;
    let e = d.as_ptr(); // Copy!
    unsafe { *e = 24 };
    assert_eq!(unsafe { *e }, 24);
    assert_eq!(unsafe { *c.as_ref() }, unsafe { *b });
    // Wraps ptr in Box for Drop cleanup to avoid leak
    let _ = unsafe { Box::from_raw(c.as_ptr()) };

    // 2.2) Inlines the raw pointer creation step
    let a: *mut usize = Box::into_raw(Box::new(64));
    let c: NonNull<usize> = unsafe { NonNull::new_unchecked(a) };
    assert_eq!(unsafe { *c.as_ref() }, unsafe { *a });
    // Wraps ptr in Box for Drop cleanup to avoid leak
    //let _ = Box::new(c); // Leaks
    let _ = unsafe { Box::from_raw(c.as_ptr()) };

    // 2.3) Uses new for an Option<NonNull> just in case
    let a = NonNull::new(Box::into_raw(Box::new(64)));
    assert_eq!(unsafe { *a.unwrap().as_ptr() }, 64usize);
    // Wraps ptr in Box for Drop cleanup to avoid leak
    //let _ = Box::new(a); // Leaks
    let _ = unsafe { Box::from_raw(a.unwrap().as_ptr()) };

    // 2.4) Inlines NonNull::new_unchecked for brevity
    // SAFETY: Pointer is guaranteed to be valid given
    // composition chain from Boxed value
    let a = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(64))) };
    assert_eq!(unsafe { *a.as_ptr() }, 64usize);
    // Wraps ptr in Box for Drop cleanup to avoid leak
    //let _ = Box::new(a); // Leaks
    let _ = unsafe { Box::from_raw(a.as_ptr()) };
}

#[test]
fn size_of() {
    use std::mem::size_of;
    use std::ptr::NonNull;

    // Option<NonNull> is optimized whereas Option<*mut T> is not!
    // size_of() returns the byte offset of the argument which
    // provides the effective size of a type
    assert_eq!(size_of::<Option<NonNull<String>>>(), 8_usize);
    assert_eq!(size_of::<*mut String>(), 8_usize);
    assert_eq!(size_of::<Option<*mut String>>(), 16_usize);
    assert_eq!(size_of::<usize>(), 8_usize);

    assert_eq!(
        size_of::<Option<NonNull<String>>>(),
        size_of::<*mut String>()
    );
    assert_ne!(
        size_of::<Option<NonNull<String>>>(),
        size_of::<Option<*mut String>>()
    );
}

// Using NonNull optimizations saves 24 bytes per AVLNode
#[allow(unused)]
mod opt {
    use std::ptr::NonNull;
    // Size: T + 4 * 8 bytes = T + 32 bytes
    struct AVLNode<T> {
        value: T,
        parent: Option<NonNull<AVLNode<T>>>,
        left: Option<NonNull<AVLNode<T>>>,
        right: Option<NonNull<AVLNode<T>>>,
        height: usize,
    }

    #[test]
    fn one() {
        let avl_node = AVLNode {
            value: "Hello".to_string(),
            parent: None,
            left: None,
            right: None,
            height: 1,
        };
        // String is 24 bytes + 32 bytes base = 56 bytes
        assert_eq!(size_of::<AVLNode<String>>(), 56_usize);
    }
}

#[allow(unused)]
mod nonopt {
    // Size: T + 3 * 16 bytes + 8 bytes = T + 56 bytes
    struct AVLNode<T> {
        value: T,
        parent: Option<*mut AVLNode<T>>,
        left: Option<*mut AVLNode<T>>,
        right: Option<*mut AVLNode<T>>,
        height: usize,
    }

    #[test]
    fn one() {
        let avl_node = AVLNode {
            value: "Hello".to_string(),
            parent: None,
            left: None,
            right: None,
            height: 1,
        };
        // String is 24 bytes + 56 bytes base = 80 bytes
        assert_eq!(size_of::<AVLNode<String>>(), 80_usize);
    }
}

#[test]
#[allow(unused)]
fn two() {
    // Recursive, infinitely-sized types are illegal!
    //struct Recursive<T>{
    //    data: Recursive<T>,
    //}

    // Recursive types must include:
    // - Indirection (Box, Arc, Rc, raw pointers, etc.)
    //   on inner type reference to make the size finite
    // - Non-recursive use of generic parameters, if present.
    //   For example, without data: T or PhantomData<T> this this
    //   type contains only recursive use of T. All type parameters
    //   must be used in a non-recursive way to constrain variance.
    struct Recursive<T> {
        next: Box<Recursive<T>>,
        data: T, //marker: std::marker::PhantomData<T>
    }

    // Useless but legal because Rust only requires finite,
    // computable sizing, not necessarily that any recursion terminate
    struct RecursiveSized {
        next: Box<RecursiveSized>,
    }
}

use std::alloc;
use std::mem;
use std::ptr;

#[test]
#[allow(unused)]
fn dangle() {
    // Useful for representing empty collections
    let a: *const u32 = ptr::dangling_mut();
    // This dereference represents UB (and probable segfault) because
    // b is not allocated or initialized
    //let b = unsafe { *a }; // UB!!
}
