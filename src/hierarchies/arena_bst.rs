/*! A safe, arena-based (indexed) binary search tree (BST)

# About
A binary search tree is a binary tree where the value of each node's child is less than or greater than its parent. Left children are less than their parents, and right children are greater than their parents.

Due to common usage when implementing sorted map and set structures, this implementation does not accept duplicate entries.

# Design
The design uses a flat, [Vec]-backed structure with iterative navigation. This arena-allocated design provides robust, performant operations while keeping runtime checks to a minimum. The goal of this approach is to avoid unnecessary overhead with recursive operations and extra heap allocations, making it suitable for low-spec environments.

# Example
*/

#![allow(dead_code)] // While we cook

use std::cmp::Ordering;

// Experiment in custom sum types
enum Position {
    // Key exists
    Found(usize),
    // Insertion information for new key
    Parent(usize),
}

enum Side {
    Left,
    Right,
}

#[derive(Debug)]
struct Node<T> {
    index: usize,     // Strictly necessary?
    value: Option<T>, // Wrapping the value allows us quick take() operations
    parent: Option<usize>,
    left: Option<usize>, // Explicit and simple
    right: Option<usize>,
    height: usize,
}
impl<T> Node<T> {
    // Creates a new node with its current index, a value, and its parent (if Some).
    // Guarantees that all nodes have a value.
    fn new(index: usize, value: T, parent: Option<usize>) -> Self {
        Node {
            index,
            value: Some(value),
            parent,
            left: None,
            right: None,
            height: 0,
        }
    }

    // Gets the parent index of the node, if Some
    fn get_parent(&self) -> Option<usize> {
        self.parent
    }

    /// Returns a tuple of the node's children
    /// 0 == left
    /// 1 == right
    fn get_children(&self) -> (Option<usize>, Option<usize>) {
        (self.left, self.right)
    }

    // Returns a reference to the node's value, if Some
    fn get_value(&self) -> Option<&T> {
        self.value.as_ref()
        //match &self.value {
        //    Some(val) => Some(val),
        //    None => None
        //}
    }
}

