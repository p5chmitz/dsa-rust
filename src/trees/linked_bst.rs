use crate::trees::traits::{BinaryTree, Tree};

/** Owned, smart pointer to a Node; Functions as a position */
type Pos<T> = Box<Node<T>>;

/** Represents a proper binary tree if left and right are Some */
#[derive(PartialEq)]
pub struct Node<T: std::cmp::PartialEq> {
    parent: Option<Pos<T>>,
    left: Option<Pos<T>>,
    right: Option<Pos<T>>,
    data: Option<T>,
}
/** The BinTree struct represents a binary tree structure with a root node, a
reference to the root node, and the structure's size.

BinTree's fundamental implementation includes the following functions:
- new() -> BinTree<T>
- add_root(&mut self, _node: Node<T>)
- add_left(&mut self, _p: Pos<T>, _node: Node<T>)
- add_right(&mut self, _p: Pos<T>, _node: Node<T>)
- set(&mut self, _p: Pos<T>, _node: Node<T>)
- attach(&mut self, _left: Pos<T>, _right: Pos<T>)
- remove(&mut self, _p: Pos<T>)

BinTree implements the Tree trait which provides the following functions:
- get<'a>(&self, node: &'a Pos<T>) -> Option<&'a T>
- size(&self) -> usize
- is_empty(&self) -> bool
- root(&self) -> Option<&Pos<T>>
- parent<'a>(&self, node: &'a Pos<T>) -> Option<&'a Pos<T>>
- num_children(&self, node: &Pos<T>) -> usize
- children<'a>(&self, node: &'a Pos<T>) -> Vec<&'a Pos<T>>
- is_leaf(&self, node: &Pos<T>) -> bool
- is_root(&self, node: &Pos<T>) -> bool
- depth(&self, node: &Pos<T>) -> u32
- height(&self, node: &Pos<T>) -> usize

The Binary Tree implements the BinaryTree trait which provides the following functions:
- left<'a>(&self, node: &'a Pos<T>) -> Option<&'a Pos<T>>
- right<'a>(&self, node: &'a Pos<T>) -> Option<&'a Pos<T>>
- sibling<'a>(&self, node: &'a Pos<T>) -> Option<&'a Pos<T>> */
pub struct BinTree<T: std::cmp::PartialEq> {
    root: Box<Node<T>>, // Needs Option for empty trees
    size: usize,
}
impl<T> BinTree<T>
where
    T: std::cmp::PartialEq,
{
    // Creates a new char tree
    fn new() -> BinTree<T> {
        let node: Node<T> = Node {
            parent: None,
            left: None,
            right: None,
            data: None,
        };
        BinTree {
            root: Box::from(node),
            size: 0,
        }
    }

    // All operations can (and should) require O(1) time
    fn add_root(&mut self, _node: Node<T>) {}
    fn add_left(&mut self, _p: Pos<T>, _node: Node<T>) {}
    fn add_right(&mut self, _p: Pos<T>, _node: Node<T>) {}
    fn set(&mut self, _p: Pos<T>, _node: Node<T>) {}
    fn attach(&mut self, _left: Pos<T>, _right: Pos<T>) {}
    fn remove(&mut self, _p: Pos<T>) {}
}
// NOTE: Requires the PartialEq trait bounds for binary tree operations
impl<T> Tree<Pos<T>, T> for BinTree<T>
where
    T: std::cmp::PartialEq,
{
    // Fundamental methods
    //////////////////////

    /** Returns an immutable reference to the node's data */
    fn get<'a>(&self, node: &'a Pos<T>) -> Option<&'a T> {
        //if let Some(d) = &node.data {
        //    Some(d)
        //} else { None }
        node.data.as_ref()
    }

    /** Returns the number of nodes in the tree */
    fn size(&self) -> usize {
        self.size
    }
    /** Returns true if the tree contains no nodes */
    fn is_empty(&self) -> bool {
        //if self.size > 0 { false } else { true }
        self.size() == 0
    }

    // Ancestor methods
    ///////////////////

    /** Returns an immutable reference to the root of the tree */
    fn root(&self) -> Option<&Pos<T>> {
        Some(&self.root)
    }

    /** Returns an immutable reference to the parent of the given node */
    //fn parent<'a>(&self, node: &'a Pos<T>) -> Result<&'a Pos<T>, String> {
    //    if self.is_root(node) {
    //        return Err("Error: The root node has no parent".to_string());
    //    }
    //    Ok(node.parent.as_ref().expect("Y U NO ROOT HAS PAREN?"))
    //        //node.parent
    //        //.as_ref()
    //        //.ok_or_else(|| "Error: Node has no parent".to_string())
    //}
    fn parent<'a>(&self, node: &'a Pos<T>) -> Option<&'a Pos<T>> {
        //if let Some(n) = node.parent.as_ref() {
        //    n.parent.as_ref()
        //} else {
        //    None
        //}
        node.parent.as_ref()?.parent.as_ref() // Propagates the None option with ?
    }

    // Descendant methods
    ///////////////////

    fn num_children(&self, node: &Pos<T>) -> usize {
        self.children(node).len()
    }

    /** Returns an iterator over immutable references to the node's children */
    //TODO: Make this iterable into an iterator
    //fn children<'a>(&self, node: &'a Pos<T>) -> Vec<&'a Pos<T>> {
    fn children<'a>(&self, node: &'a Pos<T>) -> Vec<&'a Pos<T>> {
        let mut vec = Vec::with_capacity(2);
        if let Some(left) = &node.left {
            vec.push(left)
        }
        if let Some(right) = &node.right {
            vec.push(right)
        }
        vec
    }

    // Query methods
    ////////////////

    /** Default implementation of is_leaf() using num_children from Tree */
    fn is_leaf(&self, node: &Pos<T>) -> bool {
        self.num_children(node) == 0
    }

    /** Returns true if the specified position is the tree's root */
    fn is_root(&self, node: &Pos<T>) -> bool {
        *node == self.root
    }

    // Derived methods
    //////////////////

    /** Recursive algorithm that returns the depth of an input node */
    fn depth(&self, node: &Pos<T>) -> u32 {
        if self.is_root(node) {
            0
        } else {
            1 + self.depth(node)
        }
    }

    /** Calculates the height of a given sub-tree based on an input position */
    fn height(&self, node: &Pos<T>) -> usize {
        let mut h = 0;
        for p in self.children(node) {
            h = std::cmp::max(h, 1 + self.height(p))
        }
        h
    }
}
impl<T> BinaryTree<Pos<T>, T> for BinTree<T>
where
    T: std::cmp::PartialEq,
{
    /** Returns the position of the left child of a given node */
    fn left<'a>(&self, node: &'a Pos<T>) -> Option<&'a Pos<T>> {
        node.left.as_ref()
        //node.and_then(|n| n.left)
    }

    /** Returns the position of the right child of a given node */
    fn right<'a>(&self, node: &'a Pos<T>) -> Option<&'a Pos<T>> {
        node.right.as_ref()
        //node.and_then(|n| n.right)
    }

    /** Returns the position of the sibling of a given node */
    fn sibling<'a>(&self, node: &'a Pos<T>) -> Option<&'a Pos<T>> {
        if let Some(parent_ref) = &node.parent {
            if parent_ref.left.as_ref() == Some(node) {
                return parent_ref.right.as_ref();
            } else {
                return parent_ref.right.as_ref();
            }
        } else {
            None
        } // else for original check
    }
}

