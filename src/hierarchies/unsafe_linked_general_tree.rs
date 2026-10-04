#![allow(unused)]
/*! An unsafe, linked, n-ary tree implementation

# About
Following classical DSA curricula, this implementation relies primarily on pointers for the structure's composition and navigation.

See the module's companion [MD tree](`crate::hierarchies::unsafe_linked_general_tree::md_tree`) tool, which takes a Markdown document and prints a hierarchical tree diagram of its heading contents.

# Design
The base [GenTree] structure only contains basic operations for constructors and metadata retrieval. Most of the magic happens in the [CursorMut] struct. Both structs rely on a [Position] struct, which provides a safe handle to all the raw pointers required to make the tree go brrr.

The design makes heavy use of `unsafe` code via raw pointers. The module represents a two-fold exercise: writing recursive traversal functions and understanding Rust's ownership model well enough to avoid falling back on tricks such as reference counting ([std::rc::Rc]/[std::sync::Arc]) or arena-like allocation with safe indexing ([Vec]) to manage the structure's operations safely.

Notably, this structure enforces strict lifetime constraints on cursors and positions to lock them to the base tree allocation, utilizing Rust's variance rules to systematically prevent pointer dangling, cross-tree pointer smuggling, and use-after-free errors.

In addition to leveraging lifetimes for safety bounds, the module also uses [Cell] for interior mutability on [CursorMut]. This allows cursor navigation methods to retain immutable `&self` borrows for a more ergonomic API.
*/

use std::marker::PhantomData;
use std::ptr::NonNull;

/// Represents the actual data structure. Currently this struct only
/// has constructor methods to create a new tree and create new cursor
/// handles which provide the lion's share of tree operations.
///
/// See [module-level documentation](`crate::hierarchies::unsafe_linked_general_tree`)
/// for more details.
pub struct GenTree<T> {
    root: NonNull<Node<T>>, // Private for safety reasons
}
impl<T> Default for GenTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> GenTree<T> {
    /// Instantiates a new Tree with a default root
    pub fn new() -> GenTree<T> {
        // Allocate the root node on the heap and get a raw pointer to it
        // SAFETY: Box::into_raw is guaranteed to return a valid,
        // non-null pointer because Box::new panics rather than
        // returning null on allocation failure.
        let root_node = unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                //parent: std::ptr::null_mut(),
                parent: None,
                children: Vec::new(),
                data: None, // Root starts empty
            })))
        };

        GenTree { root: root_node }
    }

    pub fn is_empty(&self) -> bool {
        // SAFETY: Even empty trees have an initialized root
        unsafe { (*self.root.as_ptr()).children.len() == 0 }
    }

    /// Creates a `CursorMut<T>` starting at the tree's root
    /// NOTE: The lifetime 'a is implicitly tied from `&'a mut self`
    /// to the returned `CursorMut<'a, T>`
    pub fn cursor_mut(&mut self) -> CursorMut<'_, T> {
        // Safety: self.root was allocated in GenTree::new() and is guaranteed not to be null
        //let non_null_root = unsafe { NonNull::new_unchecked(self.root) };
        let non_null_root = self.root;

        CursorMut {
            node: Cell::new(non_null_root),
            //tree: self,
            _marker: PhantomData,
        }
    }

    // SAFETY: Allows critical use-after-free errors!!
    //pub fn cursor_mut_from(&mut self, node: NonNull<Node<T>>) -> CursorMut<'_, T> {
    //    // Safety: self.root was allocated in GenTree::new() and is guaranteed not to be null
    //    //let non_null_root = unsafe { NonNull::new_unchecked(self.root) };
    //    let non_null_root = node;

    //    CursorMut {
    //        node: Cell::new(non_null_root),
    //        tree: self,
    //        _marker: PhantomData,
    //    }
    //}

    /** Exposes a read-only Position at the root node */
    fn root(&self) -> Position<'_, T> {
        // Safety: self.root is guaranteed valid
        //let non_null_root = unsafe { NonNull::new_unchecked(self.root) };
        let non_null_root = self.root;
        Position {
            node: non_null_root,
            _marker: PhantomData,
        }
    }
}
// Required because Rust doesn't automatically drop heap allocations for
// raw pointers (NonNull<Node<T>>)
impl<T> Drop for GenTree<T> {
    fn drop(&mut self) {
        // self.root: NonNull<Node<T>>
        let mut stack = vec![];
        let mut node = unsafe { Box::from_raw(self.root.as_ptr()) };
        stack.append(&mut node.children);

        // self.root: *mut T
        //if self.root.is_null() {
        //    return;
        //}

        //// Use an iterative stack-based teardown to prevent stack
        //// overflows on deep trees
        //let mut stack = vec![self.root];

        while let Some(node_ptr) = stack.pop() {
            unsafe {
                // 1. Snatch the children vector out of the
                // node before destroying it
                //let mut current_node = Box::from_raw(node_ptr);
                let mut current_node = Box::from_raw(node_ptr.as_ptr());

                // 2. Push all child pointers onto our teardown stack
                stack.append(&mut current_node.children);

                // 3. current_node naturally goes out of scope
                // here, freeing its allocation
                // and dropping its inner data (T) safely.
            }
        }
    }
}

