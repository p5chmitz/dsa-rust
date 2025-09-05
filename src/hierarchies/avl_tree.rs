/*! A safe, arena-backed (indexed) AVL tree

# About
Adelson-Velsky and Landis (AVL) trees represent theoretically optimal balanced binary search trees. AVL trees guarantee _~1.44 * log(n)_ height, and provide _O(log(n))_ search, insert, and delete operations. Red-black trees tend to be more popular even though they only guarantee _<= 2 * log(n)_ height. The imperfect height is offset by the fact that red-black trees often require fewer rotations, and the average number of rebalance operations is fewer than with AVL trees.

# Design
The design uses a flat, [Vec]-backed structure with iterative (read non-recursive) navigation. This arena-allocated design provides robust, performant operations while keeping runtime checks to a minimum. The goal of this approach is to avoid unnecessary overhead with recursive operations and extra heap allocations, making it suitable for low-spec environments.

The structure trades spatial efficiency for _O(1)_ insert operations. All "pointers" represent absolute positions (as indexes) and cannot be re-calculated in less than _O(n)_ time. Thus, `remove(key)` operations do not actually shrink the physical size of the structure, leaving a `None` "hole" in the removed node's place. The structure can only grow.

Due to common usage when implementing sorted map and set structures, this implementation does not accept duplicate entries by default. The structure contains an internal `SearchResult` enum that allows for duplicates by way of `Ordering::Equal`, but it is yet implemented in this version.

# Example
```rust
    use dsa_rust::hierarchies::avl_tree::AVLTree;

    let mut tree: AVLTree<u8> = AVLTree::new();

    // Create the following AVL tree
    // 
    //           39      
    //          /  \      
    //        17    41      
    //       /  \     \   
    //     13   23     43
    //     /   /  \           
    //    8   19  31          
    //
    let v = [31, 13, 23, 39, 41, 43, 8, 17, 19];
    for e in v.iter() {
        tree.insert(*e);
    }
    assert_eq!(tree.get_root(), Some(&39)); // Prove that its properly rooted


    // Remove 41 which results in the following restructure
    // 
    //         17
    //        /  \
    //      13    39
    //     /     /  \
    //    8     23   43
    //         /  \
    //        19   31
    //
    assert!(tree.contains(&41)); // Prove that its here today
    let removed = tree.remove(&41).unwrap();
    assert_eq!(removed, 41);
    assert!(!tree.contains(&41)); // ...and gone tomorrow

    tree.insert(34);
    tree.insert(67);
    tree.insert(2);
    tree.insert(4);
    tree.insert(5);
    tree.insert(1);

    // The tree is still intact, and its root has shifted
    assert_eq!(tree.get_root().unwrap(), &31);

    // In-order "snapshot" iterator
    let mut sorted = Vec::new();
    for e in tree.iter() {
        sorted.push(*e) 
    }
    assert_eq!(sorted, [1, 2, 4, 5, 8, 13, 17, 19, 23, 31, 34, 39, 43, 67]);
```
*/

#![allow(dead_code)] // While we cook

use std::cmp::{max, Ordering};

// Custom sum type for search algorithm
enum SearchResult {
    // The tree is empty (and uninitialized)
    None,
    // Index of a found key
    Exists(usize),
    // Index for insertion of a new key
    Parent(usize),
    //Parent { index: usize, side: Side },
}

#[derive(PartialEq)]
enum Side {
    Left,
    Right,
}
// NOTE: Implementing Not provides the ability to use the 
// logical negation operator !. Implementing a custom
// opposite() does the same thing, but more explicitly. 
// The choice to implement Not for &Side instead of Side
// allows the re-use of Side without making it Copy/Clone.
// Example:
//    let subtree = self.arena[child_idx].child(!side);
//    let subtree = self.arena[child_idx].child(opposite(side));
impl<'a> std::ops::Not for &'a Side {
    type Output = &'a Side;
    fn not(self) -> &'a Side {
        match self {
            Side::Left => &Side::Right,
            Side::Right => &Side::Left,
        }
    }
}
//impl std::ops::Not for Side {
//    type Output = Side;
//    fn not(self) -> Side {
//        match self {
//            Side::Left => Side::Right,
//            Side::Right => Side::Left,
//        }
//    }
//}
fn opposite(side: &Side) -> &Side {
    match side {
        Side::Left => &Side::Right,
        Side::Right => &Side::Left,
    }
}