// Original, flat implementation
//impl<T> BinTree<T>
//where
//    T: std::cmp::PartialEq
//{
//    // Fundamental methods
//    //////////////////////
//
//    /** Returns the number of nodes in the tree */
//    fn size(&self) -> usize {
//        self.size
//    }
//    /** Returns true if the tree contains no nodes */
//    fn is_empty(&self) -> bool {
//        //if self.size > 0 { false } else { true }
//        self.size() == 0
//    }
//
//    /** Returns the node's data */
//    fn get<'a>(&self, node: &'a Node<T>) -> &'a T {
//        &node.data
//    }
//
//    // Ancestor methods
//    ///////////////////
//
//    /** Returns an immutable reference to the root of the tree */
//    fn root(&self) -> Option<&T> {
//        self.root.as_ref().map(|node| &node.data)
//    }
//
//    /** Returns an immutable reference to the parent of the given node */
//    fn parent(&self, node: &T) -> Result<&T, String> {
//        if self.is_root(node) {
//            return Err("Error: The root node has no parent".to_string());
//        }
//        self.node
//            .as_ref()
//            .and_then(|node| node.parent.as_ref()) // Get the parent from Option<Box<Node<T>>>
//            .map(|parent| &parent.data)           // Extract the data from the parent
//            .ok_or_else(|| "Parent not found.".to_string()) // Convert to Result
//    }
//
//    // Descendant methods
//    ///////////////////
//
//    fn num_children(&self, node: Pos<T>) -> usize {
//        self.children(node).len()
//    }
//
//    fn children(&self, node: Pos<T>) -> Vec<Pos<T>> {
//        let mut vec = Vec::with_capacity(2);
//        if let Some(children) = node {
//            if let Some(l) = children.left {
//                vec.push(l)
//            }
//            if let Some(r) = children.right {
//                vec.push(r)
//            }
//        }
//        vec
//    }
//
//    // Query methods
//    ////////////////
//
//    /** Default implementation of is_leaf() using num_children from Tree */
//    fn is_leaf(&self, node: Pos<T>) -> bool {
//        self.num_children(node) == 0
//    }
//
//    /** Returns an immutable reference to the root of the tree */
//    fn is_root(&self, node: &T) -> bool {
//        if let Some(r) = self.root() {
//            // The == operator evaluates to bool
//            node == r
//        } else {
//            false
//        }
//    }
//
//    // Derived methods
//    //////////////////
//
//    /** Recursive algorithm that returns the depth of an input node */
//    fn depth(&self, node: &T) -> u32 {
//        if self.is_root(node) {
//            0
//        } else {
//            1 + self.depth(node)
//        }
//    }
//
//    /** Calculates the height of a given sub-tree based on an input position */
//    fn height(&self, node: Pos<T>) -> usize {
//        let mut h = 0;
//        for p in self.children(node) {
//            h = std::cmp::max(h, 1 + self.height(p))
//        }
//        h
//    }
//
//    // Specific type methods
//    ////////////////////////
//
//    // All operations can be implemented in O(1)
//    fn add_root(&mut self, node: Node<T>) {}
//    fn add_left(&mut self, position: Pos<T>, node: Node<T>) {}
//    fn add_right(&mut self, position: Pos<T>, node: Node<T>) {}
//    fn set(&mut self, position: Pos<T>, node: Node<T>) {}
//    fn attach(&mut self, left: Pos<T>, right: Pos<T>) {}
//    fn remove(&mut self, position: Pos<T>) {}
//
//}
