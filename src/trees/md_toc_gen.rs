//////////////////////////////////////////////
/** An attempt at a simple MD TOC generator */
//////////////////////////////////////////////

use crate::trees::traits::Tree;

use std::fs::File; // Used by parse_file()
use std::io::{self, BufRead, BufReader}; // Used by parse_file()
use std::path::Path; // Used by example()
use std::ptr;

use regex::Regex; // Used by parse_file()

/** Used for parsing Markdown headings; Heading is T */
#[derive(Clone, Debug)]
struct Heading {
    level: usize,
    title: String,
}
impl Heading {
    /** Just a humble Heading builder */
    fn new(title: String, level: usize) -> Heading {
        Heading { level, title }
    }

    /** For building placeholder nodes */
    fn new_root(level: usize) -> Heading {
        Heading {
            level,
            title: "ROOT".to_string(),
        }
    }
}

/** A position in the tree as raw pointer to a Node, generic over T */
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
    fn build(data: Option<T>) -> Box<Node<T>> {
        Box::new(Node {
            parent: None,
            children: Vec::new(),
            data,
        })

        // Return a raw pointer to the node
        //let node_ptr: *mut Node<T> = Box::new(node));
        //Some(node_ptr)
    }

    /** Gets an immutable reference to the data at a position */
    fn get<'a>(position: Pos<T>) -> Option<&'a T> {
        if let Some(p) = position {
            unsafe { (*p).data.as_ref() }
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
    let parsed = GenTree::<Heading>::parse_file(path);
    let tree = GenTree::<Heading>::construct(parsed.1);
    GenTree::<Heading>::preorder_proof(&tree.root);
```

Methods:
 - fn add_child(&mut self, ancestor: Pos<T>, node: Pos<T>)
 - fn children(&self, node: Pos<Heading>) -> Option<&Vec<Pos<Heading>>>
 - fn get(&self, node: Pos<Heading>) -> Option<&Heading>
 - fn parent(&self, node: Pos<Heading>) -> Pos<Heading>
 - fn is_root(&self, node: &Pos<T>) -> bool

Associated & Utility Functions:
 - fn new() -> GenTree<Heading>
 - fn simple_print(title: &String, headings: &Vec<Heading>)
 - fn parse_file(root: &Path) -> (String, Vec<Heading>)
 - fn construct(data: &Vec<Heading>) -> GenTree<Heading>
 - fn pretty_print(name: &str, position: &Pos<Heading>)
 - fn preorder(position: &Pos<Heading>, prefix: &str)
 - fn navigator(path: &Path)
*/
#[derive(Debug)]
struct GenTree<T> {
    root: Pos<T>, // Needs Option for empty trees
    size: usize,
}
impl<T> GenTree<T> {
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

    /** Returns an iterator over immutable references to the node's children */
    //TODO: Make this iterable into an iterator
    fn children(&self, node: Pos<Heading>) -> Option<&Vec<Pos<Heading>>> {
        if let Some(c) = node {
            unsafe { Some(&(*c).children.as_ref()) }
        } else {
            None
        }
    }

    /** Returns an immutable reference to the node's data type */
    fn get(&self, node: Pos<Heading>) -> Option<&Heading> {
        // Imperative approach
        if let Some(n) = node {
            unsafe { (*n).data.as_ref() }
        } else {
            None
        }
        // Functional approach
        //node.as_ref().and_then(|n| unsafe { (*(*n)).data.as_ref() })
    }

    /** Returns an immutable reference to the parent of the given node */
    fn parent(&self, node: Pos<Heading>) -> Pos<Heading> {
        if let Some(n) = node {
            unsafe { (*n).parent }
        } else {
            None
        }
    }

    /** Returns true if the specified position is the tree's root */
    fn is_root(&self, node: &Pos<T>) -> bool {
        *node == self.root
        //std::ptr::eq(node, &self.root)
        //self.root.as_ref().map_or(false, |root| std::ptr::eq(node, *root))
    }

    // Associated and utility functions
    ///////////////////////////////////

    /** Instantiates a new GenTree with a default root */
    fn new() -> GenTree<Heading> {
        let data = crate::trees::md_toc_gen::Heading::new_root(0);
        let root: Pos<Heading> = Some(Box::into_raw(Node::build(Some(data)))); // Placeholder
        GenTree { root, size: 0 }
    }

    /** A dirt-simple tree printing algorithm based on leveled elements */
    fn simple_print(title: &String, headings: &Vec<Heading>) {
        println!("📄 {}\n   │", title);
        for e in headings {
            let space = "   ".repeat(e.level - 1);
            let line = format!("   │{}{}", space, e.title);
            println!("{line}")
        }
        println!("   │\n")
    }

    /** Takes a path to a Markdown file, parses it for title and headings,
    and returns a tuple containing the document title and a vector of
    headings.

    Note: The document title portion of the tuple is specifically
    designed for the Astro-formatted frontmatter of each MD document. */
    fn parse_file(root: &Path) -> (String, Vec<Heading>) {
        // Regex for capturing the title from front matter
        let t = Regex::new(r"(?ms)^---.*?^title:\s*(.+?)\s*$.*?^---").unwrap();
        let mut doc_title = String::new();
        // Regex for capturing headings H1-H6 as #-######
        let h = Regex::new(r"^(#{1,6})\s+(.*)").unwrap();
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
    fn construct(data: &Vec<Heading>) -> GenTree<Heading> {
        // Instantiates a GenTree with a generic root and traversal positioning
        let mut tree: GenTree<Heading> = GenTree::<T>::new();
        // TODO: Make this a dynamic argument
        let mut level_cursor = 1; // Astro content starts at H2, skipping H1 
        let mut position_cursor: Pos<Heading> = tree.root;

        // Constructs tree from Vec<T>
        for e in data {
            // Creates a position from a cloned list entry
            let node: Pos<Heading> = Some(Box::into_raw(Node::build(Some(e.clone()))));

            // Case: Adds a child to the current parent and sets level cursor
            if e.level == level_cursor + 1 {
                tree.add_child(position_cursor, node);
                let data = tree.get(node).unwrap();
                level_cursor = data.level;
            }
            // Case: Adds a child with multi-generational skips with empty nodes
            else if e.level > level_cursor + 1 {
                let diff = e.level - level_cursor;
                for _ in 1..diff {
                    let heading = Heading::new("[]".to_string(), 0);
                    let placeholder: Pos<Heading> = Some(Box::into_raw(Node::build(Some(heading))));
                    tree.add_child(position_cursor, placeholder);
                    position_cursor = placeholder;
                    level_cursor += 1;
                }
                tree.add_child(position_cursor, node);
                let data = tree.get(node).unwrap();
                level_cursor = data.level;
            }
            // Case: Adds sibling to current parent
            else if e.level == level_cursor {
                tree.add_child(tree.parent(position_cursor), node);
            }
            // Case: Adds a child to the appropriate ancestor,
            // ensuring proper generational skips
            else {
                let diff = level_cursor - e.level;
                position_cursor = tree.parent(position_cursor);
                for _ in 0..diff {
                    position_cursor = tree.parent(position_cursor);
                    level_cursor -= 1;
                }
                tree.add_child(position_cursor, node);
                let data = tree.get(node).unwrap();
                level_cursor = data.level;
            }

            // Updates the most recent addition
            position_cursor = node;
        }
        tree
    }

    /** Serves as a wrapper for the traversal function that actually prints data */
    fn pretty_print(name: &str, position: &Pos<Heading>) {
        //println!("\t[] {name}\n\t│");
        println!("📄 {}\n\t│", name);
        GenTree::<T>::preorder(position, "");
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
                let node = Node::get(*e).unwrap();
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
        // 1) Walks the root path recursively, passing file paths to the parse
        if path.is_dir() {
            for e in path.read_dir().expect("read_dir call failed") {
                let entry = e.expect("failure to deconstruct value");
                Self::navigator(&entry.path()); // Recursive call
            }
        } else if path.is_file() {
            println!("{}", path.display());
            let parsed = GenTree::<T>::parse_file(path);
            let tree = GenTree::<T>::construct(&parsed.1);
            GenTree::<T>::pretty_print(&parsed.0, &tree.root);
        }
    }
}
impl<T> Drop for GenTree<T> {
    fn drop(&mut self) {
        /** Recursive tree destructor */
        // TODO: Update implementation with NonNull
        // to avoid null pointer dereference check
        unsafe fn drop_node_recursive<T>(node_ptr: *mut Node<T>) {
            // Avoids a null pointer dereference
            if node_ptr.is_null() {
                return;
            }

            // Dereference the pointer and process its children
            let node = &mut *node_ptr;
            for &child_ptr in node.children.iter() {
                if let Some(child_ptr) = child_ptr {
                    drop_node_recursive(child_ptr);
                }
            }

            // Deallocate the current node
            let _ = Box::from_raw(node_ptr);
        }

        unsafe {
            if let Some(root_ptr) = self.root {
                drop_node_recursive(root_ptr);
            }
        }
    }
}

#[test]
fn basic_function_test() {
    unsafe {
        use crate::trees::md_toc_gen::Heading;

        // Creates a tree with a default ROOT node
        let mut tree: GenTree<Heading> = GenTree::<Heading>::new();
        if let Some(r) = tree.root {
            let h: Heading = (*r).data.clone().unwrap();
            assert_eq!(&h.title, "ROOT");
        }

        // Builds a Heading that simulates an H2, converts it to a Node,
        // and finally converts it to a position Pos<Heading> as raw pointer "a"
        let h2 = Heading::new("H2".to_string(), 2);
        let node_a: Box<Node<Heading>> = Node::build(Some(h2));
        let node_a_ptr: Pos<Heading> = Some(Box::into_raw(node_a));

        // Adds a to root
        tree.add_child(tree.root, node_a_ptr);

        // Checks that add_child() assigns correct parent for the node
        assert_eq!(tree.root, tree.parent(node_a_ptr));
        // Checks that the parent (ROOT) has exactly one child as the "a" node
        assert_eq!(tree.children(tree.root), Some(&vec![node_a_ptr]));
        // Checks that the ROOT's children list _contains_ the "a" node
        assert!(tree.children(tree.root).unwrap().iter().any(|&item| {
            if let Some(ptr) = item {
                ptr::eq(ptr, node_a_ptr.unwrap())
            } else {
                false
            }
        }));

        // At this point there should be one node with one default ROOT node
        assert_eq!(tree.size, 1);
    }
}

#[test]
fn n_ary_algorithm_test() {
        // Adds an H5 under an H2

        // Adds an H2 after adding an H5

        // Empty doc test
}

/** Putting it all together */
pub fn example() {
    use crate::trees::md_toc_gen::GenTree;

    // Proof of concept
    // 1) Parse the file at the specified path and return a tuple containing
    // - 0: The doc title
    // - 1: A list of headings and their values
    let path = std::path::Path::new("src/trees/mock_data.md");
    let parsed = GenTree::<Heading>::parse_file(path);

    // Simple print without tree construction
    GenTree::<Heading>::simple_print(&parsed.0, &parsed.1);

    // 2) Constructs the tree
    let tree = GenTree::<Heading>::construct(&parsed.1);

    // 3) Takes a doc title and a tree root;
    // Traverses the tree and prints each node's raw data
    GenTree::<Heading>::pretty_print(&parsed.0, &tree.root);
    println!("");

    // Does the same thing as the above three steps, but adds the ability to
    // traverse a directory structure recursively and a pretty-printer
    // with proper box drawing components
    let path = std::path::Path::new("../tech-docs/src/content/docs/cs/CS61A.md");
    GenTree::<Heading>::navigator(path);
}