#[derive(Debug)]
pub struct BinSearchTree<T> {
    arena: Vec<Node<T>>,
    root: Option<usize>, // Wont always be 0, and wont always be Some!
}
// Im just here to make Clippy happy
impl<T> Default for BinSearchTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> BinSearchTree<T>
where
    T: Ord,
{
    /// Creates a new, empty binary search tree.
    pub fn new() -> Self {
        BinSearchTree {
            arena: Vec::new(),
            root: None,
        }
    }

    /// Creates a new, empty binary search tree with a given (growable) initial capacity.
    pub fn new_with_capacity(size: usize) -> Self {
        BinSearchTree {
            arena: Vec::with_capacity(size),
            root: Some(0),
        }
    }

    /// The index of a found key or the root of
    /// the sub-tree for its insertion point, if Some.
    /// Returns None for an empty/corrupted tree.
    /// Returns Option because the structure cannot guarantee initialized trees.
    ///
    /// This naive implementation can be used independently or within
    /// a map that does _not_ support duplicates as it does not
    /// discriminate between found and not found.
    fn find_index(&self, key: &T) -> Option<usize>
    where
        T: Ord,
    {
        let mut current = self.root?; // Return None if empty
        while let Some(node) = self.arena.get(current) {
            let value = node.get_value()?; // Return None if corrupted
            match value.cmp(key) {
                Ordering::Less => {
                    if let Some(right) = node.right {
                        current = right;
                    } else {
                        break;
                    } // return insert position
                }
                Ordering::Greater => {
                    if let Some(left) = node.left {
                        current = left;
                    } else {
                        break;
                    } // return insert position
                }
                Ordering::Equal => break, // key found
            }
        }
        Some(current)
    }

    /// Returns a `Position` that contains the index of the key if `Found`,
    /// or the root of the sub-tree for its insertion point as `Parent`,
    /// which could be 0 for an empty tree.
    /// (All trees are guaranteed to be initialized)
    fn find_index_enum(&self, _key: &T) -> Position {
        if let Some(mut _index) = self.root {
            return Position::Found(0);
        }

        // Search iteratively from the root to find a match or insertion parent
        //while let Some(index) = self.arena.get(self.root.expect("Uh oh")) {
        //    // SAFETY: Every node created with Node::new() is guaranteed to have a value
        //    match index.value.as_ref().unwrap().cmp(key) {
        //        Ordering::Less => {
        //            if let Some(right) = index.right {
        //                index = right;
        //            } else {
        //                break;
        //            }
        //        }
        //        Ordering::Greater => {
        //            if let Some(left) = index.left {
        //                index = left;
        //            } else {
        //                break;
        //            }
        //        }
        //        Ordering::Equal => break,
        //    }
        //}
        //
        //index
        Position::Found(0)
    }

    /// Returns a tuple that contains information for whether the key exists
    /// in the tree, and if so its index, if not, the insertion point (parent index)
    /// Examples:
    /// - The key is in the tree at index 23: (Some(23), None)
    /// - The key is not in the index, but its parent should be index 7 (None, Some(7))
    ///
    /// Use the following pseudocode for contains():
    /// ```text
    /// match tree.find_index_exact(key).0 {
    ///   Some(val) => true,
    ///   None => false,
    /// }
    /// ```
    ///
    /// Use the following pseudocode to locate the insertion point:
    /// ```text
    /// if let Some(parent) = tree.find_index_exact(key).1 { ... }
    /// ```
    fn find_index_exact(&self, key: &T) -> (Option<usize>, Option<usize>) {
        if let Some(mut index) = self.root {
            //let mut index = self.root;
            let mut tuple = (None, None);

            while let Some(node) = self.arena.get(index) {
                // SAFETY: Every node created with Node::new() is guaranteed to have a value
                match node.value.as_ref().unwrap().cmp(key) {
                    Ordering::Less => {
                        // Go right, continue search
                        if let Some(right) = node.right {
                            index = right;
                        } else {
                            // The key should be the right child of index
                            tuple = (None, Some(index));
                            break;
                        }
                    }
                    Ordering::Greater => {
                        // Go left, continue search
                        if let Some(left) = node.left {
                            index = left;
                        } else {
                            // The key should be the left child of index
                            tuple = (None, Some(index));
                            break;
                        }
                    }
                    // The key exists at index
                    Ordering::Equal => {
                        tuple = (Some(index), None);
                        break;
                    }
                }
            }

            tuple
        } else {
            // The tree is empty
            (None, None)
        }
    }

    fn get_node(&self, index: usize) -> Option<&Node<T>> {
        if index < self.arena.len() {
            Some(&self.arena[index])
        } else {
            None
        }
    }

    /// Inserts the given key into the tree. New insertions in a simple
    /// BST are always leafs. This implementation does not guarantee uniqueness.
    pub fn insert(&mut self, key: T) {
        if !self.arena.is_empty() {
            // Find the (presumptive) parent index
            let parent = self.find_index(&key);

            // If the parent == key, no op, otherwise its indeed the parent
            //match &key.cmp(self.arena[parent_index].value.as_ref().unwrap()) {
            match &key.cmp(
                self.get_node(parent.expect(""))
                    .unwrap()
                    .get_value()
                    .as_ref()
                    .unwrap(),
            ) {
                // The key already exists, no op
                Ordering::Equal => {}
                // The key is less than the value at the index, go left
                Ordering::Less => {
                    // New node is a left child of parent
                    self.arena[parent.expect("")].left = Some(self.arena.len())
                    //self.get_node(parent).unwrap().get_children().0 = Some(self.arena.len())
                }
                // The key is greater than the value at the index, go right
                Ordering::Greater => {
                    // parent is true!
                    // New node is a right child of parent
                    self.arena[parent.expect("")].right = Some(self.arena.len())
                }
            }

            // Create & insert the node
            self.arena.push(Node::new(self.arena.len(), key, parent));
        } else {
            // The list is empty, insert a new root
            self.arena.push(Node::new(0, key, None));
        }
    }

    fn is_leaf(&self, index: usize) -> Result<bool, &str> {
        match self.get_node(index) {
            Some(val) => {
                if val.get_children() == (None, None) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Err("Invalid index"),
        }
    }

    pub fn remove(&mut self, key: T) -> Option<T> {
        // Exists with None if the key is not in the tree
        if let Some(index) = self.find_index(&key) {
            //let index = self.find_index(&key);

            // Check if the key is actually in the tree by
            // checking if the index represents the key or its parent
            //
            // The key is in the index
            if self.arena[index].value.as_ref().unwrap() == &key {
                // Case 1) The node to delete has two children
                if self.arena[index].left.is_some() && self.arena[index].right.is_some() {
                    // TODO: Implement

                    // Get the node, take its value
                    self.arena[index].value.take()

                // Case 2) The node has only one child
                } else if self.arena[index].left.is_some() || self.arena[index].right.is_some() {
                    // Update the parent's child pointer to the removed node's child
                    if self.arena[index].parent.is_some() {
                        // Determine whether the removed is the right or left child of its parent
                        let parent = self.arena[index].parent.unwrap();

                        // The removed node is the right child of its parent
                        if self.arena[parent].right.as_ref().unwrap() == &index {
                            // The removed node only has a left child
                            if self.arena[index].left.is_some() {
                                // Update the parent's right child pointer to the removed node's left
                                // child
                                self.arena[parent].right = Some(self.arena[index].left?);
                                // Update the removed node's left child to point to the removed node's
                                // parent
                                let child = self.arena[index].left.unwrap();
                                self.arena[child].parent = Some(parent);
                            // The removed node only has a right child
                            } else {
                                // Update the parent's right child pointer to the removed node's right
                                // child
                                self.arena[parent].right = Some(self.arena[index].right?);
                                // Update the removed node's right child to point to the removed node's
                                // parent
                                let child = self.arena[index].right.unwrap();
                                self.arena[child].parent = Some(parent);
                            }
                        }
                        // The removed node is the left child
                        else {
                            // The removed node only has a left child
                            if self.arena[index].left.is_some() {
                                // Update the parent's left child pointer to the removed node's left
                                // child
                                self.arena[parent].left = Some(self.arena[index].left?);
                            // The removed node only has a right child
                            } else {
                                // Update the parent's left child pointer to the removed node's right
                                // child
                                self.arena[parent].left = Some(self.arena[index].right?);
                            }
                        }
                    }

                    // Get the node, take/return its value
                    self.arena[index].value.take()

                // Case 3) The node has no children
                } else {
                    None
                }

                // update the parent's child pointer
                // update the replacement node's parent pointer
            }
            // The key is not in the tree; no op
            else {
                None
            }
        } else {
            None
        }
    }

    /// Produces a "snapshot" iterator over immutable references to the
    /// tree in its current state.
    pub fn iter(&self) -> InOrderIter<'_, T> {
        InOrderIter::new(&self.arena[self.root.expect("IDK man")])
    }
}

