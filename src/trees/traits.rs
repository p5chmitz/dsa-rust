// Traits for to make-a da tree
///////////////////////////////

/** Defines the a basic Tree ADT where P is a position, and T is a type */
pub trait Tree<P, T> {

    // Fundamental operations
    /////////////////////////

    /** Returns the node's data */
    //fn get<'a>(&self, node: &'a T) -> &'a T;

    /** Returns the number of nodes in the tree */
    fn size(&self) -> usize;

    /** Returns true if the tree contains no nodes */
    fn is_empty(&self) -> bool;

    // Ancestor methods
    ///////////////////

    /** Returns an immutable reference to the root of the tree */
    fn root(&self) -> Option<&P>;

    /** Returns an immutable reference to the parent of the given node */
    //fn parent(&self, node: &P) -> Result<&P, String>;
    //fn parent<'a>(&self, node: &'a P) -> Result<&'a P, String>;
    fn parent<'a>(&self, node: &'a P) -> Option<&'a P>;

    /** Returns an iterator over immutable references to the node's children */
    //fn children<'a>(&self, node: &'a P) -> Box<dyn Iterator<Item = &T>>;
    fn children<'a>(&self, node: &'a P) -> Vec<&'a P>; // Returns an iterABLE for now

    // Query methods
    ////////////////

    /** Returns true if the specified position is the tree's root */
    fn is_root(&self, node: &P) -> bool;

    /** Returns true if the specified position is external */
    fn is_leaf(&self, node: &P) -> bool;

    // Derived operations
    /////////////////////

    fn depth(&self, node: &P) -> u32;

    //fn height(&self, node: &T) -> usize;
    fn height(&self, node: &P) -> usize;

    /** Returns the immediate number of children for a given node */
    fn num_children(&self, node: &P) -> usize;

}

/** Abstract (partial implementation) for a general tree type */
//pub trait AbstractTree<T: std::cmp::PartialEq>: Tree<T> {
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
//    /** Returns true if the tree is empty */
//    fn is_empty(&self) -> bool {
//        self.size() == 0
//    }
//
//    /** Default implementation of is_leaf() using num_children from Tree */
//    fn is_leaf(&self, node: &T) -> bool {
//        self.num_children(node) == 0
//    }
//
//    /** Recursive algorithm that returns the depth of an input node */
//    fn depth(&self, node: &T) -> u32 {
//        if AbstractTree::is_root(self, node) {
//            0
//        } else {
//            1 + self.depth(node)
//        }
//    }
//
//    /** Calculates the height of a given sub-tree based on an input position */
//    fn height(&self, node: &T) -> usize {
//        let mut h = 0;
//        for p in Tree::children(self, node) {
//            h = std::cmp::max(h, 1 + self.height(p))
//        }
//        h
//    }
//}

/** Abstract interface definition for a binary tree type */
//pub trait BinaryTree<T: std::cmp::PartialEq>: Tree<T> {
pub trait BinaryTree<P, T> 
where 
    T: std::cmp::PartialEq
{
    /** Returns the position of the left child of a given node */
    //fn left(&self, node: &P) -> Option<&P>;
    fn left<'a>(&self, node: &'a P) -> Option<&'a P>;

    /** Returns the position of the right child of a given node */
    //fn right(&self, node: &P) -> Option<&P>;
    fn right<'a>(&self, node: &'a P) -> Option<&'a P>;

    /** Returns the position of the sibling of a given node */
    fn sibling<'a>(&self, node: &'a P) -> Option<&'a P>;
}

// /** Abstract (partial implementation) for a binary tree */
//pub trait AbstractBinaryTree<T, P>: AbstractTree<T> + BinaryTree<T> 
//where 
//    T: std::cmp::PartialEq,
//    P: AsRef<T>
//{
//    fn sibling(&self, node: &T) -> Option<&T> {
//        if let Ok(parent) = Tree::parent(self, node) {
//            if let Some(l) = BinaryTree::left(self, parent) {
//                if parent == l {
//                    BinaryTree::right(self, parent)
//                } else {
//                    BinaryTree::left(self, parent)
//                }
//            // The node has no sibling
//            } else {
//                None
//            } 
//        // Parent is empty for some reason
//        } else {
//            None
//        } 
//    }
//    fn num_children(&self, node: &T) -> usize {
//        let mut count = 0;
//        if BinaryTree::left(self, node).is_some() {
//            count += 1
//        };
//        if BinaryTree::right(self, node).is_some() {
//            count += 1
//        };
//        count
//    }
//    /** Required method: Returns an iterator over all children in the sub-tree */
//    fn children(&self, node: &T) -> Box<dyn Iterator<Item = &T>>;
//}