/// Has no methods, but `Position` has methods to derive and create `Node`s.
///
/// See [module-level documentation](`crate::hierarchies::unsafe_linked_general_tree`)
/// for more details.
#[derive(Clone)]
pub struct Node<T> {
    parent: Option<NonNull<Node<T>>>,
    children: Vec<NonNull<Node<T>>>,
    data: Option<T>,
}

/// The `Position` struct serves as the module's safe public handle to
/// individual nodes in the tree. This struct requires a shared
/// lifetime 'a tied to the underlying tree.
///
/// See [module-level documentation](`crate::hierarchies::unsafe_linked_general_tree`)
/// for more details.
struct Position<'a, T> {
    node: NonNull<Node<T>>,
    _marker: PhantomData<&'a GenTree<T>>,
}
impl<'a, T> Position<'a, T> {
    pub fn get_data(&self) -> Option<&T> {
        unsafe { self.node.as_ref().data.as_ref() }
    }

    pub fn get_children(&self) -> Vec<Position<'a, T>> {
        unsafe {
            let v = &self.node.as_ref().children;
            v.iter().map(|x| Position::from_ptr(*x)).collect()
        }
    }

    // Internal utility for iterator
    fn get_children_iter(&self) -> Vec<Position<'a, T>> {
        unsafe {
            let v = &self.node.as_ref().children;
            v.iter().map(|x| Position::from_ptr(*x)).collect()
        }
    }

    // Private for safety; aint nobody should have a NonNull ptr!
    fn from_ptr(ptr: NonNull<Node<T>>) -> Position<'a, T> {
        Position {
            node: ptr,
            _marker: PhantomData,
        }
    }

    // SAFETY: Enables pointer smuggling
    // A safe way to expose the underlying pointer for assert_eq!
    //pub fn as_ptr(&self) -> NonNull<Node<T>> {
    //    self.node
    //}
}
// Implement Clone so "let curr = cursor.current().clone();" works
impl<'a, T> Clone for Position<'a, T> {
    fn clone(&self) -> Self {
        Position {
            node: self.node,
            _marker: PhantomData,
        }
    }
}

use std::cell::Cell;

/// CursorMut takes 'a to tie it to the tree's lifetime
/// which prevents dangling pointers by gating scopes
///
/// See [module-level documentation](`crate::hierarchies::unsafe_linked_general_tree`)
/// for more details.
pub struct CursorMut<'a, T> {
    node: Cell<NonNull<Node<T>>>,
    //tree: &'a mut GenTree<T>,
    _marker: std::marker::PhantomData<&'a mut &'a ()>,
}
impl<'a, T> CursorMut<'a, T> {
    // Utilities
    ////////////

    pub fn get_data(&self) -> Option<&T> {
        //unsafe { self.node.as_ref().data.as_ref() }
        unsafe { self.node.get().as_ref().data.as_ref() }
    }

    pub fn is_none(&self) -> bool {
        //unsafe { self.node.as_ref().data.is_none() }
        unsafe { self.node.get().as_ref().data.is_none() }
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn num_children(&self) -> usize {
        //unsafe { self.node.as_ref().children.len() }
        unsafe { self.node.get().as_ref().children.len() }
    }

    fn current(&self) -> Position<'a, T> {
        Position {
            //node: self.node,
            node: self.node.get(),
            _marker: PhantomData,
        }
    }

