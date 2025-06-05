/*! A safe, linked, n-ary tree implementation

# About
Following classical DSA curricula, this implementation relies on pointers for the structure's composition and navigation. This module explores the use of reference counting and interior mutability through the [Rc] and [RefCell] types (respectively) for a safe, positional implementation that avoids dangling pointers and reference cycles for proper [Drop] semantics. 

Reference counting provides a synchronous, deterministic form of memory management that acts like a garbage collector and prevents dangling pointers by automatically managing lifetimes. The structure is able to keep objects alive until their reference count hits zero, potentially even after they've gone out of their original scope. To avoid memory leaks caused by reference cycles, tree nodes use strong `Rc` pointers for children and [Weak] pointers for parent links. This ensures the tree can be correctly dropped recursively from the top down.

Using smart pointers to manage reference counting and interior mutability to skirt multiple mutable references is an elegant solution to the linked desgin, but its still a bit painful, and potentially overkill for many applications. The good news is that there are much easier ways to accomplish similar goals. To wit, this library also includes a `Vec`-backed tree structure with a similar API. For more polished levels of functionality with the same arena-style backing structure concepts see [id_tree](https://docs.rs/id_tree/latest/id_tree/). It is worth noting that `id_tree` uses a hash map to store node IDs, so it may not be as performanat as either a pointer-backed or simple indexed tree structure for smaller, short-lived tree structures.

# Design
The base [GenTree] structure is sparse and only contains basic operations for constructors and metadata retrieval. Most of the magic happens in the [CursorMut] struct. Both structs rely on a [Position] struct which provides a safe handle to all the reference-counted pointers required to make tree go brrr.

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
        └── Fresh Water
```
```rust
    use dsa_rust::trees::safe_linked_gentree::GenTree;

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

use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

/** The `Position` struct provides a safe, lightweight handle to `Node` data.
All meaningful accessors and mutators appear on the [CursorMut] struct. */
pub struct Position<T> {
    ptr: Option<Rc<RefCell<Node<T>>>>,
}
impl<T> Position<T> {
    /** Creates a handle to Node and returns it as a Position.*/
    fn new(ptr: Node<T>) -> Self {
        Position {
            ptr: Some(Rc::new(RefCell::new(ptr))),
        }
    }

    /** Returns an reference to the data at the Position, if Some. */
    fn get_data(&self) -> Option<Ref<T>> {
        let node_ref: Ref<Node<T>> = self.ptr.as_ref()?.borrow();
        Ref::filter_map(node_ref, |node| node.data.as_ref()).ok()
        //if let Some(val) = self.ptr.as_ref() {
        //    Some((*(*val)).borrow())
        //} else { None }
    }

    /** Returns the Node from a Position, if Some */
    //fn get_node(&self) -> Ref<Node<T>> {
    //    self.ptr.as_ref().unwrap().borrow()
    //}
    fn get_node(&self) -> Option<Ref<Node<T>>> {
        self.ptr.as_ref().map(|rc| rc.borrow())
    }

    /** Returns the Position for the current Position's parent, if Some. */
    //fn get_parent_pos(&self) -> Option<Position<T>> {
    //    if let Some(parent) = self.ptr.as_ref().unwrap().borrow().parent.clone() {
    //        Some(parent)
    //    } else { None }
    //}
    fn get_parent_pos(&self) -> Option<Position<T>> {
        if let Some(weak_parent) = &self.ptr.as_ref()?.borrow().parent {
            weak_parent.upgrade().map(|rc| Position { ptr: Some(rc) })
        } else {
            None
        }
    }
}
// "Shallow" clone only clones/increases the Rc, not the whole Node
impl<T> Clone for Position<T> {
    fn clone(&self) -> Self {
        Position {
            ptr: self.ptr.clone(),
        }
    }
}
impl<T> PartialEq for Position<T> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.ptr, &other.ptr) {
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        }
    }
}
impl<T> std::fmt::Debug for Position<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.ptr {
            Some(rc) => write!(f, "Position({:p})", Rc::as_ptr(rc)),
            None => write!(f, "Position(None)"),
        }
    }
}

/** Internal-only struct that represents the heart of the general tree. The `Node`
struct contains strong pointers to children, but weak pointers to parent nodes
for proper drop semantics to avoid reference cycles. */
struct Node<T> {
    parent: Option<Weak<RefCell<Node<T>>>>,
    children: Vec<Position<T>>, // Always exists for a Node, even if empty
    data: Option<T>,
}
impl<T> Node<T> {
    /** Builds a new Node and returns its position */
    fn root(data: Option<T>) -> Node<T> {
        Node {
            parent: None,
            children: Vec::new(),
            data,
        }
    }

