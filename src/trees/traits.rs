// Traits for to make-a da tree
///////////////////////////////

/** Defines the a basic Tree ADT where P is a position, and T is a type */
pub trait Tree<P, T>
where 
    T: std::cmp::PartialEq
{

    // Fundamental methods
    //////////////////////

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

    // Derived methods
    //////////////////

    fn depth(&self, node: &P) -> u32;

    //fn height(&self, node: &T) -> usize;
    fn height(&self, node: &P) -> usize;

    /** Returns the immediate number of children for a given node */
    fn num_children(&self, node: &P) -> usize;

}

/** Abstract interface definition for a binary tree type */
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
