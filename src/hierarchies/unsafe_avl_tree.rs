/*! An unsafe AVL tree

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

use std::borrow::Borrow;
use std::cmp::{max, Ordering};

// Custom sum type for search algorithm
enum SearchResult<T> {
    // The tree is empty (and uninitialized)
    None,
    // Index of a found key
    Exists(Link<T>),
    // Index for insertion of a new key
    Parent(Link<T>),
    //Parent { index: usize, side: Side },
}

//#[derive(PartialEq)]
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
fn opposite(side: &Side) -> &Side {
    match side {
        Side::Left => &Side::Right,
        Side::Right => &Side::Left,
    }
}

use std::fmt;
use std::ptr::NonNull;

type Link<T> = Option<NonNull<AVLNode<T>>>;

/// NOTE: T must implement fmt::Debug
pub struct AVLNode<T> {
    value: T,        // A node must have a value to exist
    parent: Link<T>, // None indicates the root of the tree
    left: Link<T>,
    right: Link<T>,
    height: usize,
}
impl<T: fmt::Debug> fmt::Debug for AVLNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("AVLNode");

        dbg.field("value", &self.value);

        match self.left {
            Some(ptr) => dbg.field("left", &format_args!("{:p}", ptr)),
            None => dbg.field("left", &"None"),
        };

        match self.right {
            Some(ptr) => dbg.field("right", &format_args!("{:p}", ptr)),
            None => dbg.field("right", &"None"),
        };

        match self.parent {
            Some(ptr) => dbg.field("parent", &format_args!("{:p}", ptr)),
            None => dbg.field("parent", &"None"),
        };

        dbg.field("height", &self.height);

        dbg.finish()
    }
}
impl<T> AVLNode<T> {
    // Creates a new node with its current index, a value, and its parent (if Some).
    // Guarantees that all nodes have a value.
    // All initial inserts are leafs before restructuring, so left and right are set to None.
    fn new(value: T, parent: Link<T>, height: usize) -> Self {
        AVLNode {
            value,
            parent,
            left: None,
            right: None,
            height,
        }
    }

    ///// Returns a reference to the node's value, if Some
    ///// NOTE: Used in public API with self.root()
    //fn get_value<'a>(link: Link<T>) -> Option<&'a T> {
    //    match link {
    //        Some(val) => Some(unsafe { &(*val.as_ptr()).value }),
    //        None => None,
    //    }
    //}

    /// Returns the L/R child AVLNode, if Some
    /// NOTE: Link<T> == Option<NonNull<AVLNode<T>>>
    fn child(&self, side: &Side) -> &Link<T> {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    // Returns the L/R child AVLNode, if Some
    fn child_mut(&mut self, side: &Side) -> &mut Link<T> {
        match side {
            Side::Right => &mut self.right,
            Side::Left => &mut self.left,
        }
    } 


    /// Set child index for a given side
    fn set_child(&mut self, side: &Side, link: Link<T>) {
        match side {
            Side::Left => self.left = link,
            Side::Right => self.right = link,
        }
    }

    // Adds 1 to the max value of a node's sub-tree heights
    //
    // SAFETY: NonNull is only dereferenced in the presence
    // of Some
    fn update_height(&mut self) {
        unsafe {
            let left = match self.left {
                Some(ptr) => (*ptr.as_ptr()).height,
                None => 0,
            };
            let right = match self.right {
                Some(ptr) => (*ptr.as_ptr()).height,
                None => 0,
            };
            self.height = 1 + max(left, right);
        }
    }

    /// Determines left-heavy (>0) or right-heavy (<0) balance factors 
    /// for a given sub-tree. The necessity for restructure 
    /// operations can be determined agnostically by
    /// `abs(balance_factor(index)) >= 2`
    ///
    /// None indicates a leaf node with no sub-tree
    fn balance_factor(&self) -> isize {
        let left = match self.left {
            Some(val) => (unsafe { (*val.as_ref()).height } as isize),
            None => 0,
        };
        let right = match self.right {
            Some(val) => (unsafe { (*val.as_ref()).height } as isize),
            None => 0,
        };
        left - right
    }

    /// Rotate the subtree rooted at `root_idx` in the given direction.
    /// `side` is the direction of the original heavy side 
    /// as Side::Left or Side::Right.
    fn rotate(&mut self, side: &Side) {
        // Heavy child becomes the new root of the subtree
        let child_ptr = *self.child(side);

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
        self.node_mut(child_idx)
            .set_child(opposite(side), Some(root_idx));
        self.node_mut(root_idx).parent = Some(child_idx);

        // Update heights
        parent.update_node_height();
        child.update_node_height();
    }

    // Tri-node restructuring is the heart of the AVL tree
    //
    // Tri-node restructuring wraps a single rotation function
    // that updates height and also checks that the parent is also balanced
    //
    /// Rebalances the subtree rooted at `index`.
    /// Performs single or double rotations as necessary.
    fn restructure(&mut self, index: usize) {
        // Determine if the insertion requires balancing,
        // if so, determine which side is heavy
        let balance: isize = self.balance_factor();
        if balance.abs() < 2 {
            return;
        }
        let heavy_side = if balance > 1 { Side::Left } else { Side::Right };

        // Determine the child index/handle for the heavy side
        //let child_idx = match self.node(index).child(&heavy_side) {
        //    Some(idx) => idx,
        //    // SAFETY: A None value on the child violates the AVL invariant
        //    None => panic!("Error: Heavy child is None"),
        //};


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


}

/// # About
///
/// See the [module-level documentation](crate::hierarchies::avl_tree) for more information.
#[derive(Debug)]
pub struct AVLTree<T> {
    root: Link<T>, // None indicates an empty tree
    size: usize,
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
            root: None,
            size: 0,
        }
    }

    // /// Creates a new, empty binary search tree with a given (growable) initial capacity.
    // pub fn new_with_capacity(size: usize) -> Self {
    //     AVLTree {
    //         arena: Vec::with_capacity(size),
    //         root: Some(0),
    //     }
    // }

    ///// Immutable node accessor
    //fn node(&self, index: usize) -> &AVLNode<T> {
    //    self.arena[index]
    //        .as_ref()
    //        .expect("Error: Invalid immutable node access")
    //}

    ///// Mutable node accessor
    //fn node_mut(&mut self, index: usize) -> &mut AVLNode<T> {
    //    self.arena[index]
    //        .as_mut()
    //        .expect("Error: Invalid mutable node access")
    //}

    /// Gets a reference to the value associated with a provided key,
    /// if it exists in the tree.
    pub fn get_value<Q>(&self, key: &Q) -> Option<&T>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        // SAFETY: Dereferencing SearchResults::Exists
        // is guaranteed to contain a value.
        unsafe {
            match self.search(key) {
                SearchResult::Exists(current) => Some(&(*current.unwrap().as_ptr()).value),
                _ => None,
            }
        }
    }

    pub fn get_root(&self) -> Option<&T> {
        match self.root {
            Some(val) => Some(unsafe { &(*val.as_ptr()).value }),
            None => None,
        }
    }

    /// Returns a `SearchResult` enum with the following variants:
    /// - None: Indicates an empty tree
    /// - Parent: The key is not in the tree, but can be inserted at the parent index value
    /// - Exists: The key was found in the tree; The caller can decide how to use this index
    ///   to deal with multi-maps and sets
    fn search<'a, Q>(&self, match_key: &Q) -> SearchResult<T>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        // Early return for empty structures
        if self.root.is_none() {
            return SearchResult::None;
        };

        // Sets the starting point for the search
        let mut current = self.root;

        // Uses iterative loop instead of recursive search
        // because fuck stack overflows (and recursion)
        //
        // The match arms indicate whether the search key is
        // greater/less than the current node's value.
        // If the search key is less than the current, go left,
        // otherwise go right. If the chosen branch does not match
        // and has no children in the intended path, return the
        // current node as the parent.
        // SAFETY:
        unsafe {
            while let Some(node_ptr) = current {
                let node: &AVLNode<T> = &*node_ptr.as_ptr();
                let node_key: &Q = node.value.borrow();
                current = match node_key.cmp(match_key) {
                    Ordering::Equal => {
                        return SearchResult::Exists(current);
                    }
                    Ordering::Less => match node.left {
                        Some(left) => Some(left),
                        None => return SearchResult::Parent(current),
                    },
                    Ordering::Greater => match node.right {
                        Some(right) => Some(right),
                        None => return SearchResult::Parent(current),
                    },
                }
            }
            // Should be unreachable, but here to
            // keep the compiler happy
            SearchResult::None
        }
    }

    /// Returns true if the key exists in the tree.
    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        matches!(self.search(key), SearchResult::Exists(_))
    }

    /// Inserts the given key into the tree maintaining an AVL
    /// structure.
    pub fn insert(&mut self, key: T)
    where
        T: Ord,
    {
        // match on three SearchResult scenarios:
        // 1) Parent(_): The search found no key match and returned
        //    the appropriate insertion point (parent).
        // 2) None: The search determined that the tree is empty,
        //    insert as root.
        // 3) Exists: The search determined that the key already
        //    exists, current no-op.
        match self.search(&key) {
            // Determine the side to insert on:
            // If key < node == left, otherwise right
            //
            // SAFETY:
            // `val` is a valid pointer to a live node in the tree.
            // The tree's invariants guarantee that writing to one
            // of its child pointers is valid.
            SearchResult::Parent(parent) => {
                if let Some(val) = parent {
                    unsafe {
                        // Attempts to move the node out of the allocation
                        // which causes serious correctness and soundness issues
                        //let node: AVLNode<T> = *val.as_ptr();
                        // Valid, but misleading semantics with AXM because the next block writes
                        // to the shared reference
                        //let node: &AVLNode<T> = &*val.as_ptr();
                        // Valid and semantically correct with hard-fought AXM instincts
                        let node: &mut AVLNode<T> = &mut *val.as_ptr();
                        let new_node =
                            NonNull::new(Box::into_raw(Box::new(AVLNode::new(key, parent, 1))));
                        if key < node.value {
                            node.left = new_node;
                        } else {
                            node.right = new_node;
                        }
                    }
                };
                // Walk up the tree to update heights and rebalance
                while let Some(link) = parent {
                    self.update_node_height(link);
                    self.restructure(link);
                    unsafe {
                        // Ensures that the loop stops at the root
                        parent = match parent {
                            Some(val) => Some(val),
                            None => None
                        }
                    }
                }
            }
            SearchResult::None => {
                let new_node = AVLNode::new(key, None, 1);
                let wrapped: Link<T> = NonNull::new(Box::into_raw(Box::new(new_node)));
                self.root = wrapped;
            }
            SearchResult::Exists(_) => {}
        }
    }

    ///// Removes and returns an element from the AVL tree as an owned value.
    //pub fn remove<Q>(&mut self, key: &Q) -> Option<T>
    //where
    //    Q: Ord + ?Sized,
    //    T: Borrow<Q>,
    //{
    //    //let target_index = match self.search(key.key()) {
    //    let target_index = match self.search(key) {
    //        SearchResult::Exists(idx) => idx,
    //        _ => return None,
    //    };

    //    // Step 1: Find node to physically remove (node with ≤1 child)
    //    let mut remove_index = target_index;
    //    if self.node(remove_index).left.is_some() && self.node(remove_index).right.is_some() {
    //        // Node has two children: find in-order successor
    //        let mut succ_index = self.node(remove_index).right.unwrap();
    //        while let Some(left) = self.node(succ_index).left {
    //            succ_index = left;
    //        }

    //        // Move successor's value into target node
    //        //let succ_value = self.arena[succ_index].take().unwrap().value;
    //        //self.node_mut(remove_index).value = succ_value;

    //        // 1. Take the value out of the successor node.
    //        let succ_value = self.node_mut(succ_index).value.take();

    //        // 2. Replace the target's value with the successor's,
    //        // getting the original target value back.
    //        let original_value = self
    //            .node_mut(target_index)
    //            .value
    //            .replace(succ_value.unwrap());

    //        // 3. Place the original target value into the successor node,
    //        // which we are about to remove.
    //        self.node_mut(succ_index).value = original_value;

    //        // Now, the successor node can be safely removed, and it
    //        // contains the correct value to return.

    //        // Now remove the successor node (guaranteed ≤1 child)
    //        remove_index = succ_index;
    //    }

    //    // Step 2: Identify the child of the node to remove (if any)
    //    let child_index = self
    //        .node(remove_index)
    //        .left
    //        .or(self.node(remove_index).right);

    //    // Step 3: Update parent to point to the child
    //    let parent_index = self.node(remove_index).parent;
    //    if let Some(p_idx) = parent_index {
    //        let parent = self.node_mut(p_idx);
    //        if parent.left == Some(remove_index) {
    //            parent.left = child_index;
    //        } else {
    //            parent.right = child_index;
    //        }
    //    } else {
    //        // Removing root
    //        self.root = child_index;
    //    }

    //    // Step 4: Update child's parent
    //    if let Some(c_idx) = child_index {
    //        self.node_mut(c_idx).parent = parent_index;
    //    }

    //    // Step 5: Take the node for return
    //    let removed_value = self.arena[remove_index].take().map(|n| n.value);

    //    // Step 6: Walk up ancestors to update heights and rebalance
    //    let mut current = parent_index;
    //    while let Some(idx) = current {
    //        self.update_node_height(idx);
    //        self.restructure(idx);
    //        current = self.node(idx).parent;
    //    }

    //    removed_value?
    //}

    // Utility functions
    ////////////////////

    ///// Updates the height of an arbitrary node in an AVL tree
    ///// where leaf nodes are defined as having height 1
    //fn update_node_height(&mut self, index: usize) {
    //    let left = self
    //        .node_mut(index)
    //        .left
    //        .map_or(0, |idx| self.node_mut(idx).height);
    //    let right = self
    //        .node_mut(index)
    //        .right
    //        .map_or(0, |idx| self.node_mut(idx).height);
    //    // Works for internal and leaf nodes, because max(0, 0) + 1 = 1
    //    self.node_mut(index).height = max(left, right) + 1
    //}
    //fn update_height(&mut self) {
    //    self.height = 1 + max(self.left, self.right)
    //}

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
}
