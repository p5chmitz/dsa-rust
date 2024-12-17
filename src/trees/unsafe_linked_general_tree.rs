/////////////////////////////
/** An unsafe general tree */
/////////////////////////////

use crate::trees::traits::Tree;
use std::ptr;

// Struct for parsing headings, this is the T, baby
#[derive(Clone, Debug)]
pub struct Heading {
    level: usize,
    title: String,
}
impl Heading {
    /** For building placeholder nodes */
    fn new_root(level: usize) -> Heading {
        Heading {
            level,
            title: "ROOT".to_string(),
        }
    }
}

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
    /** Builds a new Node and returns its position */
    fn build(data: Option<T>) -> Pos<T> {
        let node = Box::new(Node {
            parent: None,
            children: Vec::new(),
            data,
        });

        // Return a raw pointer to the node
        let node_ptr: *mut Node<T> = Box::into_raw(node);
        Some(node_ptr)
    }

    /** Gets an immutable reference to the data at a position */
    fn get<'a>(position: &'a Pos<T>) -> Option<&'a T> {
        unsafe {
            if let Some(p) = position {
                (*(*p)).data.as_ref()
            } else {
                None
            }
        }
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
    // Implementation specific methods
    //////////////////////////////////

    // /** Instantiates a new GenTree with an empty root */
    //fn new() -> GenTree<T> {
    //    let root: Pos<T> = Node::build(None); // Placeholder
    //    GenTree {
    //       root,
    //       size: 0,
    //    }
    //}
    fn new() -> GenTree<Heading> {
        let data = Heading::new_root(0);
        let root: Pos<Heading> = Node::build(Some(data)); // Placeholder
        GenTree { root, size: 0 }
    }

    // NOTE: Adapt this to replace root
    /** Adds a new root to the tree */
    fn set_root(&mut self, node: Pos<T>) {
        self.root = node;
    }

    /** Adds a child to a parent's child arena Vec<Pos<T>> */
    fn add_child(&mut self, ancestor: Pos<T>, node: Pos<T>) {
        unsafe {
            if let Some(p) = ancestor {
                // Adds the position to the parents arena
                (*p).children.push(node);

                // Links the node's parent Pos<T> to the correct ancestor
                if let Some(n) = node {
                    (*n).parent = ancestor;
                }
            }
            self.size += 1;
        }
    }

    // Fundamental methods
    //////////////////////

    /** Returns an immutable reference to the node's data type */
    fn get(&self, node: Pos<T>) -> Option<&T> {
        // Imperative approach
        if let Some(n) = node {
            unsafe { (*n).data.as_ref() }
        } else {
            None
        }
        // Functional approach
        //node.as_ref().and_then(|n| unsafe { (*(*n)).data.as_ref() })
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
    fn root(&self) -> Pos<T> {
        self.root
    }

    /** Returns an immutable reference to the parent of the given node */
    fn parent(&self, node: Pos<T>) -> Pos<T> {
        unsafe {
            if let Some(n) = node {
                (*n).parent
            } else {
                None
            }
        }
    }

    // Descendant methods
    /////////////////////

    // NOTE: Do you actually need this?
    /** Returns the number of children for a given node */
    fn num_children(&self, node: Pos<T>) -> usize {
        unsafe {
            if let Some(c) = node {
                (*c).children.len()
            } else {
                0
            }
        }
    }

    /** Returns an iterator over immutable references to the node's children */
    //TODO: Make this iterable into an iterator
    fn children(&self, node: Pos<T>) -> Option<&Vec<Pos<T>>> {
        unsafe {
            if let Some(c) = node {
                Some(&(*c).children.as_ref())
            } else {
                None
            }
        }
    }

    // Query methods
    ////////////////

    /** Returns true if the specified position is the tree's root */
    fn is_root(&self, node: &Pos<T>) -> bool {
        *node == self.root
        //std::ptr::eq(node, &self.root)
        //self.root.as_ref().map_or(false, |root| std::ptr::eq(node, *root))
    }

    // Derived methods
    //////////////////

    /** Default implementation of is_leaf() using num_children from Tree */
    fn is_leaf(&self, node: &Pos<T>) -> bool {
        self.num_children(*node) == 0
    }

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

    /** Traverse the tree, printing each node's title and child arena */
    pub fn preorder_proof(position: &Option<*mut Node<Heading>>) {
        unsafe {
            // Checks that the position (node) exists
            if let Some(p) = position {
                // Visit the node at the referenced position
                let node: Option<&Heading> = Node::get(&position);
                if let Some(t) = node {
                    // Examples for retrieving data
                    //let data: Option<&Heading> = (*(*p)).data.as_ref();
                    // NOTE: Pos<T> is Option<*mut Node<Heading>>
                    let children: &Vec<Pos<Heading>> = (*(*p)).children.as_ref();
                    println!("\t{}\n\t\tChildren: {:?}", t.title, children);
                    // Recursively visit each child
                    for e in children {
                        GenTree::<T>::preorder_proof(e);
                    }
                } else {
                    println!("No Heading at position");
                }
            } else {
                println!("Not a valid position")
            }
            // preorder(p):
            // "visit" p
            // for each child c in children(p) do
            //    preorder(c)
        }
    }

    pub fn pretty_print(_name: &str, position: &Option<*mut Node<Heading>>) {
        //println!("\t[] {name}\n\t│");
        GenTree::<Heading>::preorder(position, "");
        println!("");
    }

    /** Traverse the tree, printing each node's title and child arena */
    pub fn preorder(position: &Option<*mut Node<Heading>>, prefix: &str) {
        // Checks that the position (node) exists
        if let Some(p) = position {
            // Visit the node at the referenced position
            let children: &Vec<Pos<Heading>> = unsafe { (*(*p)).children.as_ref() };
            let mut index = children.len();

            // Recursively visit each child
            for e in children {
                let node = Node::get(&e).unwrap();
                index -= 1;
                if index == 0 {
                    println!("\t{}└── {}", prefix, node.title);
                    GenTree::<T>::preorder(e, &format!("{}    ", prefix));
                } else {
                    println!("\t{}├── {}", prefix, node.title);
                    GenTree::<T>::preorder(e, &format!("{}│   ", prefix));
                }
            }
        } else {
            println!("Not a valid position")
        }
    }
} 

