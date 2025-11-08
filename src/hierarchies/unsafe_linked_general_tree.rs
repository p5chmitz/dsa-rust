/*! An unsafe, linked, n-ary tree implementation

# About
Following classical DSA curricula, this implementation relies primarily on pointers for the structure's composition and navigation.

There are better ways to do this. Specifically, an index-backed graph crate would likely provide a more robust set of tooling to construct and navigate hierarchical, n-ary tree structures.

Warning: This structure is technically unsound and may cause dangling pointers

# Design
The base [GenTree] structure is sparse and only contains basic operations for constructors and metadata retrieval. Most of the magic happens in the [CursorMut] struct. Both structs rely on a [Position] struct which provides a safe handle to all the raw pointers required to make tree go brrr.

# Example
This section presents an algorithm that builds a tree from a `Vec` of custom `Heading` objects that contain a level and a heading value. Assume the inputs to the algorithm start at level 1 with the first (and lowest) level in the `Vec<Heading>` list being 2. The result is a single, empty root node represented by `[]`.
```text
    []
    │
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
        └── Fresh Water`
```
```rust
    use dsa_rust::hierarchies::unsafe_linked_general_tree::GenTree;

    struct Heading {
        level: usize,
        title: String,
    }
    impl Heading {
        fn new(title: String, level: usize) -> Heading {
            Heading { level, title }
        }
    }

    pub fn construct(mut cur_level: usize, data: Vec<Heading>) -> GenTree<Heading> {
        // Instantiates a Tree with a generic root and traversal positioning
        let mut tree: GenTree<Heading> = GenTree::<Heading>::new();
        let mut cursor = tree.cursor_mut(); // Sets cursor to tree.root

        // Constructs tree from Vec<T>
        for heading in data {
            let data_level = heading.level;

            // Case 1: Adds a child to the current parent and sets level cursor
            if data_level == cur_level + 1 {
                cursor.add_child(heading);
                cur_level += 1;
            }
            // Case 2: Adds a child with multi-generational skips
            else if data_level > cur_level {
                let diff = data_level - cur_level;
                for _ in 1..diff {
                    let empty = Heading::new("[]".to_string(), 0);
                    cursor.add_child(empty);
                    cur_level += 1;
                }
                cursor.add_child(heading);
                cur_level += 1;
            }
            // Case 3: Adds sibling to current parent
            else if data_level == cur_level {
                cursor.ascend().ok();
                cursor.add_child(heading);
            }
            // Case 4: Adds a child to the appropriate ancestor,
            // ensuring proper generational skips
            else {
                let diff = cur_level - data_level;
                for _ in 0..=diff {
                    cursor.ascend().ok();
                    cur_level -= 1;
                }
                cursor.add_child(heading);
                cur_level += 1;
            }
        }
        tree
    }

```

*/

//use crate::trees::traits::Tree;
use std::marker::PhantomData;
use std::ptr::NonNull;

/** The Position struct is a concrete, lightweight struct that provides a safe
handle to raw Node position data to avoid exposing/passing raw pointer data in
the public GenTree and CursorMut APIs.

```example
    // Type aliasing exposes raw pointers! 🙅
    type Pos<T> = Option<*mut Node<T>>;

    // Nullable, even if `Some`
    ptr: *mut Node<T>,
    ptr: Option<*mut Node<T>>

    // Wrapping NonNull guarantees a valid pointer, if one exists
    ptr: Option<NonNull<Node<T>>>
```

The `Position<T>` struct only contains methods for simple, internal operations
like constructing and acquiring pointers, it is intended to be an opaque handle. */
#[derive(Debug, PartialEq)]
pub struct Position<T> {
    ptr: Option<NonNull<Node<T>>>,
    _phantom: PhantomData<T>,
}
impl<T> Position<T> {
    /** Internal constructor only:

    Creates a safe handle to `Node<T>` and returns it as a `Position<T>`.
    Marked as unsafe because it takes a *mut which is the caller's
    responsibility to ensure is not null. */
    //pub(crate) unsafe fn new(ptr: *mut Node<T>) -> Self {
    unsafe fn new(ptr: *mut Node<T>) -> Self {
        Position {
            // NonNull::new() automatically wraps the pointer in Option
            ptr: NonNull::new(ptr),
            _phantom: PhantomData,
        }
    }