    //fn children(&self) -> Vec<Position<'a, T>> {
    //    // SAFETY: All nodes should have a valid node.children()
    //    unsafe {
    //        //self.node.as_ref().children.iter()
    //        self.node
    //            .get()
    //            .as_ref()
    //            .children
    //            .iter()
    //            .map(|&ptr| Position {
    //                //node: NonNull::new_unchecked(ptr),
    //                node: ptr,
    //                _marker: PhantomData,
    //            })
    //            .collect()
    //    }
    //}

    // SAFETY: Breaks uniqueness invariants
    //pub fn get_tree(&mut self) -> &mut GenTree<T> {
    //    self.tree
    //}

    // Navigation
    /////////////

    // /// Uses Cell for interior mutability in order to retain signature semantics
    // /// such that navigation methods retain &self borrows.
    //pub fn jump(&self, pos: &Position<'a, T>) {
    //    //self.node = pos.node;
    //    self.node.update(|_| pos.node);
    //}
    // Updates to take mutable reference, for safety :)
    //pub fn jump(&mut self, pos: &Position<'a, T>) {
    //    //self.node = pos.node;
    //    self.node.update(|_| pos.node);
    //}

    pub fn children_iter(&self) -> ChildIter<'a, T> {
        ChildIter {
            parent_ptr: self.node.get(),
            index: 0,
            _marker: PhantomData,
        }
    }

    /// Uses Cell for interior mutability in order to retain signature semantics
    /// such that navigation methods retain &self borrows.
    //pub fn ascend(&self) -> Result<(), &str> {
    pub fn ascend(&mut self) -> Result<(), &str> {
        unsafe {
            //let parent_ptr = self.node.as_ref().parent;
            let parent_ptr = self.node.get().as_ref().parent;
            //if parent_ptr.is_none() {
            //    Err("Cannot ascend past root")
            //} else {
            //    //self.node = NonNull::new_unchecked(parent_ptr);
            //    //self.node = parent_ptr.unwrap();
            //    self.node.update(|_| parent_ptr.unwrap());
            //    Ok(())
            //}
            if let Some(val) = parent_ptr {
                self.node.update(|_| val);
                Ok(())
            } else {
                Err("Cannot ascend past root")
            }
        }
    }

    /// Uses a closure to descend to the correct child node.
    /// Ex:
    /// ```rust
    /// ```
    pub fn descend<F>(&mut self, predicate: F) -> Result<(), &str>
    where
        F: Fn(&T) -> bool,
        T: 'a,
    {
        // 1. Get the iterator of children
        // 2. Find the first child that satisfies the predicate
        // 3. Update 'self' (the cursor) to point to that child

        let target_child = self.children_iter().find(|&child| predicate(child));

        if let Some(node) = target_child {
            // Here you update your internal cursor state.
            // Assuming your cursor stores a pointer or reference:
            //self.node = node;
            Ok(())
        } else {
            Err("No child found matching the criteria")
        }
    }
    // Mutation
    ///////////

    /// Creates and adds a child `Node` to the position of the cursor,
    /// then moves the cursor to the new child `Node`.
    pub fn add_child(&mut self, data: T) {
        unsafe {
            //let current_ptr = self.node.as_ptr();
            let current_ptr = self.node.get();
            //let new_node = Box::into_raw(Box::new(Node {
            let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                //parent: Some(current_ptr),
                parent: Some(current_ptr),
                children: Vec::new(),
                data: Some(data),
            })));
            //self.node.as_mut().children.push(new_node);
            self.node.get().as_mut().children.push(new_node);

            // Sets the cursor to the new child node
            self.node.set(new_node);
        }
    }

    pub fn delete(&mut self) -> Option<T> {
        unsafe {
            //let current_ptr = self.node.as_ptr();
            let current_ptr = self.node.get();
            //let parent_ptr = self.node.as_ref().parent;
            let parent_ptr = self.node.get().as_ref().parent;

            //if parent_ptr.is_null() {
            //if parent_ptr.is_none() {
            //    return None;
            //}
            parent_ptr?;

            //let parent = &mut *parent_ptr;
            let parent = parent_ptr.unwrap().as_mut();

            if let Some(pos) = parent.children.iter().position(|&x| x == current_ptr) {
                parent.children.remove(pos);

                //let mut orphans = std::mem::take(&mut self.node.as_mut().children);
                let mut orphans = std::mem::take(&mut self.node.get().as_mut().children);
                for &orphan in &orphans {
                    //(*orphan).parent = parent_ptr;
                    (*orphan.as_ptr()).parent = parent_ptr;
                }
                parent.children.append(&mut orphans);
            }

            //self.node = NonNull::new_unchecked(parent_ptr);
            //self.node = parent_ptr.unwrap();
            self.node = Cell::new(parent_ptr.unwrap());

            //let boxed_node = Box::from_raw(current_ptr);
            let boxed_node = Box::from_raw(current_ptr.as_ptr());
            boxed_node.data
        }
    }
}
pub struct ChildIter<'a, T> {
    // pub struct Node<T> {
    //    parent: Option<NonNull<Node<T>>>,
    //    children: Vec<NonNull<Node<T>>>,
    //    data: Option<T>,
    //}
    parent_ptr: NonNull<Node<T>>,
    index: usize,
    // Ensures correct lifetime tracking
    _marker: std::marker::PhantomData<&'a T>,
}
impl<'a, T> Iterator for ChildIter<'a, T> {
    // Yields raw mutable pointers to the children so you can use them to move the cursor
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let parent_ref = self.parent_ptr.as_ref();
            let child = parent_ref.children.get(self.index).copied();
            match child {
                Some(val) => {
                    self.index += 1;
                    Some(val.as_ref().data.as_ref().unwrap())
                }
                None => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    /** Creates this tree to test properties
        []
        ├── Landlocked
        │   ├── Switzerland
        │   │   └── Geneva
        │   │       └── Old Town
        │   │           └── Cathédrale Saint-Pierre
        │   └── Bolivia
        │       └── []
        │           └── []
        │               ├── Puerta del Sol
        │               └── Puerta de la Luna
        └── Islands
            ├── Marine
            │   └── Australia
            └── Fresh Water
    */
    fn basic() {
        //use super::{md_tree, md_tree::Heading, GenTree, Position};
        use crate::hierarchies::unsafe_linked_general_tree::{md_tree, md_tree::Heading};
        use crate::hierarchies::unsafe_linked_general_tree::{GenTree, Position};
        let tree_vec = vec![
            Heading {
                level: 2,
                title: "Landlocked".to_string(),
            },
            Heading {
                level: 3,
                title: "Switzerland".to_string(),
            },
            Heading {
                level: 4,
                title: "Geneva".to_string(),
            },
            Heading {
                level: 5,
                title: "Old Town".to_string(),
            },
            Heading {
                level: 6,
                title: "Cathédrale Saint-Pierre".to_string(),
            },
            Heading {
                level: 3,
                title: "Bolivia".to_string(),
            },
            Heading {
                level: 6,
                title: "Puerta del Sol".to_string(),
            },
            Heading {
                level: 6,
                title: "Puerta de la Luna".to_string(),
            },
            Heading {
                level: 2,
                title: "Islands".to_string(),
            },
            Heading {
                level: 3,
                title: "Marine".to_string(),
            },
            Heading {
                level: 4,
                title: "Australia".to_string(),
            },
            Heading {
                level: 3,
                title: "Fresh Water".to_string(),
            },
        ];

        // Constructs tree ignoring the first heading
        let mut tree: GenTree<Heading> = md_tree::construct(1, tree_vec);

        ////////////////////////////
        // TESTS CURSOR FUNCTIONS //
        ////////////////////////////

        //        let mut cursor = tree.cursor_mut();
        //        // Tests that root is empty with is_some() and is_none()
        //        assert!(!cursor.is_some());
        //        assert!(cursor.is_none());
        //        // Tests root() -> Position<T>
        //        assert_eq!(cursor.node.as_ptr().ok(), tree.root().as_ptr().ok());
        //        assert_eq!(cursor.node.as_ptr().ok(), tree.root().as_ptr().ok());
        //
        //        // Tests num_children()
        //        assert_eq!(cursor.num_children(), 2); // Root has [Landlocked, Islands]
        //
        //        // Tests children(), jump(), and get_data()
        //        let kids = cursor.children();
        //        let mut kids_iter = kids.iter();
        //        let root: Option<&Heading> = cursor.children_iter().next();
        //        assert_eq!(*root.unwrap().title, "Landlocked".to_string());
        //
        //        //cursor.jump(kids_iter.next().unwrap()); // Moves to first child
        //        let curr: Position<Heading> = cursor.current().clone(); // Passes the torch
        //        let data = cursor.get_data().unwrap();
        //        assert_eq!(*data.title, "Islands".to_string());
        //
        //        // Jumps down a generation to [Marine, Fresh Water]
        //        cursor.jump(&curr);
        //        let new_kids = cursor.children();
        //        let mut kids_iter = new_kids.iter();
        //        cursor.jump(kids_iter.next().unwrap()); // Moves to first child
        //        let data = cursor.get_data().unwrap();
        //        assert_eq!(*data.title, "Marine".to_string());
        //
        //        // Jumps down a generation, for fun
        //        let new_kids = cursor.children(); // Gets cursor's chidlren
        //        let mut kids_iter = new_kids.iter(); // Creates an iterator
        //        cursor.jump(kids_iter.next().unwrap()); // Moves to first child
        //        let data = cursor.get_data().unwrap();
        //        assert_eq!(*data.title, "Australia".to_string());
        //
        //        // Tests ascend()
        //        assert!(cursor.ascend().is_ok()); // Marine
        //        assert!(cursor.ascend().is_ok()); // Islands
        //        let data = cursor.get_data().unwrap();
        //        assert_eq!(*data.title, "Islands".to_string());
        //        assert!(cursor.ascend().is_ok()); // []
        //        assert!(cursor.ascend().is_err()); // Cannot ascend() past root
        //                                           //assert!(cursor.is_root()); // Double checks, just in case
        //
        //        // Descends to Islands to test delete()
        //        let kids = cursor.children(); // Gets cursor's chidlren
        //        let mut kids_iter = kids.iter(); // Creates an iterator
        //        cursor.jump(kids_iter.next().unwrap()); // Moves to Landlocked
        //        cursor.jump(kids_iter.next().unwrap()); // Moves to Islands
        //        let data = cursor.get_data().unwrap();
        //        assert_eq!(*data.title, "Islands".to_string());
        //
        //        // Tests delete()
        //        // Creates placeholder Heading
        //        let mut deleted = Heading {
        //            title: String::new(),
        //            level: 0,
        //        };
        //        // Iterates through the child position's under the cursor
        //        // looking for a matching Heading; Once found, jumps to that position,
        //        // and deletes the Heading; The delete() operation automatically jumps
        //        // the cursor to the parent of the deleted position
        //        for position in cursor.children() {
        //            if position.get_data().unwrap().title == "Marine" {
        //                //cursor.jump(&position);
        //                deleted = cursor.delete().unwrap();
        //            }
        //        }
        //        // Tests that the correct Heading was deleted
        //        assert_eq!(deleted.level, 3);
        //        assert_eq!(deleted.title, "Marine".to_string());
        //
        //        // Tests that the cursor got bumped up to Islands
        //        let data = cursor.get_data().unwrap();
        //        assert_eq!(data.title, "Islands".to_string());
        //
        //        // Tests that the Islands node has the correct children
        //        let mut kids = Vec::new();
        //        assert_eq!(cursor.children().len(), 2);
        //        for child in cursor.children() {
        //            let title = child.get_data().unwrap().title.clone();
        //            kids.push(title)
        //        }
        //        assert_eq!(kids, ["Fresh Water".to_string(), "Australia".to_string()]);

        // Print debug, uncomment panic to print
        md_tree::pretty_print("TEST", &tree);
        //panic!();
    }

    // Pointer smuggling & (lifetime) covariance holes
    //////////////////////////////////////////////////

    #[test]
    #[allow(unused)]
    fn test_use_after_free() {
        // 1)
        // This test verifies that the `_marker: PhantomData<&'a mut &'a ()>`
        // successfully enforces invariance on CursorMut.
        // If your invariance fix works perfectly, THIS TEST MUST FAIL TO COMPILE.
        use crate::hierarchies::unsafe_linked_general_tree::{md_tree, md_tree::Heading};
        use crate::hierarchies::unsafe_linked_general_tree::{GenTree, Position};
        let one = vec![
            Heading {
                level: 1,
                title: "Landlocked".to_string(),
            },
            Heading {
                level: 2,
                title: "Switzerland".to_string(),
            },
        ];
        let two = vec![
            Heading {
                level: 1,
                title: "Bolivia".to_string(),
            },
            Heading {
                level: 2,
                title: "Zimbabwe".to_string(),
            },
        ];

        let mut outer_tree: GenTree<Heading> = md_tree::construct(0, one);
        let cursor = outer_tree.cursor_mut();
        {
            //let inner_tree: GenTree<String> = GenTree::new();
            let inner_tree: GenTree<Heading> = md_tree::construct(0, two);
            // COMPILER ERROR EXPECTED HERE:
            // inner_tree does not live long enough. Invariance stops the compiler
            // from shrinking `cursor`'s lifetime to match `inner_tree`.
            let inner_pos = inner_tree.root();
            //cursor.jump(&inner_pos); // Illegal with multiple mutable borrows
        }
        // If it compiled, this would be a use-after-free,
        // but its illegal because CursorMut is invariant
        cursor.get_data();

        // CRITICAL ERROR: solved
        let mut tree1: GenTree<&str> = GenTree::new();
        let mut tree2: GenTree<&str> = GenTree::new();
        let pos1: Position<'_, _> = tree1.cursor_mut().current();
        //let ptr = pos1.as_ptr(); // Allows pointer smuggling
        drop(tree1);
        //let pos2 = tree2.cursor_mut_from(ptr); // Illegal pointer smuggling
        let pos2 = tree2.cursor_mut(); // Legal
        let _ = pos2.get_data(); // Avoided use-after-free

        // CRITICAL ERROR: pending
        let mut tree: GenTree<i32> = GenTree::new();
        let mut cursor = tree.cursor_mut();
        cursor.add_child(42);
        // Navigate to child and capture its Position
        //let child_pos = cursor.children()[0].clone();
        //cursor.jump(&child_pos);
        // Assert data is there
        assert_eq!(cursor.get_data(), Some(&42));
        //assert_eq!(child_pos.get_data(), Some(&42));
        // Now drop/delete the current node using &mut self mutation
        let deleted_data = cursor.delete();
        assert_eq!(deleted_data, Some(42));
        // cursor.delete() must internally reset cursor.node to a safe fallback
        // such as the parent node or the tree's root. Otherwise this triggers UB
        // by pointing to the freed node.
        cursor.get_data();
        // SAFETY: The borrow checker cannot protect child_pos from becoming stale
        // because it has the same lifetime as the cursor and the tree itself.
        // This is an inherent risk of keeping long-lived Positions.
        // Consider removing Position or adding some reference counting?
        //let _ = child_pos.get_data(); // Expected Miri error if unhandled
    }

    #[test]
    fn test_ascend_past_deleted_parent() {
        // Illegal out-of-bounds indexing!
        //let i: usize = Vec::new()[0];
        //assert_eq!(i, 0);

        let mut tree: GenTree<&str> = GenTree::new();
        let mut cursor = tree.cursor_mut();

        cursor.add_child("parent");
        //let parent_pos = cursor.children()[0].clone();
        //cursor.jump(&parent_pos);

        cursor.add_child("child");
        //let child_pos = cursor.children()[0].clone();

        // Delete parent node while cursor is aware
        //cursor.jump(&parent_pos);
        cursor.delete();

        // Reposition cursor to the child (if it was preserved/re-linked)
        // Verify that ascend safely handles situations where raw pointers are invalidated
        //cursor.jump(&child_pos);
        if let Err(e) = cursor.ascend() {
            assert_eq!(e, "Cannot ascend past root"); // Or your custom orphan error handler
        }
    }

    // Null pointer dereferences & bounds testing
    /////////////////////////////////////////////

    #[test]
    fn test_ascend_past_root_error_handling() {
        let mut tree: GenTree<f64> = GenTree::new();
        let mut cursor = tree.cursor_mut();

        // Verify root node has no parent and safely returns Err instead of null deref
        let result = cursor.ascend(); // Illegal multiple mutable borrow
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot ascend past root");
    }

    #[test]
    fn test_empty_root_data_handling() {
        let tree: GenTree<i32> = GenTree::new();
        let root_pos = tree.root();

        // Verify the un-initialized data option returns None safely without a null deref
        assert_eq!(root_pos.get_data(), None);
    }

    // Memory aliasing & vector invalidations
    /////////////////////////////////////////

    #[test]
    fn test_children_vector_reallocation_aliasing() {
        let mut tree: GenTree<usize> = GenTree::new();
        let mut cursor = tree.cursor_mut();

        // Collect positions of children
        cursor.add_child(1);
        //let first_child_pos = cursor.children()[0].clone();

        // Mass-push items to force the internal Vec<NonNull<Node<T>>> to reallocate
        // its capacity, moving its backing buffer elsewhere in memory.
        for i in 2..100 {
            cursor.add_child(i);
        }

        // Verify that tracking pointers remain valid or that the tree layout
        // does not break parental pointer linkage due to backing array growth.
        //cursor.jump(&first_child_pos);
        //assert_eq!(cursor.get_data(), Some(&1));
        assert_eq!(cursor.get_data(), Some(&99));
    }

    #[test]
    fn test_mut_exclusivity() {
        // Enforces that structural adjustments cannot happen if read-only structures
        // are actively interacting across restricted blocks.
        let mut tree: GenTree<char> = GenTree::new();

        {
            let mut cursor = tree.cursor_mut();
            cursor.add_child('A');
        } // cursor drops here, relinquishing exclusive access to tree

        let root_pos = tree.root();
        assert_eq!(root_pos.get_data(), None);

        // This line would fail to compile if root_pos held a mutable borrow,
        // confirming that shared read-only states don't collide with subsequent allocations.
        let mut _cursor_two = tree.cursor_mut();
    }
}

pub mod md_tree {
    /*! A handy little tool to create tree diagrams from MD headings

    # About
    This module sits on top of the [GenTree](`crate::hierarchies::unsafe_linked_general_tree`) structure and contains five functions:
    - A top-level [navigator] function that takes a [Path] and a level setting to indicate the level that the output drawing should start at
    - A [parse] function that takes a [Path] and outputs a list of headings
    - A [construct] function that builds the `GenTree`
    - A [pretty_print] function that traverses the tree and prints the contents to terminal

    The overall output should look something like this,
    ```text
    📄 /document.md
        │
        ├── Landlocked
        │    ├── Switzerland
        │    │    └── Geneva
        │    │        └── Old Town
        │    │            └── Cathédrale Saint-Pierre
        │    └── Bolivia
        │        └── []
        │            └── []
        │                ├── Puerta del Sol
        │                └── Puerta de la Luna
        └── Islands
            ├── Fresh Water
            └── Australia
    ```

    # Design
    This is mostly just an excuse to write recursive tree traversal functions. All functions but the parsing function utilize recursion.

    **/

