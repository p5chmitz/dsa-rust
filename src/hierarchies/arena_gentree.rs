/*! A safe, indexed, n-ary tree implementation

# About
This module explores using arena allocation for fewer allocations and simpler reference munging via indexes. This implementation includes several critical compromises over the link-based approach. See the Drawbacks below for more details.

Compromises over the link-based tree include being less spatially efficient as the arena's growth algorithm logically shifts "pointers" to Nodes in the arena.

# Design
The implementation stores all `Node` values in a `Vec`-backed arena. For small trees (fewer than ~100 nodes), it is marginally slower than the `Rc<RefCell>`-based design due to fixed arena management overhead. However, for larger trees (starting around 1,000–10,000 nodes), it improves construction speed by roughly 20–25%, primarily from reduced heap allocations and better cache locality.

## Drawbacks
The Vec-backed design is intended to provide a more ergonomic design over the heavy-handed syntax and semantics of reference counting and interior mutability over the pointer-backed version of this general tree. Unfortunately, this Vec-backed design also comes with its own compromises, which may prove to be more impactful. The Vec-backed design is less spatially efficient due to the structure's growth. As the tree gets mutated, it also potentially loses cache locality.

# Example

```rust

```

*/

//type Position = usize;
#[derive(Debug, PartialEq)]
pub struct Position {
    pub ptr: usize,
}
impl Position {
    pub fn new(position: usize) -> Position {
        Position { ptr: position }
    }

    fn get(&self) -> usize {
        self.ptr
    }
}
impl Clone for Position {
    fn clone(&self) -> Self {
        Position { ptr: self.ptr }
    }
}

#[derive(Debug)]
struct Node<T> {
    parent: Option<Position>,
    children: Vec<Position>,
    data: Option<T>,
}
impl<T> Node<T> {
    fn _get_parent(&self) -> Option<&Position> {
        self.parent.as_ref()
    }
    fn get_children(&self) -> &Vec<Position> {
        &self.children
    }
}

#[derive(Debug)]
pub struct GenTree<T> {
    arena: Vec<Node<T>>,
    size: usize,
    root: Position,
    free_list: Vec<usize>,
}
impl<T> Default for GenTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> GenTree<T> {
    /// Creates a new, zero-sized `GenTree`.
    pub fn new() -> GenTree<T> {
        GenTree {
            arena: Vec::new(),
            size: 0,
            root: Position::new(0),
            free_list: Vec::new(),
        }
    }

    /// Creates a new `GenTree` that pre-allocates to the given capacity.
    pub fn new_with_capacity(capacity: usize) -> GenTree<T> {
        GenTree {
            arena: Vec::with_capacity(capacity),
            size: 0,
            root: Position::new(0),
            free_list: Vec::new(),
        }
    }

    /// Returns the `Position` to the tree's root node.
    pub fn root(&self) -> Position {
        self.root.clone()
    }

    pub fn mut_root(&mut self, data: T) {
        self.arena[0].data = Some(data);
    }

    // Unnecessary, implementing all functions on GenTree with no mutable borrows
    // pub fn cursor_mut(&mut self) -> CursorMut<'_, T>
    // pub fn cursor_from(&mut self, position: Position<T>) -> CursorMut<'_, T>

    /// Indicates whether the given `Position` is the tree's root.
    pub fn is_root(&self, position: &Position) -> bool {
        position.ptr == self.root.ptr
    }

    /// Indicates whether the given `Position` contains data.
    pub fn is_some(&self, position: &Position) -> bool {
        self.arena[position.get()].data.is_some()
    }

    /// Indicates whether the given `Position` contains data.
    pub fn is_none(&self, position: &Position) -> bool {
        self.arena[position.ptr].data.is_none()
    }

    /// Indicates whether the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the number of live nodes in the tree.
    pub fn size(&self) -> usize {
        self.size 
    }

    /// Returns the number of children for a `Node` at the given `Position`.
    pub fn num_children(&self, position: &Position) -> usize {
        self.arena[position.get()].children.len()
    }

    /// WARNING: Unimplemented
    pub fn depth(&mut self, _node: &Position) -> usize {
        0
    }

    /// WARNING: Unimplemented
    pub fn height(&mut self, _node: &Position) -> usize {
        0
    }