    /** Utility for internal access:

    Takes a `Position<T>` and safely extracts the raw `Node<T>` pointer. */
    //pub(crate) fn as_ptr(&self) -> Result<*mut Node<T>, String> {
    fn as_ptr(&self) -> Result<*mut Node<T>, String> {
        match self.ptr {
            Some(non_null_ptr) => Ok(non_null_ptr.as_ptr()),
            None => Err("Oh noez: pointer is null".to_string()),
        }
    }

    /** Returns an immutable reference to the data at the Position, if Some. */
    pub fn get_data(&self) -> Option<&T> {
        if let Ok(ptr) = self.as_ptr() {
            unsafe { (*ptr).data.as_ref() }
        } else {
            None
        }
    }

    /** Replaces the Node data for the given Position

    NOTE: Experimental */
    fn _replace(&mut self, data: T) -> Option<T> {
        if let Ok(ptr) = self.as_ptr() {
            unsafe {
                let boxed_node: Box<Node<T>> = Box::from_raw(ptr);
                (*ptr).data = Some(data);
                boxed_node.data
            }
        } else {
            None
        }
    }
}
impl<T> Clone for Position<T> {
    fn clone(&self) -> Self {
        Position {
            ptr: self.ptr,
            _phantom: PhantomData,
        }
    }
}

/** Represents a general tree with a collection of children */
struct Node<T> {
    parent: Option<Position<T>>,
    children: Vec<Position<T>>, // Always exists for a Node, even if empty
    data: Option<T>,
}
impl<T> Node<T> {
    /** Builds a new Node and returns its position */
    fn build(data: Option<T>) -> Box<Node<T>> {
        Box::new(Node {
            parent: None,
            children: Vec::new(),
            data,
        })
    }
    // /** Builds a new Node and returns its position */
    // fn new(data: T) -> Position<T> {
    //     // Gotta make-a da box, then make-a da *mut 🤌
    //     unsafe { Position::new(Box::into_raw(Box::new(Node {
    //         parent: None,
    //         children: Vec::new(),
    //         data: Some(data),
    //     }))) }
    // }

    // /** Gets an immutable reference to the data at a position */
    // fn get<'a>(position: Position<T>) -> Option<&'a T> {
    //     // Safely extract the raw pointer from Position<T>
    //     // and then YOLO the deref
    //     if let Ok(p) = position.as_ptr() {
    //         unsafe { (*p).data.as_ref() }
    //     } else {
    //         None
    //     }
    // }

    // /** Returns a reference to the Node's child Vec */
    // fn children(&self) -> &Vec<Position<T>> {
    //     &self.children
    // }
}

/** The Tree struct represents a positional, linked-based general
tree structure with a root node that contains a single raw pointer
to the root node and the structure's size.
The genericity of the struct means you'll have to explicitly
type associated functions.

Example:
```example
    let path = std::path::Path::new("~/Developer/project/src/doc");
    let parsed = Tree::<Heading>::parse(path);
    let tree = Tree::<Heading>::construct(parsed.1);
    Tree::<Heading>::preorder_proof(&tree.root);
```
*/
//#[derive(Debug)] // No Debug because Position is secret
pub struct GenTree<T> {
    root: Position<T>,
    size: usize,
}
impl<T> Default for GenTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> GenTree<T> {
    /** Instantiates a new Tree with a default root */
    pub fn new() -> GenTree<T> {
        //let root: Pos<T> = Some(Box::into_raw(Node::build(None)));
        let root: Position<T> = unsafe { Position::new(Box::into_raw(Node::build(None))) };
        GenTree { root, size: 0 }
    }

