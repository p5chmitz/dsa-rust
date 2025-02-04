use crate::trees::traits::{BinaryTree, Tree};

/** Owned non-null pointer to Node; Functions as a position */
type Pos<T> = Box<Node<T>>;

/** Represents a proper binary tree if left and right are Some */
#[derive(Clone, PartialEq)]
pub struct Node<T: std::cmp::PartialEq> {
    parent: Option<Pos<T>>,
    left: Option<Pos<T>>,
    right: Option<Pos<T>>,
    data: Option<T>,
}

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
    fn set_root(&mut self, data: T) {
        self.root.data = Some(data)
    }

    fn add_left(&mut self, mut ancestor: Pos<T>, node: Pos<T>) {
        ancestor.left = Some(node);
    }
    fn add_right(&mut self, mut ancestor: Pos<T>, node: Pos<T>) {
        ancestor.right = Some(node);
    }

    fn set(&mut self, _p: Pos<T>, _node: Node<T>) {}
    fn attach(&mut self, _left: Pos<T>, _right: Pos<T>) {}
    fn remove(&mut self, _p: Pos<T>) {}
}
// NOTE: Requires the PartialEq trait bounds for binary tree operations
//impl<T> Tree<Pos<T>, T> for BinTree<T>
impl<T> Tree<T> for BinTree<T>
where
    T: Clone + std::cmp::PartialEq,
{
    type Position = Pos<T>;

    /** Returns an immutable reference to the node's data */
    fn get<'a>(&'a self, node: &'a Self::Position) -> Option<&'a T> {
        if node.data.is_some() {
            node.data.as_ref()
        } else {
            None
        }
    }

    // /** Returns the number of nodes in the tree */
    // fn size(&self) -> usize {
    //     self.size
    // }
    // /** Returns true if the tree contains no nodes */
    // fn is_empty(&self) -> bool {
    //     //if self.size > 0 { false } else { true }
    //     self.size() == 0
    // }

    // Ancestor methods
    ///////////////////

    // /** Returns an immutable reference to the root of the tree */
    // fn root(&self) -> Option<&Pos<T>> {
    //     Some(&self.root)
    // }

    /** Returns an immutable reference to the parent of the given node */
    fn parent(&self, node: Self::Position) -> Option<Self::Position> {
        //if self.validate(&node) {
        //    node.unwrap().parent
        //} else { None }
        if node.parent.is_some() {
            node.parent
        } else {
            None
        }
    }

    // Descendant methods
    ///////////////////

    fn num_children(&self, node: Self::Position) -> Option<usize> {
        if let Some(n) = self.children(node) {
            Some(n.len())
        } else {
            None
        }
    }

    fn children(&self, node: Self::Position) -> Option<Vec<Self::Position>> {
        let mut vec = Vec::with_capacity(2);
        if let Some(l) = node.left {
            vec.push(l)
        };
        if let Some(r) = node.right {
            vec.push(r)
        };
        Some(vec)
    }

    // Query methods
    ////////////////

    fn is_leaf(&self, node: Self::Position) -> bool {
        //if self.num_children(node.clone()).is_some()
        //    && (*node.clone().unwrap()).left.is_none()
        //    && (*node.clone().unwrap()).right.is_none() {
        //    true
        //} else { false }
        node.left.is_none() && node.right.is_none()
    }

    fn is_root(&self, node: Self::Position) -> bool {
        node == self.root
    }

    // Derived methods
    //////////////////

    fn depth(&self, node: Self::Position) -> Option<usize> {
        let mut d = 1;
        let mut cursor = node.clone();
        while !self.is_root(cursor.clone()) {
            cursor = self.parent(cursor.clone()).unwrap();
            d += 1;
        }
        Some(d)
    }

    fn height(&self, node: Self::Position) -> Option<usize> {
        let mut h = 0;
        if let Some(c) = self.children(node) {
            for p in c {
                h = std::cmp::max(h, 1 + self.height(p).unwrap())
            }
            Some(h)
        } else {
            None
        }
    }
}
impl<T> BinaryTree<T> for BinTree<T>
where
    T: Clone + std::cmp::PartialEq,
{
    type Position = Pos<T>;

    fn left(&self, node: Self::Position) -> Option<Self::Position> {
        if node.left.is_some() {
            node.left
        } else {
            None
        }
    }

    fn right(&self, node: Self::Position) -> Option<Self::Position> {
        if node.right.is_some() {
            node.right
        } else {
            None
        }
    }

    fn sibling(&self, node: Self::Position) -> Option<Self::Position> {
        //if node.parent.is_some() { // Validate that the argument's parent exists
        //    let p = node.clone().parent;
        //    if p.clone().unwrap().left == Some(node) {
        //        return p.unwrap().right; // If the argument is left, return right
        //    } else {
        //        return p.unwrap().left; // If the argument is left, return right
        //    }
        //} else { None }
        if let Some(ref parent) = node.parent {
            // Checks that node has a parent
            // Checks if the argument is left, if so, return right and vice versa
            if parent.left.as_ref() == Some(&node) {
                parent.right.clone() // Return the right sibling
            } else {
                parent.left.clone() // Return the left sibling
            }
        } else {
            None // Return None if the node has no parent
        }
    }
}
