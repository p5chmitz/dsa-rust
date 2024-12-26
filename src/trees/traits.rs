// Traits for to make-a da tree
///////////////////////////////

/** Defines the a basic Tree ADT where P is a position, and T is a type */
pub trait Tree<T> {
    type Position;

    // Fundamental methods
    //////////////////////

    /** Returns an iterable collection over the node's children 

    NOTE: To make this iterable into an iterator simply call something like
    self.children(node).into_iter() */
    fn children(&self, node: Self::Position) -> Option<Vec<Self::Position>>;

    /** Returns an immutable reference to the node's data type */
    fn get<'a>(&'a self, node: &'a Self::Position) -> Option<&'a T>;

    /** Returns the position of a node's parent, if it exists */
    fn parent(&self, node: Self::Position) -> Option<Self::Position>;

    // Maybe?
    //fn size(&self) -> usize;
    //fn is_empty(&self) -> bool;

    // Query methods
    ////////////////

    /** Returns true if the specified position is the tree's root */
    fn is_root(&self, node: Self::Position) -> bool;

    /** Returns true if the specified position is external */
    fn is_leaf(&self, node: Self::Position) -> bool;

    // Derived methods
    //////////////////

    fn depth(&self, node: Self::Position) -> Option<usize>;

    //fn height(&self, node: &T) -> usize;
    fn height(&self, node: Self::Position) -> Option<usize>;

    //NOTE: Is this really needed?
    /** Returns the immediate number of children for a given node */
    fn num_children(&self, node: Self::Position) -> Option<usize>;

    //fn validate(&self, node: &Self::Position) -> bool;

}

/** Abstract interface definition for a binary tree type */
pub trait BinaryTree<T>
where
    T: Clone + std::cmp::PartialEq,
{
    type Position;

    /** Returns the position of the left child of a given node */
    fn left(&self, node: Self::Position) -> Option<Self::Position>;

    /** Returns the position of the right child of a given node */
    fn right(&self, node: Self::Position) -> Option<Self::Position>;

    /** Returns the position of the sibling of a given node */
    fn sibling(&self, node: Self::Position) -> Option<Self::Position>;
}