    ///// Returns the given `Position`'s parent, if Some.
    //pub fn get_parent(&self, position: &Position) -> Option<Position> {
    //    let mut pos = Position::new(0);
    //    if position.ptr < self.arena.len() {
    //        let val = self.arena[position.ptr].parent.clone().unwrap().ptr;
    //        pos.ptr = val;
    //        Some(pos)
    //    } else { None }  
    //}

    //pub fn jump(&self, position: Position) {} 

    // /// Returns a list of the given `Position`'s children.
    // pub fn get_children(&self) -> Vec<Position> {}

    /// Returns an immutable reference to the data at the given `Position`, if Some.
    pub fn get_data(&self, position: &Position) -> Option<&T> {
        self.arena[position.get()].data.as_ref()
    }

    /// Returns an immutable reference to the data at the given `Position`, if Some.
    pub fn get_for_pos(&self, position: &Position) -> Option<&T> {
        if position.ptr < self.arena.len() {
            self.arena[position.get()].data.as_ref()
        } else { panic!("Error: index out-of-bounds") }
    }

    /// Adds a child to the given `Position` and returns its `Position`.
    pub fn add_child(&mut self, position: &Position, data: T) -> Position {
        // Creates a new Node
        let node = Node {
            parent: if self.arena.is_empty() {
                None
            } else {
                Some(position.clone())
            },
            children: Vec::new(),
            data: Some(data),
        };

        // Finds the appropriate index to insert the node;
        // If there are unused indexes in the free list, push there,
        // if there are no free indexes, append the list and set the index at
        // the end of the arena
        let index = if let Some(reuse) = self.free_list.pop() {
            self.arena[reuse] = node;
            reuse
        } else {
            self.arena.push(node);
            self.arena.len() - 1
        };

        // Push the new Node's index to the parent's list of children;
        // If the position has no parent, add to root's list of children
        if self.arena.len() == 1 {
            // No-op: The Node being pushed is the only node
        } else {
            self.arena[position.get()]
                .children
                .push(Position::new(index))
        };

        // Increase the size of the tree's size counter and returns the new Node's index
        self.size += 1;
        Position::new(index)
    }

    /// Returns a reference to the list of child `Position`s for the given `Position`.
    //pub fn children(&self, position: &Position) -> Option<&Vec<Position>> {
    //    if position.ptr < self.arena.len() {
    //        Some(self.arena[position.get()].get_children())
    //    } else { None }
    //}
    pub fn children(&self, position: &Position) -> &Vec<Position> {
        self.arena[position.get()].get_children()
    }

