/*! A safe, indexed, n-ary tree implementation

# About
This module explores using arena allocation for fewer allocations and simpler reference munging via indexes. The biggest tradeoff includes a potentially less spatially efficient implmenetaion with the arena's growth algorithm, and logically shifting "pointers" to Nodes in the arena.

# Design

# Example

```rust
```

*/

//type Position = usize;
#[derive(Debug, PartialEq)]
pub struct Position {
    ptr: usize,
}
impl Position {
    fn new(position: usize) -> Position {
        Position { ptr: position }
    }
    fn _set(&mut self, index: usize) {
        self.ptr = index
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
    pub fn root(&self) -> &Position {
        &self.root
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

    /// Returns the number of children for a `Node` at the given `Position`.
    pub fn num_children(&self, position: &Position) -> usize {
        self.arena[position.get()].children.len()
    }

    /// UNIMPLEMENTED
    pub fn depth(&mut self, _node: &Position) -> usize {
        0
    }

    /// UNIMPLEMENTED
    pub fn height(&mut self, _node: &Position) -> usize {
        0
    }

    /// Returns an immutable reference to the data at the given `Position`, if Some.
    pub fn get_data(&self, position: &Position) -> Option<&T> {
        self.arena[position.get()].data.as_ref()
    }

    /// Adds a child to the given `Position`.
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

        // Push the new Node's index to the parent's list of children
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
        //if let Some(parent) = self.arena[position.get()].parent.clone() {
        //    Some(parent)
        //} else { None }
        self.arena[position.get()].parent.clone()
    }
}

mod builder {

    use super::*;

    #[allow(unused)]
    #[derive(Debug)]
    pub struct Heading {
        pub level: usize,
        pub title: String,
    }

