////////////////////////////////////////////////////////////
/** An earnest, but faulty attempt at a safe general tree */
////////////////////////////////////////////////////////////
use crate::trees::traits::Tree;

/** Owned, smart pointer to a Node; Functions as a position */
type Pos<T> = Box<Node<T>>;

/** Represents a general tree with a collection of children */
#[derive(PartialEq)]
pub struct Node<T> {
    parent: Option<Pos<T>>,
    children: Vec<Pos<T>>,
    data: Option<T>,
}
impl<T> Node<T> {
    fn build(data: Option<T>) -> Pos<T> {
        Box::new(Node {
            parent: None,
            children: Vec::new(),
            data,
        })
    }
}
/** The GenTree struct represents a general tree structure with a root node
and the structure's size. */
pub struct GenTree<T> {
    root: Option<Pos<T>>, // Needs Option for empty trees
    size: usize,
}
impl<T> GenTree<T> {
    pub fn new() -> GenTree<T> {
        let node: Node<T> = Node {
            parent: None,
            children: Vec::new(),
            data: None,
        };
        GenTree {
            root: Some(Box::from(node)),
            size: 0,
        }
    }

    // All operations can (and should) require O(1) time
    fn add_parent(&mut self, _parent: Node<T>, _node: Node<T>) {}

    fn add_child(&mut self, _node: Pos<T>) {}

    fn set(&mut self, _p: Pos<T>, _data: T) {}
    fn remove(&mut self, _p: Pos<T>) {}
}
impl<T> Tree<Pos<T>, T> for GenTree<T> {
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
        self.root.as_ref()
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
    fn children<'a>(&self, node: &'a Pos<T>) -> Vec<&'a Pos<T>> {
        // Creates a new collection with node-specifc references
        node.children.iter().collect()
    }

    // Query methods
    ////////////////

    /** Default implementation of is_leaf() using num_children from Tree */
    fn is_leaf(&self, node: &Pos<T>) -> bool {
        self.num_children(node) == 0
    }

    /** Returns true if the specified position is the tree's root */
    fn is_root(&self, node: &Pos<T>) -> bool {
        //*node == self.root
        //std::ptr::eq(node, &self.root)
        self.root
            .as_ref()
            .map_or(false, |root| std::ptr::eq(node, root))
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

//pub fn example(file_path: &str) -> io::Result<String> {
pub fn example(file_path: &str) {
    use std::io::{self, BufRead};

    use crate::trees::linked_general_tree::GenTree;
    use regex::Regex;
    use std::fs::File;
    use std::path::Path;

    // 1) Read and parse headings
    /////////////////////////////

    // Struct for parsing headings
    #[derive(Debug)]
    struct Heading {
        level: usize,
        title: String,
    }

    // 1.1) Input reader
    let file = File::open(file_path).unwrap(); // TODO: Lazy error handling
    let reader = io::BufReader::new(file);

    // 1.2) Parse each line for a header using regex,
    // push heading as level and title to a vec
    // NOTE: I dont use H1s, so the regex only catches H2s and above
    let re = Regex::new(r"^(#{2,6})\s+(.*)").unwrap();
    let mut headings: Vec<Heading> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap(); // TODO: address lazy error handling
        if let Some(captures) = re.captures(&line) {
            let level = captures.get(1).unwrap().as_str().len();
            let title = captures.get(2).unwrap().as_str().to_string();
            headings.push(Heading { level, title });
        }
    }

    // 2) Construct the tree
    ////////////////////////

    // 2.1) Instantiate the tree
    let tree: GenTree<&Heading> = GenTree::new();
    // 2.2) Add nodes by level
    let mut _level = 0;
    let mut _current_parent: &Option<Pos<&Heading>> = &tree.root;
    //for (i, val) in headings.iter().enumerate() {
    //    // Creates a node out of val
    //    let node: Pos<&Heading> = Node::build(Some(val));
    //    // Sets the first heading as the root
    //    if i == 0 {
    //        tree.root = Some(node);
    //        current_parent = &tree.root;
    //    }
    //    // Adds children to the root
    //    else if val.level > level {
    //        tree.add_child(node)
    //    }
    //    // Adds parent sibling
    //    else if val.level < level {
    //        // Cant access ancestors if there aren't any
    //        if tree.is_root(&current_parent.unwrap()) {
    //            panic!("Cant access ancestors if there aren't any!");
    //        }
    //        // Resets the level
    //        let parent = &current_parent.as_ref().unwrap().parent;
    //        tree.add_child(node);
    //    }
    //    // Resets the current level
    //    level = val.level;
    //}

    // 3) Traverse the tree
    ///////////////////////
}

// Visual reference for algorithm construction
// [
//    Heading { level: 2, title: "Subtitle With Spaces" },
//    Heading { level: 3, title: "Another Subtitle" },
//    Heading { level: 3, title: "Second H3" },
//    Heading { level: 2, title: "Back up to H2" },
//    Heading { level: 3, title: "This H2 Has an H3 too" },
//    Heading { level: 4, title: "This is an H4" },
//    Heading { level: 3, title: "Final H3" }
//]
//
//[] Lorem Ipsum Test
//│    An ordered look at MD parsing
//│
//├── Subtitle With Spaces
//│   ├── Another Subtitle
//│   └── Second H3
//└── Back up to H2
//    ├── This H2 Has an H3 too
//    │   └── This is an H4
//    └── Final H3
//
//
//                   Lorem Ipsum Test
//                          |
//                 -------------------
//                 |                 |
//     H2         One               Two
//                 |                 |
//             ----------       -----------
//             |        |       |         |
//     H3  Another   Again    Bother   Brothel
//                     |
//     H4           Castrate
