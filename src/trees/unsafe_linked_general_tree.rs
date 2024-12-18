/////////////////////////////
/** An unsafe general tree */
/////////////////////////////

use crate::trees::traits::Tree;

use std::fs::File; // Used by parser()
use std::io::{self, BufRead, BufReader}; // Used by parser()
use std::path::Path; // Used by example()
use std::ptr;

use regex::Regex; // Used by parser()

/** Used for parsing Markdown headings; Heading is T
 - fn new_root(level: usize) -> Heading
*/
#[derive(Clone, Debug)]
struct Heading {
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

/** Represents a general tree with a collection of children 
 - fn build(data: Option<T>) -> Pos<T> {
 - fn get<'a>(position: &'a Pos<T>) -> Option<&'a T>
*/
#[derive(PartialEq)]
struct Node<T> {
    parent: Pos<T>,
    children: Vec<Pos<T>>,
    data: Option<T>,
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
        if let Some(p) = position {
            unsafe { (*(*p)).data.as_ref() }
        } else {
            None
        }
    }
}
/** The GenTree struct represents a positional, linked-based general 
tree structure with a root node that contains a single raw pointer 
to the root node and the structure's size. 
The genericity of the struct means you'll have to explicitly 
type associated functions.

Example:
```
    let path = std::path::Path::new("~/Developer/project/src/doc");
    let parsed = GenTree::<Heading>::parser(path);
    let tree = GenTree::<Heading>::construct_heading_tree(parsed.1);
    GenTree::<Heading>::preorder_proof(&tree.root);
```

Methods:
 - fn set_root(&mut self, node: Pos<T>)
 - fn add_child(&mut self, ancestor: Pos<T>, node: Pos<T>)
 - fn get(&self, node: Pos<T>) -> Option<&T>
 - fn size(&self) -> usize
 - fn is_empty(&self) -> bool
 - fn root(&self) -> Pos<T>
 - fn parent(&self, node: Pos<T>) -> Pos<T>
 - fn num_children(&self, node: Pos<T>) -> usize
 - fn children(&self, node: Pos<T>) -> Option<&Vec<Pos<T>>>
 - fn is_root(&self, node: &Pos<T>) -> bool
 - fn is_leaf(&self, node: &Pos<T>) -> bool 
 - fn depth(&self, node: &Pos<T>) -> u32
 
Associated Functions:
 - fn new() -> GenTree<Heading>
 - fn print_node(position: Pos<Heading>)
 - fn preorder_proof(position: &Pos<Heading>)
 - fn simple_print(headings: Vec<&Heading>)
 - fn parser(root: &Path) -> (String, Vec<Heading>)
 - fn construct_heading_tree(data: Vec<Heading>) -> GenTree<Heading>
 - fn pretty_print(_name: &str, position: &Pos<Heading>)
 - fn preorder(position: &Pos<Heading>, prefix: &str)
 - fn navigator(path: &Path)
*/
#[derive(Debug)]
struct GenTree<T> {
    root: Pos<T>, // Needs Option for empty trees
    size: usize,
}
impl<T> GenTree<T> {

    // TODO: Adapt this to replace root
    /** Adds a new root to the tree */
    //fn set_root(&mut self, node: Pos<T>) {
    //    self.root = node;
    //}

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

    /** Returns the number of nodes in the tree */
    //fn size(&self) -> usize {
    //    self.size
    //}

    /** Returns true if the tree contains no nodes */
    //fn is_empty(&self) -> bool {
    //    //if self.size > 0 { false } else { true }
    //    self.size() == 0
    //}

    /** Returns an immutable reference to the root of the tree */
    //fn root(&self) -> Pos<T> {
    //    self.root
    //}

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

    // Ancestor methods
    ///////////////////

    /** Returns an immutable reference to the parent of the given node */
    fn parent(&self, node: Pos<T>) -> Pos<T> {
        if let Some(n) = node {
            unsafe { (*n).parent }
        } else {
            None
        }
    }

    // Descendant methods
    /////////////////////

    // NOTE: Do you actually need this?
    /** Returns the number of children for a given node */
    fn num_children(&self, node: Pos<T>) -> usize {
        if let Some(c) = node {
            unsafe { (*c).children.len() }
        } else {
            0
        }
    }

    /** Returns an iterator over immutable references to the node's children */
    //TODO: Make this iterable into an iterator
    //fn children(&self, node: Pos<T>) -> Option<&Vec<Pos<T>>> {
    //    if let Some(c) = node {
    //        unsafe { Some(&(*c).children.as_ref()) }
    //    } else {
    //        None
    //    }
    //}

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
    //fn is_leaf(&self, node: &Pos<T>) -> bool {
    //    self.num_children(*node) == 0
    //}

    /** Recursive algorithm that returns the depth of an input node */
    //fn depth(&self, node: &Pos<T>) -> u32 {
    //    if self.is_root(node) {
    //        0
    //    } else {
    //        1 + self.depth(node)
    //    }
    //}