    use regex::Regex;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    //use crate::hierarchies::unsafe_linked_general_tree::{CursorMut, GenTree};
    use crate::hierarchies::unsafe_linked_general_tree::{GenTree, Position};
    use std::path::Path;

    #[derive(Debug, PartialEq)]
    pub struct Heading {
        pub level: usize,
        pub title: String,
    }
    impl Heading {
        /** Just a humble Heading md_tree */
        fn new(title: String, level: usize) -> Heading {
            Heading { level, title }
        }
    }

    /** Takes a path to a Markdown file, parses it for title and headings,
    and returns a tuple containing the document title and a vector of
    headings.

    Note: The document title portion of the tuple is specifically
    designed for the Astro-formatted frontmatter of each MD document. */
    fn parse(root: &Path) -> (String, Vec<Heading>) {
        // Regex for capturing the title from front matter
        let t = Regex::new(r"(?ms)^---.*?^title:\s*(.+?)\s*$.*?^---").unwrap();
        let mut doc_title = String::new();
        // Regex for capturing headings H1-H6 as #-######
        let h = Regex::new(r"^(#{1,6})\s+(.*)").unwrap();
        let mut headings: Vec<Heading> = Vec::new();

        // Read input
        let file_path = root;
        let file = File::open(file_path).unwrap(); // TODO: Fix lazy error handling
        let reader = BufReader::new(file);

        // Read the entire file into a single string
        // Imperative style
        let mut content = String::new();
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(&line);
        }
        // Functional style
        //let content: String = reader
        //    .lines()
        //    .map(|l| l.unwrap())
        //    .collect::<Vec<_>>()
        //    .join("\n");