    // /** Creates a new node and returns its position */
    //pub fn new_node(&self, data: T) -> Position<T> {
    //    let boxed = Box::new(Node {
    //        parent: None,
    //        children: Vec::new(),
    //        data: Some (data)
    //    });
    //    //Some(Box::into_raw(boxed))
    //    unsafe { Position::new(Box::into_raw(boxed)) }
    //}

    // /** Returns an immutable reference to a Node's data at the specified Position,
    // if it exists */
    //pub fn get(&self, node: &Position<T>) -> Option<&T> {
    //    // Imperative approach
    //    if let Ok(n) = node.as_ptr() {
    //        unsafe { (*n).data.as_ref() } // Double de-ref for &*mut type
    //    } else {
    //        None
    //    }
    //    // Functional approach
    //    //node.as_ref().and_then(|n| unsafe { (*(*n)).data.as_ref() })
    //}

    //fn dummy(&self) -> Position<T> {
    //    let ptr = self.root.as_ptr().ok().unwrap();
    //    unsafe { Position::new(ptr) }
    //}

    // /** Returns the parent of a given node, if it exists */
    //pub fn get_parent(&self, node: Position<T>) -> Option<Position<T>> {
    //    if let Ok(n) = node.as_ptr() {
    //        unsafe { (*n).parent }
    //    } else {
    //        None
    //    }
    //}
    //pub fn get_parent(&self, node: Position<T>) -> Option<Position<T>> {
    //    // Check if there even is a pointer
    //    let raw = match node.as_ptr() {
    //        Ok(ptr) => ptr,
    //        Err(_) => return None,
    //    };
    //
    //    // Deref the pointer and wrap it in a Position<T>, if Some
    //    // Borrow the parent Position (if any), extract the raw pointer, and wrap it in a new Position<T>
    //    unsafe {
    //        let parent_ptr = (*raw).parent.as_ref()?.as_ptr().ok()?;
    //        Some(Position {
    //            ptr: NonNull::new(parent_ptr),
    //            _phantom: PhantomData,
    //        })
    //    }
    //    // More explicit operation
    //    //unsafe {
    //    //    match &(*raw).parent {
    //    //        Some(position) => {
    //    //            let ptr = match position.as_ptr() {
    //    //                Ok(ptr) => ptr,
    //    //                Err(_) => return None,
    //    //            };
    //    //            Some(Position {
    //    //                ptr: NonNull::new(ptr),
    //    //                _phantom: PhantomData,
    //    //            })
    //    //        }
    //    //        None => None,
    //    //    }
    //    //}
    //}

    // /** Adds a child to a parent's children field represented as Vec<Pos<T>> */
    //pub fn add_child(&mut self, ancestor: Position<T>, data: T) {
    //    // Create the new child node
    //    let node: Box<Node<T>> = Box::new(Node {
    //        parent: Some(ancestor),
    //        children: Vec::new(),
    //        data: Some(data)
    //    });
    //    let node_ptr: *mut Node<T> = Box::into_raw(node);
    //    let pos = unsafe { Position::new(node_ptr) };

    //    // Add the new child to the parent's child list
    //    unsafe {
    //        if let Some(p) = ancestor {
    //            (*p).children.push(Some(node_ptr));

    //            // Links the node's parent Pos<T> to the correct ancestor
    //            //(*node_ptr).parent = ancestor;
    //        }
    //        self.size += 1;
    //    }
    //    //Some(node_ptr)
    //}

    // /** Returns a reference to the collection of children for a given position, if any */
    //pub fn children(&self, node: Pos<T>) -> Option<&Vec<Pos<T>>> {
    //    if let Some(c) = node {
    //        Some(unsafe { (*c).children.as_ref() })
    //    } else {
    //        None
    //    }
    //}