    // /** Calculates the height of a given sub-tree based on an input position */
    //fn height(&self, node: Pos<T>) -> usize {
    //    let mut h = 0;
    //    for p in self.children(node) {
    //        h = std::cmp::max(h, 1 + self.height(p))
    //    }
    //    h
    //}



    // Associated and utility functions
    ///////////////////////////////////

    /** Instantiates a new GenTree with a default root */
    fn new() -> GenTree<Heading> {
        let data = Heading::new_root(0);
        let root: Pos<Heading> = Node::build(Some(data)); // Placeholder
        GenTree { root, size: 0 }
    }

    /** Print-debugging function */
    fn print_node(position: Pos<Heading>) {
        unsafe {
            if let Some(p) = position {
                let parent: Pos<Heading> = (*p).parent;
                let data: Option<&Heading> = (*p).data.as_ref();
                let children: &Vec<Pos<Heading>> = (*p).children.as_ref();
                println!(
                    "Node:\n\tParent: {:?}\n\tData: {:?}\n\tChildren: {:?}",
                    parent, data, children
                );
            } else {
                println!(": Something's missing...")
            }
        }
    }

    /** POC for preorder traversal */
    fn preorder_proof(position: &Pos<Heading>) {
        // Checks that the position (node) exists
        if let Some(p) = position {
            // Visit the node at the referenced position
            let node: Option<&Heading> = Node::get(&position);
            if let Some(_) = node {
                let children: &Vec<Pos<Heading>> = unsafe { (*(*p)).children.as_ref() }; 
                // Recursively visit each child
                for e in children {
                    Self::print_node(*e);
                    GenTree::<T>::preorder_proof(e);
                }
            } else {
                println!("No Heading at position");
            }
        } else {
            println!("Not a valid position")
        }
    }

    /** A dirt-simple tree printing algorithm based on leveled elements */
    fn simple_print(headings: Vec<&Heading>) {
        let mut result = String::new();
        for e in headings {
            let space = "   ".repeat(e.level - 1);
            let line = format!("{}{}\n", space, e.title);
            result.push_str(&line);
        }
    }
 
    /** Takes a path to a Markdown file, parses it for title and headings,
and returns a tuple containing the document title and a vector of 
headings.

Note: The document title portion of the tuple is specifically 
designed for the Astro-formatted frontmatter of each MD document. 
The navigator() used in the final example omits this field. */
    fn parser(root: &Path) -> (String, Vec<Heading>) {
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
        let file = std::fs::File::open(file_path).unwrap(); // TODO: Fix lazy error handling
        let reader = BufReader::new(file);
    
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

        (doc_title, headings)
    }
    
    /** Constructs a tree of Heading types */
    fn construct_heading_tree(data: Vec<Heading>) -> GenTree<Heading> {
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

    fn pretty_print(_name: &str, position: &Pos<Heading>) {
        //println!("\t[] {name}\n\t│");
        GenTree::<Heading>::preorder(position, "");
        println!("");
    }

    /** Traverse the tree recursively, printing each node's title and 
children */
    fn preorder(position: &Pos<Heading>, prefix: &str) {
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

    /** This function chains the module's utility functions to pretty-print 
a table of contents for each Markdown file in the specified directory */
    fn navigator(path: &Path) {
        // 1) Walks the root path recursively, passing file paths to the parser
        if path.is_dir() {
            for e in path.read_dir().expect("read_dir call failed") {
                let entry = e.expect("failure to deconstruct value");
                Self::navigator(&entry.path()); // Recursive call
            }
        } else if path.is_file() {
            println!("{}", path.display());
        
            if let Some(name) = path.file_name() {
                println!("📄 {}", name.to_string_lossy());
            }
            let parsed = GenTree::<Heading>::parser(path);
            let tree = GenTree::<Heading>::construct_heading_tree(parsed.1);
            GenTree::<Heading>::pretty_print(&parsed.0, &tree.root);
        }
    }

}


/** Putting it all together */
pub fn example() {
    use crate::trees::unsafe_linked_general_tree::{GenTree, Heading};

    // Both example opertaion chains use the same mock data path
    let path = std::path::Path::new("src/trees/mock_data.md");

    // Proof of concept
    // 1) Parses the file and returns a tuple containing
    // - 0: The doc title
    // - 1: A list of headings and their values
    let parsed = GenTree::<Heading>::parser(path);

    // 2) Constructs the tree
    let tree = GenTree::<Heading>::construct_heading_tree(parsed.1);

    // 3) Takes a doc title and a tree root;
    // Traverses the tree and prints each node's raw data
    GenTree::<Heading>::preorder_proof(&tree.root);
    println!("");

    // Does the same thing as the above three steps, but adds the ability to 
    // traverse a directory structure recursively and a pretty-printer
    // with proper box drawing components
    GenTree::<Heading>::navigator(path);

}