    /** Creates a new `Node` with given data for the given `Position` */
    fn new(parent: &Position<T>, data: T) -> Node<T> {
        Node {
            //parent: Some(parent.clone()),
            parent: Some(Rc::downgrade(parent.ptr.as_ref().unwrap())),
            children: Vec::new(),
            data: Some(data),
        }
    }
}

/** The `GenTree` struct represents a positional, linked-based general
tree structure that contains a pointer to the root node and the structure's size.
The genericity of the struct means you'll have to explicitly type the
tree at instantiation.

Most of the major accessors and mutators appear on the [CursorMut] struct.

Example:
```example
    // Creates a tree over Heading objects
    let mut tree: GenTree<Heading> = GenTree::<Heading>::new();

    // Creates a CursorMut to navigate/mutate the tree,
    // starting at the root node
    let mut cursor = tree.cursor_mut();
```
*/
#[derive(Debug)]
pub struct GenTree<T> {
    root: Position<T>,
    size: usize,
}
impl<T> GenTree<T> {
    /** Instantiates a new `GenTree` */
    pub fn new() -> GenTree<T> {
        let root: Position<T> = Position::new(Node::root(None));
        GenTree { root, size: 0 }
    }

    /** Returns the `Position` of the tree's root */
    pub fn root(&self) -> Position<T> {
        self.root.clone()
    }

    /** Creates a `CursorMut` starting at the tree's root */
    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            node: self.root.clone(),
            tree: self,
        }
    }

    /** Creates a `CursorMut` from a given `Position` */
    pub fn cursor_from(&mut self, position: Position<T>) -> CursorMut<T> {
        CursorMut {
            node: position,
            tree: self,
        }
    }
}

/** A cursor over mutable `Node` data with safe, reference-counted `Position` handles.

This struct represents the majority of major operations for the [GenTree] structure.
All operations run in `O(1)` time unless otherwise noted. */
pub struct CursorMut<'a, T> {
    node: Position<T>,
    tree: &'a mut GenTree<T>,
}
impl<'a, T> CursorMut<'a, T> {
    // METADATA
    ///////////

    /** Returns `true` if the `Node` under the curosr is the tree's root */
    pub fn is_root(&self) -> bool {
        self.node.clone() == self.tree.root()
    }

    /** Returns `true` if the `Node` under the curosr has data */
    pub fn is_some(&self) -> bool {
        let val = self.node.get_data();
        val.is_some()
    }

    /** Returns `true` if the `Node` under the cursor is empty */
    pub fn is_none(&self) -> bool {
        let val = self.node.get_data();
        val.is_none()
    }

    /** Returns the number of children for the `Node` under the cursor as usize */
    pub fn num_children(&self) -> usize {
        if let Some(val) = self.node.ptr.clone() {
            (*(*val).borrow()).children.len()
        } else {
            0
        }
    }

    /** Returns the depth of the cursor from the tree's root */
    pub fn depth(&mut self) -> usize {
        let mut depth = 0;
        let current = self.current().clone();
        while !self.is_root() {
            self.ascend().ok();
            depth += 1;
        }
        self.jump(&current);
        depth
    }

    /** Returns the height of the tallest sub-tree at the current position */
    pub fn height(&self) -> usize {
        let current = self.current();
        self.height_rec(current.clone())
    }
    /** The recursive guts of the height function */
    fn height_rec(&self, node: Position<T>) -> usize {
        let mut h = 0;
        if let Some(n) = node.ptr.clone() {
            for e in &(*(*n).borrow()).children {
                h = std::cmp::max(h, self.height_rec(e.clone()))
            }
        }
        h + 1
    }

    // ACCESSORS AND MUTATORS
    /////////////////////////

    /** Returns an _immutable_ reference to the data under the cursor, if `Some` */
    pub fn get_data(&self) -> Option<Ref<T>> {
        let node_ref: Ref<Node<T>> = self.node.get_node()?;
        Ref::filter_map(node_ref, |node| node.data.as_ref()).ok()
    }