    // /** Returns true if the given position is the tree's root.
    //
    //WARNING: Unsafe */
    //pub unsafe fn is_root(&self, node: Position<T>) -> bool {
    //    // Sloppy
    //    node.as_ptr().ok().unwrap() == self.root.as_ptr().ok().unwrap()
    //}

    /** Returns the Position of the tree's root */
    pub fn root(&self) -> Position<T> {
        let ptr = self.root.as_ptr().ok().unwrap();
        unsafe { Position::new(ptr) }
    }

    /** Creates a `CursorMut<T>` starting at the tree's root */
    pub fn cursor_mut(&mut self) -> CursorMut<'_, T> {
        // Gets the *mut from root
        let ptr = self.root.as_ptr().ok().unwrap();
        // Constructs and returns the Position<T>
        let root = unsafe { Position::new(ptr) };

        CursorMut {
            node: root,
            tree: self,
        }
    }

    pub fn cursor_from(&mut self, position: &mut Position<T>) -> CursorMut<'_, T> {
        // Gets the *mut from root
        let ptr = position.as_ptr().ok().unwrap();
        // Constructs and returns the Position<T>
        let root = unsafe { Position::new(ptr) };

        CursorMut {
            node: root,
            tree: self,
        }
    }

    /** Returns the depth for a given node */
    pub fn depth(&mut self, node: Position<T>) -> usize {
        let mut depth = 1;
        let mut cursor = self.cursor_mut();
        cursor.jump(&node);
        while !cursor.is_root() {
            cursor.ascend().ok();
            depth += 1;
        }
        depth
    }

    // /** Returns the height of a sub-tree at a given position */
    //pub fn height(&self, node: Pos<T>) -> Option<usize> {
    //    let mut h = 0;
    //    if let Some(n) = node {
    //        for e in unsafe { &(*n).children } {
    //            h = std::cmp::max(h, self.height(Some(e.expect("uh oh")))?)
    //        }
    //    }
    //    Some(h + 1)
    //}
}

impl<T> Drop for GenTree<T> {
    /** Recursive Drop implementation with some dirty tricks */
    fn drop(&mut self) {
        // Recursive wrapper/entry point
        unsafe {
            if let Ok(root_ptr) = &self.root.as_ptr() {
                drop_node_recursive(*root_ptr);
            }
        }

        // Recursive function to Drop Nodes
        unsafe fn drop_node_recursive<T>(node_ptr: *mut Node<T>) {
            // Take the children by swapping out the Vec
            let children = std::mem::take(&mut (*node_ptr).children);
            // Original results in double free
            //let children = std::ptr::read(&(*node_ptr).children);

            // Recursively visit the Positions for each Node
            for child_pos in children {
                if let Ok(child_ptr) = child_pos.as_ptr() {
                    drop_node_recursive(child_ptr);
                }
            }

            // Deallocate the current node
            drop(Box::from_raw(node_ptr));
        }
    }
}

/** A cursor over mutable data that operates with the safe `Position<T>` handle over raw pointers. */
pub struct CursorMut<'a, T> {
    node: Position<T>, // ptr: Option<NonNull<Node<T>>>
    tree: &'a mut GenTree<T>,
}