pub struct InOrderIter<'a, T> {
    stack: Vec<&'a Node<T>>,
    current: Option<&'a Node<T>>,
}
impl<'a, T> InOrderIter<'a, T> {
    fn new(root: &'a Node<T>) -> Self {
        Self {
            stack: Vec::new(),
            current: Some(root),
        }
    }
}
//impl<'a, T> Iterator for InOrderIter<'a, T> {
//    type Item = &'a T;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        while let Some(node) = self.current {
//            self.stack.push(node);
//            self.current = item.arena[node.left];
//        }
//
//        self.stack.pop().map(|node| {
//            self.current = node.right.as_deref();
//            &node.value
//        })
//    }
//}

//#[test]
//fn basic_bst() {
//    let mut bst: BinSearchTree<u8> = BinSearchTree::new();
//
//    // Produces a BST
//    let v = [31, 13, 23, 39, 41, 43, 8, 17, 19];
//    for e in v.iter() {
//        bst.insert(*e);
//    }
//
//    eprintln!("{bst:#?}");
//    assert_eq!(bst.arena[bst.root.expect("No root")].value, Some(31));
//    let root_node = &bst.arena[bst.root.expect("nah, brah")];
//    assert_eq!(bst.arena[root_node.left.unwrap()].value, Some(13));
//    assert_eq!(bst.arena[root_node.right.unwrap()].value, Some(39));
//}