#[derive(Debug)]
struct AVLNode<T> {
    value: Option<T>,      // Option allows take() operations for efficient node removals
    parent: Option<usize>, // None indicates the root of the tree
    left: Option<usize>,
    right: Option<usize>,
    height: usize,

    // Self-referencial position value for testing/debug ops to check algorithmic correctness
    // TODO: remove in final implementation
    index: usize,
}
impl<T> AVLNode<T> {
    // Creates a new node with its current index, a value, and its parent (if Some).
    // Guarantees that all nodes have a value. 
    // All initial inserts are leafs before restructuring, so left and right are set to None.
    fn new(index: usize, value: T, parent: Option<usize>, height: usize) -> Self {
        AVLNode {
            index,
            value: Some(value),
            parent,
            left: None,
            right: None,
            height,
        }
    }

    // /// Gets the parent index of the node, if Some; None represents the tree's root
    // fn get_parent(&self) -> Option<usize> {
    //     self.parent
    // }

    // /// Returns a tuple of the node's children
    // fn get_children(&self) -> (Option<usize>, Option<usize>) {
    //     (self.left, self.right)
    // }

    // Returns a reference to the node's value, if Some
    fn get_value(&self) -> Option<&T> {
        self.value.as_ref()
        //match &self.value {
        //    Some(val) => Some(val),
        //    None => None
        //}
    }

    //fn is_leaf(&self) -> bool {
    //    self.left.is_none() && self.right.is_none()
    //}

    /// Get child index for a given side
    fn child(&self, side: &Side) -> Option<usize> {
        match side {
            Side::Left => self.left,
            Side::Right => self.right,
        }
    }

    /// Set child index for a given side
    fn set_child(&mut self, side: &Side, idx: Option<usize>) {
        match side {
            Side::Left => self.left = idx,
            Side::Right => self.right = idx,
        }
    }

}