impl<T> CursorMut<'_, T> {
    // METADATA
    ///////////

    /** Returns true if the Node under the curosr is the tree's root. */
    pub fn is_root(&self) -> bool {
        self.node.as_ptr().ok() == self.tree.root().as_ptr().ok()
    }

    /** Returns true if the Node under the curosr has data. */
    pub fn is_some(&self) -> bool {
        if let Some(node) = self.node.ptr {
            let ptr = node.as_ptr();
            unsafe { (*ptr).data.is_some() }
        } else {
            false
        }
    }

    /** Returns true if the Node under the cursor is empty. */
    pub fn is_none(&self) -> bool {
        if let Some(node) = self.node.ptr {
            let ptr = node.as_ptr();
            unsafe { (*ptr).data.is_none() }
        } else {
            false
        }
    }

    /** Returns the size of the Node's children Vec as usize. */
    pub fn num_children(&self) -> usize {
        if let Ok(ptr) = self.node.as_ptr() {
            unsafe { (*ptr).children.len() }
        } else {
            0
        }
    }

    // ACCESSORS AND MUTATORS
    /////////////////////////

    /** Gets an immutable reference to the data under the cursor, if Some. */
    pub fn get_data(&self) -> Option<&T> {
        let ptr = self.node.as_ptr().ok()?;
        unsafe { (*ptr).data.as_ref() }
    }

    // /** Gets an immutable reference to the data for the current Node */
    //pub fn get(&self) -> Option<&T> {
    //    // Imperative approach
    //    if let Ok(n) = self.node.as_ptr() {
    //        unsafe { (*n).data.as_ref() }
    //    } else {
    //        None
    //    }
    //    // Functional approach
    //    //node.as_ref().and_then(|n| unsafe { (*(*n)).data.as_ref() })
    //}

    // /** Gets an immutable reference to the data for a supplied Position */
    //pub fn get_for_pos(&self, pos: &Position<T>) -> Option<&'a T> {
    //    if let Ok(n) = pos.as_ptr() {
    //        unsafe { (*n).data.as_ref() }
    //    } else {
    //        None
    //    }
    //}

    // /** Overwrites the data for the current Node without affecting its position,
    // returns the old data, if Some */
    //pub fn set(&mut self, data: T) -> Option<T> {
    //    if let Ok(n) = self.node.as_ptr() {
    //        unsafe {
    //            let old = (*n).data.take();
    //            (*n).data = Some(data);
    //            return old;
    //        }
    //    } else {
    //        None
    //    }
    //}

    /** Adds a new child Node under the current cursor and advances the cursor to the new child. */
    pub fn add_child(&mut self, data: T) {
        // Get the *mut out of the node's Position<T>
        let ptr = self.node.as_ptr().ok().unwrap();

        // Create the new child node and give it a Position
        let new_boxed_node: Box<Node<T>> = Box::new(Node {
            parent: Some(unsafe { Position::new(ptr) }),
            children: Vec::new(),
            data: Some(data),
        });
        let node_ptr: *mut Node<T> = Box::into_raw(new_boxed_node);
        let new_node = unsafe { Position::new(node_ptr) };

        // Add the new child to the parent's child list
        unsafe { (*ptr).children.push(new_node) };

        // Mutates self to be the Position of the new node
        self.node = unsafe { Position::new(node_ptr) };

        // Increment the size of the tree
        self.tree.size += 1;
    }

    /** Deletes the node at the current cursor position,
    adds all children to the parent (if Some), and returns the deleted Node.
    If the cursor is at the tree's root, this just deletes the Node's data, leaving None.
    Moves the cursor to the parent, if Some */
    pub fn delete(&mut self) -> Option<T> {
        // Gets a raw pointer to self
        let self_ptr = self.node.ptr.unwrap().as_ptr();

        // Transfers the self.chilren to parent.children, if Some
        unsafe {
            if let Some(parent_position) = &(*self_ptr).parent {
                let parent_ptr = parent_position.as_ptr().unwrap();

                // Get mutable references to the parent and self children vecs
                let parent_children = &mut (*parent_ptr).children;
                let self_children = &mut (*self_ptr).children;

                // Remove self from parent's children Vec
                if let Some(index) = parent_children
                    .iter()
                    .position(|c| c.as_ptr() == self.node.as_ptr())
                {
                    parent_children.remove(index);
                }

                // Move each self.child to the parent's children Vec
                parent_children.append(self_children);

                // Update their parent pointers
                for child in parent_children.iter_mut() {
                    (*child.as_ptr().unwrap()).parent = Some(Position::new(parent_ptr));
                }

                // Move the cursor up to the parent
                self.jump(parent_position);

                // Save original pointer to access deleted data
                // Takes ownership so Box can be automatically dropped
                let boxed_node: Box<Node<T>> = Box::from_raw(self_ptr);

                // Return the deleted node's data
                boxed_node.data
            } else {
                None
            }
        }
    }

    // /** Same as `CursorMut::delete()` but for an arbitrary cursor position.
    //
    // Deletes the node at the current cursor position,
    // adds all children to the parent (if Some), and returns the deleted Node.
    // If the cursor is at the tree's root, this just deletes the Node's data, leaving None.
    // Moves the cursor to the parent, if Some */
    //pub fn delete_node(&mut self, node: Position<T>) Option<Node<T>> {}

    // /** Returns an immutable reference to the parent position for the Node at
    // the current cursor position, if Some. */
    //pub fn parent(&self) -> Option<&Position<T>> {
    //    if let Ok(ptr) = self.node.as_ptr() {
    //        unsafe { (*ptr).parent.as_ref() }
    //    } else {
    //        None
    //    }
    //}

    // NAVIGATION
    /////////////

    /** Returns a reference to the current Position. */
    pub fn current(&self) -> &Position<T> {
        &self.node
    }

    /** Jump the cursor to the defined Position */
    pub fn jump(&mut self, new: &Position<T>) {
        let ptr = new.as_ptr().ok().unwrap();
        self.node = unsafe { Position::new(ptr) };
    }

    // /** Navigates down a generation, if Some */
    //pub fn descend(&self) -> Option<Position<T>> {
    //    unsafe { Some(Position::new(Box::into_raw(Node::build(None)))) }
    //    //let ptr = self.node.as_ptr().ok().unwrap();
    //    //unsafe {
    //    //    if let Some(children) = (*ptr).children { // Unsafe deref
    //    //        let parent_ptr = children.as_ptr().ok().unwrap();
    //    //        Some(Position::new(parent_ptr)) // Unsafe constructor
    //    //    } else { None }
    //    //}
    //}
    //pub fn descend_first(&mut self) -> Option<Position<T>> {
    //    // Gets cursor's current Position
    //    let ptr = self.node.as_ptr().ok()?;

    //    // Returns the position of the first child
    //    unsafe {
    //        let node = &*ptr;
    //        node.children.get(0).cloned()
    //    }
    //}

    // /** Moves the cursor up a generation, if Some. Trying to ascend past the root results in an error. */
    //pub fn ascend(&mut self) -> Result<(), String> {

    //    // Finds current position
    //    if let Some(ptr) = self.node.as_ptr().ok() {

    //        // Finds parent's position
    //        let par = unsafe {
    //            (*ptr).parent.take()
    //        };

    //        // Resets self to parent's Position, if Some
    //        if let Some(ptr) = par {
    //            self.node = ptr
    //        };

    //        Ok(())

    //    } else {

    //        Err("Error: Cannot ascend past root".to_string())
    //    }
    //}
    pub fn ascend(&mut self) -> Result<(), String> {
        if let Ok(ptr) = self.node.as_ptr() {
            // SAFETY: ptr is a valid pointer to a Node<T>
            let parent = unsafe { (*ptr).parent.clone() };

            if let Some(parent_ptr) = parent {
                self.node = parent_ptr;
                Ok(())
            } else {
                Err("Error: Cannot ascend past root".to_string())
            }
        } else {
            Err("Error: No current node to ascend from".to_string())
        }
    }
    /** Returns a list of owned descendant (child) Positions for the current cursor node. */
    pub fn children(&self) -> Vec<Position<T>> {
        let Some(ptr) = self.node.as_ptr().ok() else {
            return vec![];
        };

        unsafe {
            let node = &*ptr;
            node.children.clone()
        }
    }
}