    /** Returns an _immutable_ reference to the data for a supplied `Position` */
    pub fn get_for_pos(&'a self, pos: &'a Position<T>) -> Option<Ref<'a, T>> {
        let node_ref: Ref<Node<T>> = pos.get_node()?;
        Ref::filter_map(node_ref, |node| node.data.as_ref()).ok()
    }

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

    /** Adds a new child `Node` under the current cursor and advances the cursor
    to the new child in `O(1)` time */
    pub fn add_child(&mut self, data: T) {
        let parent = self.node.clone();

        // Create the new child node and give it a Position
        let new_node = Node::new(&parent, data);
        let new_pos = Position::new(new_node);

        // Add the new child to the parent's child list
        let kids = parent.ptr.unwrap();
        (*kids).borrow_mut().children.push(new_pos.clone());

        // Mutates self to be the Position of the new node
        self.node = new_pos;

        // Increment the size of the tree
        self.tree.size += 1;
    }

    /** Returns a list of owned descendant (child) `Position`s for the `Node`
    under the cursor in `O(c)` time where `c` is the number of children; The
    clone used here is a cheap pointer copy, not an underlying data copy */
    //pub fn children(&self) -> Vec<Position<T>> {
    //    self.node
    //        .get_node()
    //        .unwrap()
    //        .children
    //        .iter()
    //        .cloned()
    //        .collect::<Vec<_>>()
    //}
    // Allocates a new Vec and clones Positions in O(n) time
    pub fn children(&self) -> Vec<Position<T>> {
        self.node
            .get_node()
            .map(|node| node.children.iter().cloned().collect())
            .unwrap_or_default()
    }

    /** Deletes the node at the current cursor position in `O(c)` time where `c`
    is the number of children for the given node; Adds all children to the parent
    (if `Some`), and returns the deleted `Node`; If the cursor is at the tree's
    root, this just deletes the `Node`'s data, leaving `None`; Moves the cursor
    to the parent, if `Some` */
    pub fn delete(&mut self) -> Option<T> {
        let self_pos = self.node.clone();
        let self_rc = self_pos.ptr.clone()?;

        // Check and get parent
        let parent_pos = self_rc.borrow().parent.as_ref()?.upgrade()?;
        let parent_pos = Position {
            ptr: Some(parent_pos),
        };
        let parent_rc = parent_pos.ptr.clone().unwrap();

        // 1. Remove self from parent.children
        {
            let mut parent_node = parent_rc.borrow_mut();
            if let Some(index) = parent_node.children.iter().position(|c| *c == self.node) {
                parent_node.children.remove(index);
            }
        }

        // 2. Take self's children (detach them)
        let mut self_children = {
            let mut self_node = self_rc.borrow_mut();
            std::mem::take(&mut self_node.children)
        };

        // 3. Reparent each child and move them to parent's children
        {
            let mut parent_node = parent_rc.borrow_mut();
            for child in &mut self_children {
                if let Some(child_rc) = child.ptr.clone() {
                    child_rc.borrow_mut().parent = Some(Rc::downgrade(&parent_rc));
                }
                parent_node.children.push(child.clone());
            }
        }

        // 4. Move cursor to parent
        self.jump(&parent_pos);

        // 5. Take and return data from the deleted node
        let mut self_node = self_rc.borrow_mut();
        self_node.data.take()
    }

    // NAVIGATION
    /////////////

    /** Returns a reference to the current `Position` */
    pub fn current(&self) -> &Position<T> {
        &self.node
    }

    /** Jump the cursor to the given `Position` */
    pub fn jump(&mut self, new: &Position<T>) {
        self.node = (*new).clone();
    }

    /** Moves the cursor up a generation, if `Some`; Trying to ascend past the root results in an error */
    pub fn ascend(&mut self) -> Result<(), String> {
        if let Some(parent) = self.node.get_parent_pos() {
            self.node = parent;
            Ok(())
        } else {
            Err("Error: Cannot ascend past root".to_string())
        }
    }
}

#[cfg(test)]
mod tests {

    // Both basic and dangle tests use the tree builder
    use crate::trees::safe_linked_gentree_builder::{construct, Heading};

    use super::{GenTree, Position};

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
        let mut tree: GenTree<Heading> = construct(1, tree_vec);

        // Tests root() -> Position<T>
        // By identity (using custom PartialEq ipml)
        assert!(tree.cursor_mut().node == tree.root);
        // By assert_eq!'s default Debug route
        let cursor = tree.cursor_mut();
        assert_eq!(cursor.node, tree.root());