/// # About
///
/// See the [module-level documentation](crate::hierarchies::avl_tree) for more information.
#[derive(Debug)]
pub struct AVLTree<T> {
    // Option wrapper for efficient take() operations during removal
    // without incurring the wrath of O(n) resize ops
    arena: Vec<Option<AVLNode<T>>>, 
    // Wont always be 0, and wont always be Some!
    root: Option<usize>, 
}
// Im just here to make Clippy happy
impl<T> Default for AVLTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> AVLTree<T>
where
    T: Ord,
{
    /// Creates a new, empty binary search tree.
    pub fn new() -> Self {
        AVLTree {
            arena: Vec::new(),
            root: None,
        }
    }

    /// Creates a new, empty binary search tree with a given (growable) initial capacity.
    pub fn new_with_capacity(size: usize) -> Self {
        AVLTree {
            arena: Vec::with_capacity(size),
            root: Some(0),
        }
    }

    /// Immutable node accessor 
    fn node(&self, index: usize) -> &AVLNode<T> {
        self.arena[index].as_ref().expect("Error: Invalid immutable node access")
    }

    /// Gets a reference to the value of a key, if Some.
    //pub fn get_node(&self, key: &T) -> Option<&T> {
    //    if let SearchResult::Exists(index) = self.search(key) {
    //        self.node(index).get_value()
    //    } else {
    //        None
    //    }
    //}
    pub fn get_node(&self, key: &T) -> Option<&T> {
        match self.search(key) {
            SearchResult::Exists(index) => Some(self.node(index).get_value()?),
            _ => None,
        }
    }

    /// Mutable node accessor
    fn node_mut(&mut self, index: usize) -> &mut AVLNode<T> {
        self.arena[index].as_mut().expect("Error: Invalid mutable node access")
    }

    //fn get_root_index(&self) -> Option<usize> {
    //    self.root
    //}

    pub fn get_root(&self) -> Option<&T> {
        if let Some(node) = self.root {
            self.node(node).get_value()
        } else { None }
    }

    /// Returns a `SearchResult` enum with the following variants:
    /// - None: Indicates an empty tree
    /// - Parent: The key is not in the tree, but can be inserted at the parent index value
    /// - Exists: The key was found in the tree; The caller can decide how to use this index
    ///   to deal with multi-maps and sets
    ///
    /// SAFETY: May panic if a node does not contain a value, but that 
    /// would violate the AVL tree invariant, so its highly unlikely, and only present to
    /// handle the possibility of corrupted structures.
    fn search(&self, key: &T) -> SearchResult
    where
        T: Ord,
    {
        // Early return for empty structures
        if self.arena.is_empty() {
            return SearchResult::None;
        };

        // Sets the starting point for the search
        // Safety: Valueless nodes violate the AVL tree invariant
        let mut current = self
            .root
            .expect("Error: Root should always contain a value");

        // Uses iterative loop instead of recursive search
        // because fuck stack overflows (and recursion)
        loop {
            if let Some(val) = &self.arena[current] {
                // Safety: Valueless nodes violate the AVL tree invariant
                let value = val
                    .get_value()
                    .expect("Error: Node does not contain a value");

                match value.cmp(key) {
                    // Go right or return parent
                    Ordering::Less => {
                        if let Some(right) = val.right {
                            current = right;
                        } else {
                            return SearchResult::Parent(current);
                        }
                    }
                    // Go left or return parent
                    Ordering::Greater => {
                        if let Some(left) = val.left {
                            current = left;
                        } else {
                            return SearchResult::Parent(current);
                        }
                    }
                    // The key already exists in the tree at the current index
                    Ordering::Equal => return SearchResult::Exists(current),
                }
            };
        }
    }

    /// Returns true if the key exists in the tree.
    pub fn contains(&self, key: &T) -> bool {
        //match self.search(key) {
        //    SearchResult::Exists(_) => true,
        //    _ => false,
        //}
        matches!(self.search(key), SearchResult::Exists(_))
    }

    /// Inserts the given key into the tree maintaining an AVL structure.
    ///
    /// NOTE: Does not handle duplicate keys by convention, but may overwrite values for
    /// arbitrarily complex T with custom Ordering.
    pub fn insert(&mut self, key: T) {
        match self.search(&key) {
            SearchResult::Parent(parent) => {
                // Determine child position
                let new_idx = self.arena.len();
                // SAFETY: Parent is guaranteed by search() enums
                //if let Some(val) = &self.arena[parent].as_mut().unwrap().value {
                if let Some(val) = &self.node_mut(parent).value {
                    match key.cmp(val) {
                        Ordering::Less => { 
                            //self.arena[parent].as_mut().unwrap().left = Some(new_idx);
                            self.node_mut(parent).left = Some(new_idx);
                        },
                        Ordering::Greater => { 
                            //self.arena[parent].as_mut().unwrap().right = Some(new_idx);
                            self.node_mut(parent).right = Some(new_idx);
                        },
                        Ordering::Equal => {}
                    }
                };

                // Insert new node
                self.arena.push(Some(AVLNode::new(new_idx, key, Some(parent), 1)));

                // Walk up the tree to update heights and rebalance
                let mut current = Some(parent);
                while let Some(idx) = current {
                    self.update_node_height(idx);
                    self.restructure(idx);
                    //current = self.arena[idx].as_mut().unwrap().parent;
                    current = self.node_mut(idx).parent;
                }
            }
            SearchResult::None => {
                // Empty tree, insert root
                let new_node = AVLNode::new(0, key, None, 1);
                self.arena.push(Some(new_node));
                self.root = Some(0);
            }
            SearchResult::Exists(_) => {}
        }
    }

    /// Removes and returns an element from the AVL tree as an owned value.
    pub fn remove(&mut self, key: &T) -> Option<T> {
        let target_index = match self.search(key) {
            SearchResult::Exists(idx) => idx,
            _ => return None,
        };
    
        // Step 1: Find node to physically remove (node with ≤1 child)
        let mut remove_index = target_index;
        if self.node(remove_index).left.is_some() && self.node(remove_index).right.is_some() {
            // Node has two children: find in-order successor
            let mut succ_index = self.node(remove_index).right.unwrap();
            while let Some(left) = self.node(succ_index).left {
                succ_index = left;
            }
    
            // Move successor's value into target node
            let succ_value = self.arena[succ_index].take().unwrap().value;
            self.node_mut(remove_index).value = succ_value;
    
            // Now remove the successor node (guaranteed ≤1 child)
            remove_index = succ_index;
        }
    
        // Step 2: Identify the child of the node to remove (if any)
        let child_index = self.node(remove_index).left.or(self.node(remove_index).right);
    
        // Step 3: Update parent to point to the child
        let parent_index = self.node(remove_index).parent;
        if let Some(p_idx) = parent_index {
            let parent = self.node_mut(p_idx);
            if parent.left == Some(remove_index) {
                parent.left = child_index;
            } else {
                parent.right = child_index;
            }
        } else {
            // Removing root
            self.root = child_index;
        }
    
        // Step 4: Update child's parent
        if let Some(c_idx) = child_index {
            self.node_mut(c_idx).parent = parent_index;
        }
    
        // Step 5: Take the node for return
        let removed_value = self.arena[remove_index].take().map(|n| n.value);
    
        // Step 6: Walk up ancestors to update heights and rebalance
        let mut current = parent_index;
        while let Some(idx) = current {
            self.update_node_height(idx);
            self.restructure(idx);
            current = self.node(idx).parent;
        }
    
        removed_value?
    }

    /// Updates the height of an arbitrary node in an AVL tree
    /// where leaf nodes are defined as having height 1
    //fn update_node_height(&mut self, index: usize) {
    //    let left = self.arena[index].as_mut().unwrap().left.map_or(0, |idx| self.arena[idx].as_mut().unwrap().height);
    //    let right = self.arena[index].as_mut().unwrap().right.map_or(0, |idx| self.arena[idx].as_mut().unwrap().height);
    //    // Works for internal and leaf nodes, because max(0, 0) + 1 = 1
    //    self.arena[index].as_mut().unwrap().height = max(left, right) + 1
    //}
    fn update_node_height(&mut self, index: usize) {
        let left = self.node_mut(index).left.map_or(0, |idx| self.node_mut(idx).height);
        let right = self.node_mut(index).right.map_or(0, |idx| self.node_mut(idx).height);
        // Works for internal and leaf nodes, because max(0, 0) + 1 = 1
        self.node_mut(index).height = max(left, right) + 1
    }

    /// Rotate the subtree rooted at `root_idx` in the given direction.
    /// `side` is the direction of the original heavy side (Side::Left or Side::Right).
    fn rotate(&mut self, root_idx: usize, side: &Side) {
        // Heavy child becomes the new root of the subtree
        let child_idx = self.node_mut(root_idx)
            .child(side)
            .expect("Rotation requires heavy child");

        // Move child's opposite subtree into root's heavy side
        //let subtree = self.arena[child_idx].child(opposite(side));
        let subtree = self.node_mut(child_idx).child(!side);
        self.node_mut(root_idx).set_child(side, subtree);
        if let Some(sub_idx) = subtree {
            self.node_mut(sub_idx).parent = Some(root_idx);
        }

        // Update parent pointers
        let parent_idx = self.node_mut(root_idx).parent;
        self.node_mut(child_idx).parent = parent_idx;

        if let Some(p_idx) = parent_idx {
            if self.node_mut(p_idx).left == Some(root_idx) {
                self.node_mut(p_idx).left = Some(child_idx);
            } else {
                self.node_mut(p_idx).right = Some(child_idx);
            }
        } else {
            self.root = Some(child_idx);
        }

        // Make old root the child of new root
        self.node_mut(child_idx).set_child(opposite(side), Some(root_idx));
        self.node_mut(root_idx).parent = Some(child_idx);

        // Update heights
        self.update_node_height(root_idx);
        self.update_node_height(child_idx);
    }


    /// Determines left-heavy (>0) or right-heavy (<0) balance factors for a given node index
    /// The necessity for restructure operations can be determined agnostically by
    /// `abs(balance_factor(index)) >= 2`
    fn balance_factor(&self, index: usize) -> isize {
        let node = &self.node(index);
        let left = node.left.map_or(0, |l| self.node(l).height as isize);
        let right = node.right.map_or(0, |r| self.node(r).height as isize);
        left - right
    }

    /// Rebalances the subtree rooted at `index`.
    /// Performs single or double rotations as necessary.
    fn restructure(&mut self, index: usize) {

        // Not all inserts require restructuring
        let balance = self.balance_factor(index);
        if balance.abs() < 2 {
            return;
        }

        // Determine heavy side
        let heavy_side = if balance > 1 { Side::Left } else { Side::Right };

        // Determine the child index for the heavy side
        let child_idx = match self.node(index).child(&heavy_side) {
            Some(idx) => idx,
            // SAFETY: A None value on the child violates the AVL invariant 
            None => panic!("Error: Heavy child is None"),
        };

        // Double rotation check
        //let child_balance = self.balance_factor(child_idx);
        //if heavy_side == Side::Left && child_balance < 0 {
        //    self.rotate(child_idx, &Side::Right);
        //} else if heavy_side == Side::Right && child_balance > 0 {
        //    self.rotate(child_idx, &Side::Left);
        //}
        match (&heavy_side, self.balance_factor(child_idx)) {
            (Side::Left, b) if b < 0 => self.rotate(child_idx, &Side::Right), // LR
            (Side::Right, b) if b > 0 => self.rotate(child_idx, &Side::Left), // RL
            _ => {}
        }

        // Single rotation on parent
        self.rotate(index, &heavy_side);
    }

    /// Produces a "snapshot" iterator over immutable references to the
    /// tree in its current state.
    pub fn iter(&self) -> InOrderIter<'_, T> {
        InOrderIter::new(&self.arena, self.root)
    }
}