        // Extract the document title
        if let Some(captures) = t.captures(&content) {
            let title = captures.get(1).unwrap().as_str();
            doc_title.push_str(title);
        }

        // Parse headings line by line
        for line in content.lines() {
            if let Some(captures) = h.captures(line) {
                let level = captures.get(1).unwrap().as_str().len();
                let text = captures.get(2).unwrap().as_str().to_string();
                headings.push(Heading { level, title: text });
            }
        }

        (doc_title, headings)
    }

    /** Constructs a tree of Heading types */
    pub fn construct(mut cur_level: usize, data: Vec<Heading>) -> GenTree<Heading> {
        let mut tree: GenTree<Heading> = GenTree::<Heading>::new();
        let mut cursor = tree.cursor_mut();

        for node in data {
            let data_level = node.level;

            // Case 1: Add child directly (level increases by 1)
            if data_level == cur_level + 1 {
                cursor.add_child(node);

                // Move the cursor to the new child
                //let kids = cursor.children();
                //cursor.jump(kids.last().unwrap());
                cur_level += 1;
            }
            // Case 2: Add child for multi-generational skip downwards
            // (level increses by n)
            else if data_level > cur_level {
                let diff = data_level - cur_level;
                for _ in 1..diff {
                    let empty = Heading::new("[]".to_string(), 0);
                    cursor.add_child(empty);

                    //let kids = cursor.children();
                    //cursor.jump(kids.last().unwrap());
                    cur_level += 1;
                }
                cursor.add_child(node);

                //let kids = cursor.children();
                //cursor.jump(kids.last().unwrap());
                cur_level += 1;
            }
            // Case 3: Add sibling (level does not change)
            else if data_level == cur_level {
                cursor.ascend().ok(); // Back to parent
                cursor.add_child(node);

                // Move into new sibling to prepare for possible nested children
                //let kids = cursor.children();
                //cursor.jump(kids.last().unwrap());
            }
            // Case 4: Add child for multi-generational skip upwards
            // (level decreases by n)
            else {
                let diff = cur_level - data_level;
                // Ascend to the appropriate parent level (+1 for the current node itself)
                for _ in 0..=diff {
                    cursor.ascend().ok();
                    cur_level -= 1;
                }
                cursor.add_child(node);

                //let kids = cursor.children();
                //cursor.jump(kids.last().unwrap());
                cur_level += 1;
            }
        }
        tree
    }