#[cfg(test)]
mod tests {

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
        use super::{builder, builder::Heading, GenTree, Position};
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
        let mut tree: GenTree<Heading> = builder::construct(1, tree_vec);
        let cursor = tree.cursor_mut();

        // Tests root() -> Position<T>
        assert_eq!(cursor.node.as_ptr().ok(), tree.root().as_ptr().ok());

        let mut cursor = tree.cursor_mut();
        // Tests that root is empty with is_some() and is_none()
        assert!(!cursor.is_some());
        assert!(cursor.is_none());

        // Tests num_children()
        assert_eq!(cursor.num_children(), 2); // Root has [Landlocked, Islands]

        // Tests children(), jump(), and get_data()
        let kids = cursor.children();
        let mut kids_iter = kids.iter();
        cursor.jump(kids_iter.next().unwrap()); // Moves to first child
        let data = cursor.get_data().unwrap();
        assert_eq!(*data.title, "Landlocked".to_string());

        cursor.jump(kids_iter.next().unwrap()); // Moves to first child
        let curr: Position<Heading> = cursor.current().clone(); // Passes the torch
        let data = cursor.get_data().unwrap();
        assert_eq!(*data.title, "Islands".to_string());

        // Jumps down a generation to [Marine, Fresh Water]
        cursor.jump(&curr);
        let new_kids = cursor.children();
        let mut kids_iter = new_kids.iter();
        cursor.jump(kids_iter.next().unwrap()); // Moves to first child
        let data = cursor.get_data().unwrap();
        assert_eq!(*data.title, "Marine".to_string());