        let mut cursor = tree.cursor_mut();
        // Tests that root is empty with is_some() and is_none()
        assert!(!cursor.is_some());
        assert!(cursor.is_none());
        // tests height and depth
        assert_eq!(cursor.depth(), 0);
        assert_eq!(cursor.height(), 6);

        // Tests num_children()
        assert_eq!(cursor.num_children(), 2); // Root has [Landlocked, Islands]

        // Tests children(), jump(), and get_data()
        let kids = cursor.children();
        let mut kids_iter = kids.iter();

        // Moves to first child "Landlocked"
        cursor.jump(kids_iter.next().unwrap());
        {
            let data = cursor.get_data().unwrap();
            assert_eq!(*data.title, "Landlocked".to_string());
        }
        assert_eq!(cursor.depth(), 1);
        assert_eq!(cursor.height(), 5);

        // Moves to second child "Islands"
        cursor.jump(kids_iter.next().unwrap());
        let curr: Position<Heading> = cursor.current().clone(); // Passes the torch
        {
            let data = cursor.get_data().unwrap();
            assert_eq!(*data.title, "Islands".to_string());
        }
        assert_eq!(cursor.depth(), 1);
        assert_eq!(cursor.height(), 3);

        // Jumps down a generation to [Marine, Fresh Water]
        cursor.jump(&curr);
        {
            let new_kids = cursor.children();
            let mut kids_iter = new_kids.iter();
            cursor.jump(kids_iter.next().unwrap()); // Moves to first child
            let data = cursor.get_data().unwrap();
            assert_eq!(*data.title, "Marine".to_string());
        }
        // tests height and depth
        assert_eq!(cursor.depth(), 2);
        assert_eq!(cursor.height(), 2);


        // Jumps down a generation, for fun
        let new_kids = cursor.children(); // Gets cursor's chidlren
        let mut kids_iter = new_kids.iter(); // Creates an iterator
        cursor.jump(kids_iter.next().unwrap()); // Moves to first child
        {
            let data = cursor.get_data().unwrap();
            assert_eq!(*data.title, "Australia".to_string());
        }
        assert_eq!(cursor.depth(), 3);
        assert_eq!(cursor.height(), 1);

        // Tests ascend()
        assert!(cursor.ascend().is_ok()); // Marine
        assert!(cursor.ascend().is_ok()); // Islands
        {
            let data = cursor.get_data().unwrap();
            assert_eq!(*data.title, "Islands".to_string());
        }
        assert!(cursor.ascend().is_ok()); // []
        assert!(cursor.ascend().is_err()); // Cannot ascend() past root
        assert!(cursor.is_root()); // Double checks, just in case
        assert_eq!(cursor.depth(), 0);
        assert_eq!(cursor.height(), 6);

        // Descends to Islands to test delete()
        let kids = cursor.children(); // Gets cursor's chidlren
        let mut kids_iter = kids.iter(); // Creates an iterator
        cursor.jump(kids_iter.next().unwrap()); // Moves to Landlocked
        cursor.jump(kids_iter.next().unwrap()); // Moves to Islands
        {
            let data = cursor.get_data().unwrap();
            assert_eq!(*data.title, "Islands".to_string());
        }

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
        for position in cursor.children().iter() {
            if position.get_data().unwrap().title == "Marine".to_string() {
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
        for child in cursor.children().iter() {
            let title = child.get_data().unwrap().title.clone();
            kids.push(title)
        }
        assert_eq!(kids, ["Fresh Water".to_string(), "Australia".to_string()]);
    }

    #[test]
    fn dangle() {
        use super::{GenTree, Position};
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
        let mut outer_tree: GenTree<Heading> = construct(0, one.clone());
        let mut _pos: Position<Heading> = outer_tree.root();
        let mut cursor = outer_tree.cursor_mut();

        {
            let inner_tree: GenTree<Heading> = construct(0, two.clone());
            _pos = inner_tree.root();
            cursor.jump(&_pos);

        }

        // No more UB!!
        cursor.get_data();
        _pos.get_data();

        // Creates a tree, Position, and CursorMut
        let mut outer_tree: GenTree<Heading> = construct(0, one);
        let mut pos: Position<Heading> = outer_tree.root();
        let mut cursor = outer_tree.cursor_from(pos);

        {
            let inner_tree: GenTree<Heading> = construct(0, two);
            pos = inner_tree.root();
            cursor.jump(&pos);


        }

        // No more UB!!
        cursor.get_data();
        _pos.get_data();

    }
}