    // Dictates the print spacing for the tree drawing used in pretty_print
    // and its recursive helper function
    const SPACE: &str = "    ";

    /// A wrapper for a recursive preorder(ish) traversal function;
    /// Contains logic to print [] on empty trees for more appealing presentation
    /// Takes a reference to the GenTree and pretty-prints its contents.
    pub fn pretty_print(title: &str, tree: &GenTree<Heading>) {
        // Grab a shared reference to the Position at the root node
        let root_pos = tree.root();

        if tree.is_empty() {
            println!("📄 {title}\n{SPACE}[]\n"); // Empty trees
        } else {
            println!("📄 {title}\n{SPACE}│");
            // Recursive helper function call
            preorder(&root_pos, "");
            println!();
        }
    }
    /// Modified preorder traversal function that walks the tree
    /// recursively printing each node's title and children with
    /// appropriate box drawing components.
    fn preorder(pos: &Position<'_, Heading>, prefix: &str) {
        // Safely collect child positions. Because Position is Clone,
        // this creates an array of independent pointers bound to the
        // same tree lifetime.
        let children = pos.get_children();

        for (index, child_pos) in pos.get_children().iter().enumerate() {
            if let Some(child_data) = child_pos.get_data() {
                let (marker, next_prefix) = if index == children.len() - 1 {
                    ("└── ", format!("{prefix}{SPACE}"))
                } else {
                    ("├── ", format!("{prefix}│{SPACE}"))
                };
                // Print the current node layout
                println!("{SPACE}{}{}{}", prefix, marker, child_data.title);
                // Safely recurse! No mutable re-borrows, no raw pointer
                // smuggling. The compiler tracks the shared tree
                // lifetime across the entire call stack.
                preorder(child_pos, &next_prefix);
            }
        }
    }

    /** A recursive function that chains the module's utility functions to
    pretty-print a table of contents for each Markdown file in the specified
    directory; The is_file() path contains logic to build a tree from filtered
    values, skipping headers above the user-supplied level argument;
    The function also substitues the file name (if any) for all MD files
    not formatted with Astro's frontmatter */
    pub fn navigator(level: usize, path: &Path) {
        if path.is_dir() {
            for component in path.read_dir().expect("read_dir call failed") {
                let entry = component.expect("failure to deconstruct value");
                navigator(level, &entry.path()); // Recursive call
            }
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                match ext.to_str() {
                    Some("md") | Some("mdx") => {
                        println!("{}", path.display());
                        let parsed = parse(path);
                        let mut name: String = parsed.0;
                        if name.is_empty() {
                            if let Some(n) = path
                                .file_name()
                                .expect("Error extracting file name")
                                .to_str()
                            {
                                name = n.to_string()
                            }
                        }
                        let filtered = parsed.1.into_iter().filter(|h| h.level > level).collect();
                        let tree = construct(level, filtered);
                        pretty_print(&name, &tree);
                    }
                    _ => (),
                }
            }
        }
    }
}