    pub fn _construct(mut cur_level: usize, data: Vec<Heading>) -> GenTree<Heading> {
        // Instantiates a Tree with a generic root and traversal positioning
        let mut tree: GenTree<Heading> = GenTree::<Heading>::new();
        let mut cursor: Position = tree.root().clone(); // Sets cursor to tree.root

        // Constructs tree from Vec<T>
        for node in data {
            let title_level = node.level;

            // Case 1: Adds a child to the current parent
            if title_level == cur_level + 1 {
                cursor = tree.add_child(&cursor, node);
                cur_level += 1;
            }
            // Case 2: Adds a child with multi-generational skips
            else if title_level > cur_level {
                let diff = title_level - cur_level;
                for _ in 1..diff {
                    //let empty = Heading::new("[]".to_string(), 0);
                    let empty = Heading {
                        title: "[]".to_string(),
                        level: 0,
                    };
                    cursor = tree.add_child(&cursor, empty);
                    cur_level += 1;
                }
                cursor = tree.add_child(&cursor, node);
                cur_level += 1;
            }
            // Case 3: Adds sibling to current parent
            else if title_level == cur_level {
                cursor = tree.parent(&cursor).expect("Oopsie");
                cursor = tree.add_child(&cursor, node);
            }
            // Case 4: Adds a child to the appropriate ancestor,
            // ensuring proper generational skips
            else {
                let diff = cur_level - title_level;
                for _ in 0..=diff {
                    cursor = tree.parent(&cursor).expect("Dang!");
                    cur_level -= 1;
                }
                cursor = tree.add_child(&cursor, node);
                cur_level += 1;
            }
        }
        tree
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::hierarchies::arena_gentree::builder::{Heading, _construct};

    #[test]
    fn one() {
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
        let mut tree: GenTree<Heading> = _construct(0, tree_vec);
        let mut cursor = tree.root();

        // Tests that the tree/root contains data
        assert!(tree.get_data(cursor).is_some());
        assert!(tree.is_some(cursor));
        assert!(!tree.is_none(cursor));

        // Tests num_children()
        assert_eq!(tree.num_children(cursor), 2); // Root has [Landlocked, Islands]

        // Tests children() and get_data()
        let kids = tree.children(cursor);
        let mut kids_iter = kids.iter();

        // Steps through the root's children
        cursor = kids_iter.next().unwrap();
        let data = tree.get_data(cursor).unwrap();
        assert_eq!(*data.title, "Landlocked".to_string());
        cursor = kids_iter.next().unwrap();
        let data = tree.get_data(cursor).unwrap();
        assert_eq!(*data.title, "Islands".to_string());

        // Jumps down a generation to Islands' kids [Marine, Fresh Water]
        let new_kids = tree.children(cursor);
        kids_iter = new_kids.iter();
        cursor = kids_iter.next().unwrap();
        let data = tree.get_data(cursor).unwrap();
        assert_eq!(*data.title, "Marine".to_string());

        // Jumps down another generation, for fun
        let new_kids = tree.children(cursor).clone(); // Gets cursor's chidlren
        kids_iter = new_kids.iter(); // Creates an iterator
        cursor = kids_iter.next().unwrap(); // Moves to first child
        let data = tree.get_data(cursor).unwrap();
        assert_eq!(*data.title, "Australia".to_string());

        // Tests parent()
        let parent = tree.parent(cursor).unwrap(); // Marine
        let data = tree.get_data(&parent).unwrap();
        assert_eq!(*data.title, "Marine".to_string());
        cursor = &parent;
        let parent = tree.parent(cursor).unwrap(); // Islands
        let data = tree.get_data(&parent).unwrap();
        assert_eq!(*data.title, "Islands".to_string());
        cursor = &parent;
        let binding = tree.parent(cursor).unwrap();
        cursor = &binding; // []
        assert!(tree.parent(cursor).is_none()); // The root doesn't have any parents

        // Descends to Islands to test delete()
        let kids = tree.children(cursor); // Gets cursor's chidlren
        let mut kids_iter = kids.iter(); // Creates an iterator
        let _temp = kids_iter.next().unwrap().clone();
        let temp = kids_iter.next().unwrap().clone();
        cursor = &temp; // Moves to Islands
        {
            let data = tree.get_data(cursor).unwrap();
            assert_eq!(*data.title, "Islands".to_string());
        }

        // Tests delete()
        // Before deletion, checks that the childen are correct
        let mut kids = Vec::new();
        for child in tree.children(cursor).iter() {
            let title = tree.get_data(child).unwrap().title.clone();
            kids.push(title)
        }
        assert_eq!(kids, ["Marine".to_string(), "Fresh Water".to_string()]);

        // Creates placeholder Heading
        let mut deleted = Heading {
            title: String::new(),
            level: 0,
        };
        // Iterates through the child position's under the cursor
        // looking for a matching Heading; Once found, jumps to that position,
        // and deletes the Heading; The delete() operation automatically jumps
        // the cursor to the parent of the deleted position
        for position in tree.children(cursor).iter() {
            if tree.get_data(position).unwrap().title == *"Marine" {
                //cursor.jump(&position);
                deleted = tree.remove(position.clone()).unwrap();
                break;
            }
        }
        // Tests that the correct Heading was deleted
        assert_eq!(deleted.level, 3);
        assert_eq!(deleted.title, "Marine".to_string());

        // Tests that the cursor got bumped up to Islands
        let data = tree.get_data(cursor).unwrap();
        assert_eq!(data.title, "Islands".to_string());

        // Tests that the Islands node has the correct children
        assert_eq!(tree.children(cursor).len(), 2);
        let mut kids = Vec::new();
        for child in tree.children(cursor).iter() {
            let title = tree.get_data(child).unwrap().title.clone();
            kids.push(title)
        }
        assert_eq!(kids, ["Fresh Water".to_string(), "Australia".to_string()]);

        // Tests deleting the (empty) root
        let parent = tree.parent(cursor); // Points to the (empty) root
        let deleted = tree.remove(parent.unwrap());
        assert_eq!(deleted.unwrap().title, "[]");
        let new_root = tree.root();
        let mut kids = Vec::new();
        for child in tree.children(new_root).iter() {
            let title = tree.get_data(child).unwrap().title.clone();
            kids.push(title)
        }
        assert_eq!(
            kids,
            [
                "Switzerland".to_string(),
                "Bolivia".to_string(),
                "Islands".to_string()
            ]
        );
    }

    #[test]
    fn dangle() {
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
        let outer_tree: GenTree<Heading> = _construct(0, one);
        let mut _pos = outer_tree.root();

        {
            let inner_tree: GenTree<Heading> = _construct(0, two);
            _pos = inner_tree.root();
        } // inner_tree dropped here

        // No UB (not possible) because inner_tree and _pos is already dropped
        //let _oopsie = outer_tree.get_data(_pos);
    }
}
