/////////////////////////////
/** An unsafe general tree */
/////////////////////////////

use crate::trees::traits::Tree;
use std::ptr;

/** A position Pos is an optional raw pointer to a Node, generic over T */
type Pos<T> = Option<*mut Node<T>>;

/** Represents a general tree with a collection of children */
#[derive(PartialEq)]
pub struct Node<T> {
    parent: Pos<T>,
    children: Vec<Pos<T>>,
    pub data: Option<T>,
}
impl<T> Node<T> {
    fn build(data: Option<T>) -> Pos<T> {
        let mut node = Box::new(Node {
            parent: None,
            children: Vec::new(),
            data,
        });
        
        // Return a raw pointer to the node
        let node_ptr: *mut Node<T> = &mut *node;
        Some(node_ptr)
    }
}
/** The GenTree struct represents a general tree structure with a root node
and the structure's size. */
#[derive(Debug)]
pub struct GenTree<T> {
    root: Pos<T>, // Needs Option for empty trees
    size: usize,
}
impl<T> GenTree<T> {
    /** Instantiates a new GenTree with an empty root */
    fn new() -> GenTree<T> { 
        let root: Pos<T> = Node::build(None); // Placeholder
        GenTree {
           root,
           size: 0,
        }
    }

    /** Adds a new root to the tree */
    fn add_root(&mut self, node: Pos<T>) {
        self.root = node;
        self.size += 1;
    }

    /** Adds a child to a parent's child arena Vec<Pos<T>> */
    fn add_child(&mut self, parent: Pos<T>, node: Pos<T>) {
        unsafe { 
            if let Some(p) = parent {
                (*p).children.push(node)
            }
            self.size += 1;
        }
    }

    // Fundamental methods
    //////////////////////

    // Redesigning the Tree trait functions
    /** Returns an immutable reference to the node's data */
    fn get<'a>(&self, node: Pos<T>) -> Option<&T> {
        // Imperative approach
        if let Some(n) = node {
            unsafe { (*n).data.as_ref() } 
        } else { None }
        // Functional approach
        //node.as_ref().and_then(|n| unsafe { (*(*n)).data.as_ref() })
    }

    /** Returns an immutable reference to the node's data */
    //fn get<'a>(&self, node: &'a Pos<T>) -> Option<&'a T> {
    //    // Imperative approach
    //    //unsafe {
    //    //    match (*node.unwrap()).data.as_ref() { 
    //    //        Some(d) => Some(d),
    //    //        _ => None
    //    //    } 
    //    //}
    //    if let Some(n) = *node {
    //        unsafe { (*n).data.as_ref() } 
    //    } else { None }
    //    // Functional approach
    //    //node.as_ref().and_then(|n| unsafe { (*(*n)).data.as_ref() })
    //}

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
    fn root(&self) -> Pos<T> {
        self.root
    }

    /** Returns an immutable reference to the parent of the given node */
    fn parent(&self, node: Pos<T>) -> Pos<T> {
        unsafe { 
            if let Some(n) = node {
                (*n).parent
            } else { None }
        }
    }

    // Descendant methods
    ///////////////////

    fn num_children(&self, node: Pos<T>) -> usize {
        unsafe { 
            if let Some(c) = node {
                (*c).children.len()
            } else { 0 }
        }
    }

    /** Returns an iterator over immutable references to the node's children */
    //TODO: Make this iterable into an iterator
    fn children(&self, node: Pos<T>) -> Option<&Vec<Pos<T>>> {
        unsafe { 
            if let Some(c) = node {
                Some(&(*c).children.as_ref())
            } else { None }
        }
    }

    /** Returns an iterator over immutable references to the node's children */
    //TODO: Make this iterable into an iterator
    //fn children<'a>(&self, node: &'a Pos<T>) -> Vec<&'a Pos<T>> {
    //    // Creates a new collection with node-specifc references
    //    node.children.iter().collect() 
    //}

    // Query methods
    ////////////////

    /** Default implementation of is_leaf() using num_children from Tree */
    fn is_leaf(&self, node: &Pos<T>) -> bool {
        self.num_children(*node) == 0
    }

    /** Returns true if the specified position is the tree's root */
    fn is_root(&self, node: &Pos<T>) -> bool {
        *node == self.root
        //std::ptr::eq(node, &self.root)
        //self.root.as_ref().map_or(false, |root| std::ptr::eq(node, *root))
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

    // /** Calculates the height of a given sub-tree based on an input position */
    //fn height(&self, node: Pos<T>) -> usize {
    //    let mut h = 0;
    //    for p in self.children(node) {
    //        h = std::cmp::max(h, 1 + self.height(p))
    //    }
    //    h
    //}

}

// Struct for parsing headings
#[derive(Clone, Debug)]
pub struct Heading {
    level: usize,
    title: String,
}

pub fn construct_heading_tree(data: Vec<Heading>) -> GenTree<Heading> {
    unsafe {
        let mut tree: GenTree<Heading> = GenTree::new();
        let mut curr_level = data[0].level - 1; // Accommodates starts above H1
        let current_parent: Pos<Heading> = tree.root;
        for e in data {
            // Creates a node out of val
            let node: Pos<Heading> = Node::build(Some(e.clone()));
            // Adds children to the current node (as "root")
            if e.level > curr_level {
                let diff = e.level - curr_level;
                let mut descendant: Pos<Heading> = current_parent;
                for _ in 0..diff {
                    descendant = (*descendant.unwrap()).parent
                }
                tree.add_child(descendant, node);
                println!("Adding new child")
            } else if e.level == curr_level {
                tree.add_child(current_parent, node);
                println!("Adding new sibling")
            }
            // Adds and ancestor according to its level
            else if e.level < curr_level {
                // Traverses back up the tree
                let diff = curr_level - e.level;
                println!("Adding new ancestor x{diff}");
                let mut ancestor: Pos<Heading> = current_parent;
                for _ in 0..diff {
                    ancestor = (*ancestor.unwrap()).parent
                }
                tree.add_child(ancestor, node);
            }
            // Resets the current level
            curr_level = e.level;
        }
        tree
    }

}


pub fn example() {
    use std::io::{self, BufRead};

    use std::fs::File;
    use regex::Regex;
    use std::path::Path;

    use crate::trees::unsafe_linked_general_tree::{Heading, GenTree};

    // 1) Read and parse headings
    /////////////////////////////

    // 1.1) Input reader
    let file_path = std::path::Path::new("./src/trees/mock_data.md");
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
            headings.push(Heading { level, title});
        }
    }
    //println!("Debug print of input:\n{:#?}", headings);

    // 2) Construct the tree
    ////////////////////////

    let tree = construct_heading_tree(headings);
    println!("The tree: \n {:?}", tree);

    // 3) Traverse the tree
    ///////////////////////

    // TODO

}

//  Visual references for algorithm construction
//  [
//    lvl: 2, title: "H2"
//    lvl: 3, title: "H3"
//    lvl: 3, title: "H3"
//    lvl: 2, title: "H2"
//    lvl: 3, title: "H3"
//    lvl: 4, title: "H4"
//    lvl: 3, title: "H3"
//  ]
//
//
//  [] Lorem Ipsum Test 
//  │    An ordered look at MD parsing
//  │
//  ├── H2
//  │   ├── H3
//  │   └── H3
//  └── H2
//      ├── H3
//      │   └── H4
//      └── H3
//
//
//       Lorem Ipsum Test
//              |
//         ------------
//         |          |
//        H2         H2
//         |          |
//     -------      -------
//     |     |      |     |
//    H3    H3      H3    H3
//           |
//           H4
//