        // Jumps down a generation, for fun
        let new_kids = cursor.children(); // Gets cursor's chidlren
        let mut kids_iter = new_kids.iter(); // Creates an iterator
        cursor.jump(kids_iter.next().unwrap()); // Moves to first child
        let data = cursor.get_data().unwrap();
        assert_eq!(*data.title, "Australia".to_string());

        // Tests ascend()
        assert!(cursor.ascend().is_ok()); // Marine
        assert!(cursor.ascend().is_ok()); // Islands
        let data = cursor.get_data().unwrap();
        assert_eq!(*data.title, "Islands".to_string());
        assert!(cursor.ascend().is_ok()); // []
        assert!(cursor.ascend().is_err()); // Cannot ascend() past root
        assert!(cursor.is_root()); // Double checks, just in case

        // Descends to Islands to test delete()
        let kids = cursor.children(); // Gets cursor's chidlren
        let mut kids_iter = kids.iter(); // Creates an iterator
        cursor.jump(kids_iter.next().unwrap()); // Moves to Landlocked
        cursor.jump(kids_iter.next().unwrap()); // Moves to Islands
        let data = cursor.get_data().unwrap();
        assert_eq!(*data.title, "Islands".to_string());

        // Tests delete()
        // Creates placeholder Heading
        let mut deleted = Heading {
            title: String::new(),
            level: 0,
        };
        // Iterates through the child position's under the cursor
        // looking for a matching Heading; Once found, jumps to that position,
        // and deletes the Heading; The delete() operation automatically jumps
        // the cursor to the parent of the deleted position
        for position in cursor.children() {
            if position.get_data().unwrap().title == "Marine" {
                cursor.jump(&position);
                deleted = cursor.delete().unwrap();
            }
        }
        // Tests that the correct Heading was deleted
        assert_eq!(deleted.level, 3);
        assert_eq!(deleted.title, "Marine".to_string());

        // Tests that the cursor got bumped up to Islands
        let data = cursor.get_data().unwrap();
        assert_eq!(data.title, "Islands".to_string());