pub struct InOrderIter<'a, T> {
    arena: &'a [Option<AVLNode<T>>],
    stack: Vec<usize>, // store indices, not references
    current: Option<usize>,
}
impl<'a, T> InOrderIter<'a, T> {
    fn new(arena: &'a [Option<AVLNode<T>>], root: Option<usize>) -> Self {
        Self {
            arena,
            stack: Vec::new(),
            current: root,
        }
    }
}
//impl<'a, T> Iterator for InOrderIter<'a, T> {
//    type Item = &'a T;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        // Uses a simplified inorder traversal with a stack
//        // to fetch elements in sorted order
//        while let Some(idx) = self.current {
//            self.stack.push(idx);
//            self.current = self.arena[idx].left;
//        }
//
//        if let Some(idx) = self.stack.pop() {
//            self.current = self.arena[idx].right;
//            self.arena[idx].value.as_ref() // convert Option<T> -> Option<&T>
//        } else {
//            None
//        }
//    }
//}
impl<'a, T> Iterator for InOrderIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(idx) = self.current {
            if let Some(node) = self.arena[idx].as_ref() {
                self.stack.push(idx);
                self.current = node.left;
                continue;
            } else {
                // Node was removed, skip
                self.current = None;
            }
        }

        if let Some(idx) = self.stack.pop() {
            if let Some(node) = self.arena[idx].as_ref() {
                self.current = node.right;
                node.value.as_ref()
            } else {
                // Skip removed node
                self.next()
            }
        } else {
            None
        }
    }
}

