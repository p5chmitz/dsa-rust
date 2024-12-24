// Traits for to make-a da tree
///////////////////////////////

/** Defines the a basic Tree ADT where P is a position, and T is a type */
pub trait ATree<P, T> {
    // Fundamental methods
    //////////////////////

    /** Returns the node's data */
    fn get<'a>(&self, node: &'a P) -> Option<&'a T>;

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
    //fn children<'a>(&self, node: &'a P) -> Vec<&'a P>; // Returns an iterABLE for now
    fn children<'a>(&self, node: &'a P) -> Vec<&'a P>;

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

pub trait Tree<T> {
    type Position;

    // Fundamental methods
    //////////////////////

    /** Returns an iterator over immutable references to the node's children */
    //TODO: Make this iterable into an iterator
    fn children(&self, node: Self::Position) -> Option<&Vec<Self::Position>>;

    /** Returns an immutable reference to the node's data type */
    fn get(&self, node: Self::Position) -> Option<&T>;

    /** Returns an immutable reference to the parent of the given node */
    fn parent(&self, node: Self::Position) -> Self::Position;

    // Query methods
    ////////////////

    /** Returns true if the specified position is the tree's root */
    fn is_root(&self, node: Self::Position) -> bool;

    /** Returns true if the specified position is external */
    fn is_leaf(&self, node: Self::Position) -> bool;

    // Derived methods
    //////////////////

    fn depth(&self, node: Self::Position) -> usize;

    //fn height(&self, node: &T) -> usize;
    fn height(&self, node: Self::Position) -> usize;

    /** Returns the immediate number of children for a given node */
    fn num_children(&self, node: Self::Position) -> usize;

}

/** Abstract interface definition for a general tree type */
pub trait GeneralTree<P, T>
where
    T: std::cmp::PartialEq,
{
}

/** Abstract interface definition for a binary tree type */
pub trait BinaryTree<P, T>
where
    T: std::cmp::PartialEq,
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