        // Tests that the Islands node has the correct children
        let mut kids = Vec::new();
        assert_eq!(cursor.children().len(), 2);
        for child in cursor.children() {
            let title = child.get_data().unwrap().title.clone();
            kids.push(title)
        }
        assert_eq!(kids, ["Fresh Water".to_string(), "Australia".to_string()]);
    }

    // This test illustrates that this structure is unsound
    // and causes a dangling pointer
    // THIS IS WHY WE CANT HAVE NICE THINGS
    #[test]
    fn dangle() {
        use super::{builder, builder::Heading, GenTree, Position};
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

        // Creates a tree, Position, and CursorMut
        let mut outer_tree: GenTree<Heading> = builder::construct(0, one);
        let mut _pos: Position<Heading> = outer_tree.root();
        let mut cursor = outer_tree.cursor_mut();

        {
            let inner_tree: GenTree<Heading> = builder::construct(0, two);
            _pos = inner_tree.root();
            cursor.jump(&_pos);
        }

        // UB: Attempts to access dangling pointer on CursorMut::get_data()
        // and Position::get_data() :(
        // Uncomment to trigger miri test failure
        //let _oopsie = cursor.get_data();
        //let _oopsie = _pos.get_data();
    }
}

pub mod builder {

    use regex::Regex;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crate::hierarchies::unsafe_linked_general_tree::{CursorMut, GenTree};
    use std::path::Path;

    #[derive(Debug, PartialEq)]
    pub struct Heading {
        pub level: usize,
        pub title: String,
    }
    impl Heading {
        /** Just a humble Heading builder */
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
        // Instantiates a Tree with a generic root and traversal positioning
        let mut tree: GenTree<Heading> = GenTree::<Heading>::new();
        let mut cursor = tree.cursor_mut(); // Sets cursor to tree.root

        // Constructs tree from Vec<T>
        for node in data {
            let data_level = node.level;

            // Case 1: Adds a child to the current parent
            if data_level == cur_level + 1 {
                cursor.add_child(node);
                cur_level += 1;
            }
            // Case 2: Adds a child with multi-generational skips
            else if data_level > cur_level {
                let diff = data_level - cur_level;
                for _ in 1..diff {
                    let empty = Heading::new("[]".to_string(), 0);
                    cursor.add_child(empty);
                    cur_level += 1;
                }
                cursor.add_child(node);
                cur_level += 1;
            }
            // Case 3: Adds sibling to current parent
            else if data_level == cur_level {
                cursor.ascend().ok();
                cursor.add_child(node);
            }
            // Case 4: Adds a child to the appropriate ancestor,
            // ensuring proper generational skips
            else {
                let diff = cur_level - data_level;
                for _ in 0..=diff {
                    cursor.ascend().ok();
                    cur_level -= 1;
                }
                cursor.add_child(node);
                cur_level += 1;
            }
        }
        tree
    }

    /** Modified preorder traversal function that walks the tree recursively
    printing each node's title and children with appropriate box drawing components */
    fn preorder(cursor: &mut CursorMut<Heading>, prefix: &str) {
        let children = cursor.children();

        if !children.is_empty() {
            let mut index = children.len();

            for child_pos in children {
                index -= 1;

                // Create a CursorMut from the child Position
                let mut child_cursor = CursorMut {
                    node: child_pos,
                    tree: cursor.tree,
                };

                // Access data at child position
                if let Some(child_data) = child_cursor.get_data() {
                    if index == 0 {
                        println!("\t{}└── {}", prefix, child_data.title);
                        preorder(&mut child_cursor, prefix);
                    } else {
                        println!("\t{}├── {}", prefix, child_data.title);
                        preorder(&mut child_cursor, prefix);
                    }
                }
            }
        }
    }

    /** A wrapper for a recursive preorder(ish) traversal function;
    Contains logic to print [] on empty trees for more appealing presentation */
    fn pretty_print(name: &str, position: &mut CursorMut<Heading>) {
        let children = &position.children();
        if children.is_empty() {
            println!("📄 {name}\n\t[]\n"); // Empty trees
        } else {
            println!("📄 {name}\n\t│");
            preorder(position, "");
            println!();
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
                        let mut tree = construct(level, filtered);
                        pretty_print(&name, &mut tree.cursor_mut());
                    }
                    _ => (),
                }
            }
        }
    }
}