// Impelmentation-specific behaviors
////////////////////////////////////
//
// These elements cover construction and traversal

/** Print-debugging function */
pub fn print_node(position: Pos<Heading>) {
    unsafe {
        if let Some(p) = position {
            let parent: Option<*mut Node<Heading>> = (*p).parent;
            let data: Option<&Heading> = (*p).data.as_ref();
            let children: &Vec<Option<*mut Node<Heading>>> = (*p).children.as_ref();
            println!(
                "Node:\n\tParent: {:?}\n\tData: {:?}\n\tChildren: {:?}",
                parent, data, children
            );
        } else {
            println!(": Something's missing...")
        }
    }
}

/** Constructs a tree of Heading types */
pub fn construct_heading_tree(data: Vec<Heading>) -> GenTree<Heading> {
    // Instantiates a GenTree with a generic root
    let mut tree: GenTree<Heading> = GenTree::<Heading>::new();

    let mut curr_level = data[0].level; // Heading always starts at H2
    let mut current_parent: Pos<Heading> = tree.root;
    let mut last: Pos<Heading> = tree.root;

    // Constructs tree from Vec<T>
    for e in data {
        // Creates a position from a cloned T
        let node: Pos<Heading> = Node::build(Some(e.clone()));

        // Case: Adds a child and adjusts the level accordingly
        // TODO: Add PLACEHOLDER nodes for non-sequential skips
        if e.level > curr_level {
            let diff = e.level - curr_level;
            // If there is a generational difference,
            // sets the parent to the most recently added node
            // and increments the level
            for _ in 0..diff {
                //println!("Down a gen...");
                current_parent = last;
                curr_level += 1;
            }
            //print!("Adding new child or sibling ");
            tree.add_child(current_parent, node);
            //print_node(tree.parent(node));
        }
        // Adds ancestor ("pibling", grandparent, etc.)
        // according to its level;
        else if e.level <= curr_level {
            let diff = curr_level - e.level;
            for _ in 0..diff {
                //println!("Up a gen...");
                current_parent = tree.parent(tree.parent(last)); // Grandparent
                curr_level -= 1;
            }
            //print!("Adding new ancestor ");
            tree.add_child(current_parent, node);
            //print_node(tree.parent(node));
        }
        // Updates the last addition
        last = node;
    }
    tree
}

pub fn parser(root: &Path) {
    use std::io::{self, BufRead};

    use regex::Regex;
    use std::fs::File;
    use std::path::Path;

    use crate::trees::unsafe_linked_general_tree::{GenTree, Heading};

    // Regex for capturing the title from front matter
    // NOTE: I dont use H1s, so the regex only catches H2s and above
    let t = Regex::new(r"(?ms)^---.*?^title:\s*(.+?)\s*$.*?^---").unwrap();
    let mut doc_title = String::new();
    // Regex for capturing headings
    let h = Regex::new(r"^(#{2,6})\s+(.*)").unwrap();
    let mut headings: Vec<Heading> = Vec::new();

    // Read input
    //let file_path = std::path::Path::new("./src/trees/mock_data.md");
    let file_path = root;
    let file = File::open(file_path).unwrap(); // TODO: Fix lazy error handling
    let reader = io::BufReader::new(file);

    // Read the entire file into a single string
    let content: String = reader
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    // Extract the document title
    if let Some(captures) = t.captures(&content) {
        let title = captures.get(1).unwrap().as_str();
        doc_title.push_str(title);
    }

    // Parse headings line by line
    for line in content.lines() {
        if let Some(captures) = h.captures(line) {
            let level = captures.get(1).unwrap().as_str().len();
            let text = captures.get(2).unwrap().as_str().to_string();
            headings.push(Heading { level, title: text });
        }
    }

    let tree = construct_heading_tree(headings);

    GenTree::<Heading>::pretty_print(&doc_title, &tree.root);
}

use std::path::Path;

/** Walks the source dir recursively creating a String for each MD document */
pub fn example(root: &Path) {

    if root.is_dir() {
        for e in root.read_dir().expect("read_dir call failed") {
            let entry = e.expect("failure to deconstruct value");
            example(&entry.path());
        }
    } else if root.is_file() {
        println!("{}", root.display());

        if let Some(name) = root.file_name() {
            println!("📄 {}", name.to_string_lossy());
        }

        parser(root);
        
    }
}