#[test]
fn avl_construction() {
    let mut tree: AVLTree<u8> = AVLTree::new();

    let v = [31, 13, 23, 39, 41, 43, 8, 17, 19];
    // Produces the following AVL tree
    // 
    //           39      
    //          /  \      
    //        17    41      
    //       /  \     \   
    //     13   23     43
    //     /   /  \           
    //    8   19  31          
    //
    for e in v.iter() {
        tree.insert(*e);
    }

    // Tests that the root is being updated properly
    assert_eq!(tree.get_root().unwrap(), &39);
    assert_eq!(tree.node(tree.root.expect("You fucked up")).value, Some(39));
    let root_node = &tree.node(tree.root.expect("nah, brah"));
    assert_eq!(tree.node(root_node.left.unwrap()).value, Some(17));
    assert_eq!(tree.node(root_node.right.unwrap()).value, Some(41));
    assert_eq!(tree.node(0).left, None);
    assert_eq!(tree.node(0).right, None);

    assert_eq!(tree.node(tree.node(7).left.unwrap()).value, Some(13));
    assert_eq!(tree.node(tree.node(7).right.unwrap()).value, Some(23));

    let mut sorted = Vec::new();
    for e in tree.iter() {
        sorted.push(*e) 
    }
    assert_eq!(sorted, [8, 13, 17, 19, 23, 31, 39, 41, 43]);

    let mut tree: AVLTree<u8> = AVLTree::new();
    let v = [1, 2, 3, 4, 5, 6, 7];
    // Produces the following AVL tree
    // 
    //          4
    //        /   \
    //      2       6
    //     / \     / \
    //    1   3   5   7
    //       
    for e in v.iter() {
        tree.insert(*e);
    }

    assert_eq!(tree.get_root().unwrap(), &4);
    assert_eq!(tree.node(tree.root.expect("You fucked up")).value, Some(4));
    let root_node = &tree.node(tree.root.expect("nah, brah"));
    assert_eq!(tree.node(root_node.left.unwrap()).value, Some(2));
    assert_eq!(tree.node(root_node.right.unwrap()).value, Some(6));
    assert_eq!(tree.node(0).left, None);
    assert_eq!(tree.node(0).right, None);

    assert_eq!(tree.node(tree.node(5).left.unwrap()).value, Some(5));
    assert_eq!(tree.node(tree.node(5).right.unwrap()).value, Some(7));

    let mut sorted = Vec::new();
    for e in tree.iter() {
        sorted.push(*e) 
    }
    assert_eq!(sorted, [1, 2, 3, 4, 5, 6, 7]);

    // Print visualization/debug
    eprintln!("{tree:#?}");
    //panic!();
}