    #[allow(clippy::style)]
    /// Removes a node at the given `Position` and returns its data.
    /// If the removed `Position` has a parent, all the deleted node's children
    /// get pushed to the deleted `Position`'s parent. If the deleted `Position`
    /// has no parent (root) _a new parent is designated as the first child of
    /// the deleted node_, e.g. this is a tree, not a forest. Ordering for the
    /// designation of the new root is not guaranteed, and depends on when child
    /// `Position`s were added. For this reason, caution is advised when
    /// potentially deleting root nodes.
    ///
    /// Runs in `O(s + c)` time where:
    /// - `s` is the number of the deleted node's siblings (i.e. the number of
    ///    its parent's children),
    /// - `c` is the number of the deleted node's children.    
    ///
    /// Precondition:
    /// ```text
    ///           3
    ///        /    \
    ///       5      9 (marked for deleteion)
    ///     /  \    /  \
    ///    8   12  11  14
    ///
    /// ```
    ///
    /// Postcondition:
    /// ```text
    ///           3
    ///        /  |  \
    ///       5   11  14
    ///     /  \    
    ///    8   12  
    ///
    /// ```
    ///
    /// Precondition:
    /// ```text
    ///           3 (marked for deletion)
    ///        /  |  \
    ///       5   11  14
    ///     /  \    
    ///    8   12  
    ///
    /// ```
    ///
    /// Postcondition:
    /// ```text
    ///          5
    ///     /  /   \  \
    ///    8  12   11 14
    ///
    /// ```
    ///
    /// Precondition:
    /// ```text
    ///           3 (marked for deletion)
    ///        /    \
    ///       9      5
    ///     /  \    /  \
    ///    8   12  11  14
    ///
    /// ```
    ///
    /// Postcondition:
    /// ```text
    ///           9
    ///        /  |  \
    ///       8  12   5
    ///              /  \
    ///             11   14
    /// ```
    pub fn remove(&mut self, position: Position) -> Option<T> {
        // Gets the data out of the deleted Node, leaving None in its place
        let data = self.arena[position.ptr].data.take();

        // Adds deleted Position to the free list
        self.free_list.push(position.ptr);

        // Gets the deleted Node's parent's position
        let parent_pos = self.arena[position.ptr].parent.clone();

        // Decrease the size of the tree
        self.size -= 1;

        // Move all of the deleted Node's children up a generation, if theres a parent
        if let Some(parent_node) = parent_pos {
            // Alter the parent's children to exclude the deleted node
            self.arena[parent_node.ptr]
                .children
                .retain(|p| *p != position);

            // Push the deleted node's children to the parent node's children
            //let children: Vec<_> = self.arena[position.ptr].children.to_vec();
            let children = std::mem::take(&mut self.arena[position.ptr].children);
            for child in children {
                self.arena[parent_node.get()].children.push(child.clone());
                self.arena[child.ptr].parent = Some(parent_node.clone());
            }
            data
        }
        // If the deleted Node has no parent (root), make the first child the new
        // root (if it has children), and add old siblings as children
        else {
            if !self.arena[position.ptr].children.is_empty() {
                // Remove and promote the first child as new root
                let new_root = self.arena[position.ptr].children.remove(0);
                self.root = new_root.clone();

                // Move the remaining children vector out of the node to avoid cloning each element
                let remaining_children = std::mem::take(&mut self.arena[position.ptr].children);

                // Extend the new root's children with the remaining children (skipping the removed root)
                self.arena[new_root.ptr].children.extend(remaining_children);
            } // If the list only contains the root, then the free list should allow the position
              // to be overwritten
            data
        }
    }

    /// Returns the `Position` of a given `Position`'s parent, if Some.
    pub fn parent(&mut self, position: &Position) -> Option<Position> {
        #[allow(clippy::manual_map)]
        if let Some(parent) = self.arena[position.get()].parent.clone() {
            Some(parent)
        } else { None }
        //self.arena[position.get()].parent.clone()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    /// TODO: actually test the structure's members!
    fn atomic() {
    
        use super::GenTree;
        use crate::hierarchies::arena_gentree_builder::Heading;

        let mut tree = GenTree::new();
        assert_eq!(tree.size(), 0);
        assert!(tree.is_empty());
        let root = tree.root().clone();
        let mut cursor = tree.add_child(&root, Heading {
                level: 2,
                title: "Landlocked".to_string(),
            },
        );
        assert_eq!(tree.size(), 1);
        assert!(!tree.is_empty());

        cursor = tree.add_child(&cursor, Heading {
                level: 3,
                title: "Switzerland".to_string(),
            },
        );
        cursor = tree.add_child(&cursor, Heading {
                level: 4,
                title: "Geneva".to_string(),
            },
        );
        cursor = tree.add_child(&cursor, Heading {
                level: 5,
                title: "Old Town".to_string(),
            },
        );
        cursor = tree.parent(&cursor).expect(""); // Geneva
        cursor = tree.parent(&cursor).expect(""); // Switzerland
        tree.add_child(&cursor, Heading {
                level: 3,
                title: "Botswana".to_string(),
            },
        );
        assert_eq!(tree.size(), 5);

        //eprintln!("{tree:#?}");
        //panic!("MANUAL TEST FAILURE");
    }

    #[test]
    fn dangle() {
        use crate::hierarchies::arena_gentree_builder::{Heading, construct};

        use super::GenTree;
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
        let outer_tree: GenTree<Heading> = construct(0, one);
        //let outer_tree: GenTree<Heading> = construct_from(one);
        let mut _pos = outer_tree.root();

        {
            let inner_tree: GenTree<Heading> = construct(0, two);
            //let inner_tree: GenTree<Heading> = construct_from(two);
            _pos = inner_tree.root();
        } // inner_tree dropped here

        // No UB (not possible) because inner_tree and _pos is already dropped
        //let _oopsie = outer_tree.get_data(_pos);
    }
}