#[test]
fn avl_removals() {
    let mut tree: AVLTree<u8> = AVLTree::new();

    // Construct the following AVL tree
    // 
    //           39      
    //          /  \      
    //        17    41      
    //       /  \     \   
    //     13   23     43
    //     /   /  \           
    //    8   19  31          
    //
    let v = [31, 13, 23, 39, 41, 43, 8, 17, 19];
    for e in v.iter() {
        tree.insert(*e);
    }

    // Remove 31 which results in the following AVL tree
    // 
    //           39      
    //          /  \      
    //        17    41      
    //       /  \     \   
    //     13   23     43
    //     /   /           
    //    8   19            
    //
    assert_eq!(tree.get_root().unwrap(), &39);
    assert!(tree.contains(&31));
    let removed = tree.remove(&31).unwrap();
    assert_eq!(removed, 31);
    assert!(!tree.contains(&31));

    assert_eq!(tree.node(tree.node(2).left.expect("")).value, Some(19));
    assert_eq!(tree.node(2).right, None);

    // Remove 41 which results in the following AVL tree
    // 
    //         17
    //        /  \
    //      13    39
    //     /     /  \
    //    8     23   43
    //          /
    //        19
    //
    assert!(tree.contains(&41));
    let removed = tree.remove(&41).unwrap();
    assert_eq!(removed, 41);
    assert!(tree.remove(&41).is_none()); // Test that 41 was really removed
    assert!(!tree.contains(&41));

    // 39 now has L 23 and R 43
    assert_eq!(tree.node(tree.node(3).left.expect("")).value, Some(23));
    assert_eq!(tree.node(tree.node(3).right.expect("")).value, Some(43));

    // 17 is now rooth with L 13 and R 39 
    assert_eq!(tree.get_root().unwrap(), &17);
    assert_eq!(tree.node(tree.root.expect("You fucked up")).value, Some(17));
    assert_eq!(tree.node(tree.node(7).left.expect("")).value, Some(13));
    assert_eq!(tree.node(tree.node(7).right.expect("")).value, Some(39));
    // The old root 39 now has L 23 and R 43
    assert_eq!(tree.node(tree.node(3).left.expect("")).value, Some(23));
    assert_eq!(tree.node(tree.node(3).right.expect("")).value, Some(43));

    // Remove 8 which results in the following AVL tree
    // 
    //         23
    //        /  \
    //      17    39
    //     /  \     \
    //    13   19    43
    //
    assert!(tree.contains(&8));
    let removed = tree.remove(&8).unwrap();
    assert_eq!(removed, 8);
    assert!(tree.remove(&8).is_none()); // Test that 8 was really removed
    assert!(!tree.contains(&8));

    // 23 is now rooth with L 17 and R 39
    assert_eq!(tree.get_root().unwrap(), &23);
    assert_eq!(tree.node(tree.root.expect("You fucked up")).value, Some(23));
    assert_eq!(tree.node(tree.node(2).left.expect("")).value, Some(17));
    assert_eq!(tree.node(tree.node(2).right.expect("")).value, Some(39));
    // The old root 17 now has L 13 and R 19
    assert_eq!(tree.node(tree.node(7).left.expect("")).value, Some(13));
    assert_eq!(tree.node(tree.node(7).right.expect("")).value, Some(19));

    // Print visualization/debug
    eprintln!("{tree:#?}");
    //panic!();
}
