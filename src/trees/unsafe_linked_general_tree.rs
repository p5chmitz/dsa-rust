/*! An unsafe, linked, n-ary tree implementation 

# About

#### Design

# Examples

*/

//use crate::trees::traits::Tree;
use std::ptr::NonNull;
use std::marker::PhantomData;

/** The Position struct is a concrete, lightweight struct that provides a safe 
handle to raw Node position data to avoid exposing/passing raw pointer data in 
the public GenTree and CursorMut APIs. 

```rust
    // Type aliasing exposes raw pointers! 🙅
    //type Pos<T> = Option<*mut Node<T>>; 
    
    // Nullable, even if `Some`
    //ptr: *mut Node<T>,
    //ptr: Option<*mut Node<T>>

    // Wrapping NonNull guarantees a valid pointer, if one exists
    ptr: Option<NonNull<Node<T>>> 
```

The `Position<T>` struct only contains methods for simple, internal operations 
like constructing and acquiring pointers, it is intended to be an opaque handle.
*/
pub struct Position<T> {
    ptr: Option<NonNull<Node<T>>>,
    _phantom: PhantomData<T>,
}
impl<T> Position<T> {

    /** Internal constructor only:
    Creates a safe handle to `Node<T>` and returns it as a `Position<T>`.
    Marked as unsafe because it takes a *mut which is the caller's
    responsibility to ensure is not null. */
    //pub(crate) unsafe fn new(ptr: *mut Node<T>) -> Self {
    unsafe fn new(ptr: *mut Node<T>) -> Self {
        Position { 
            // NonNull::new() automatically wraps the pointer in Option
            ptr: NonNull::new(ptr), 
            _phantom: PhantomData 
        }
    }

    /** Utility for internal access:
     
    Takes a `Position<T>` and safely extracts the raw `Node<T>` pointer. */
    //pub(crate) fn as_ptr(&self) -> Result<*mut Node<T>, String> {
    fn as_ptr(&self) -> Result<*mut Node<T>, String> {
        match self.ptr {
            Some(non_null_ptr) => Ok(non_null_ptr.as_ptr()),
            None => Err("Oh noez: pointer is null".to_string()),
        }
    }
}

/** Represents a general tree with a collection of children */
struct Node<T> {
    parent: Option<Position<T>>,
    children: Vec<Position<T>>, // Always exists for a Node, even if empty
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
    }
    // /** Builds a new Node and returns its position */
    // fn new(data: T) -> Position<T> {
    //     // Gotta make-a da box, then make-a da *mut 🤌
    //     unsafe { Position::new(Box::into_raw(Box::new(Node {
    //         parent: None,
    //         children: Vec::new(),
    //         data: Some(data),
    //     }))) }
    // }

    // /** Gets an immutable reference to the data at a position */
    // fn get<'a>(position: Position<T>) -> Option<&'a T> {
    //     // Safely extract the raw pointer from Position<T>
    //     // and then YOLO the deref
    //     if let Ok(p) = position.as_ptr() { 
    //         unsafe { (*p).data.as_ref() }
    //     } else {
    //         None
    //     }
    // }

    // /** Returns a reference to the Node's child Vec */
    // fn children(&self) -> &Vec<Position<T>> {
    //     &self.children
    // }
}

/** The Tree struct represents a positional, linked-based general
tree structure with a root node that contains a single raw pointer
to the root node and the structure's size.
The genericity of the struct means you'll have to explicitly
type associated functions.

Example:
```example
    let path = std::path::Path::new("~/Developer/project/src/doc");
    let parsed = Tree::<Heading>::parse(path);
    let tree = Tree::<Heading>::construct(parsed.1);
    Tree::<Heading>::preorder_proof(&tree.root);
```
*/
//#[derive(Debug)] // No Debug because Position is secret
struct GenTree<T> {
    root: Position<T>,
    size: usize,
}
impl<T> GenTree<T> {

    /** Instantiates a new Tree with a default root */
    pub fn new() -> GenTree<T> {
        //let root: Pos<T> = Some(Box::into_raw(Node::build(None)));
        let root: Position<T> = unsafe { Position::new(Box::into_raw(Node::build(None))) };
        GenTree { root, size: 0 }
    }

    /** Returns the Position of the tree's root */
    //pub fn root(&self) -> Position<T> {
    //    let ptr = self.root.as_ptr().ok().unwrap();
    //    unsafe { Position::new(ptr) }
    //} 

    // /** Creates a new node and returns its position */
    //pub fn new_node(&self, data: T) -> Position<T> {
    //    let boxed = Box::new(Node {
    //        parent: None,
    //        children: Vec::new(),
    //        data: Some (data)
    //    });
    //    //Some(Box::into_raw(boxed))
    //    unsafe { Position::new(Box::into_raw(boxed)) }
    //}

    /** Returns an immutable reference to a Node's data at the specified Position,
    if it exists */
    //pub fn get(&self, node: &Position<T>) -> Option<&T> {
    //    // Imperative approach
    //    if let Ok(n) = node.as_ptr() {
    //        unsafe { (*n).data.as_ref() } // Double de-ref for &*mut type
    //    } else {
    //        None
    //    }
    //    // Functional approach
    //    //node.as_ref().and_then(|n| unsafe { (*(*n)).data.as_ref() })
    //}

    fn dummy(&self) -> Position<T> {
        let ptr = self.root.as_ptr().ok().unwrap();
        unsafe { Position::new(ptr) }
    }

    /** Returns the parent of a given node, if it exists */
    //pub fn get_parent(&self, node: Position<T>) -> Option<Position<T>> {
    //    if let Ok(n) = node.as_ptr() {
    //        unsafe { (*n).parent }
    //    } else {
    //        None
    //    }
    //}
    //pub fn get_parent(&self, node: Position<T>) -> Option<Position<T>> {
    //    // Check if there even is a pointer
    //    let raw = match node.as_ptr() {
    //        Ok(ptr) => ptr,
    //        Err(_) => return None,
    //    };
    //
    //    // Deref the pointer and wrap it in a Position<T>, if Some
    //    // Borrow the parent Position (if any), extract the raw pointer, and wrap it in a new Position<T>
    //    unsafe {
    //        let parent_ptr = (*raw).parent.as_ref()?.as_ptr().ok()?;
    //        Some(Position {
    //            ptr: NonNull::new(parent_ptr),
    //            _phantom: PhantomData,
    //        })
    //    }
    //    // More explicit operation
    //    //unsafe {
    //    //    match &(*raw).parent {
    //    //        Some(position) => {
    //    //            let ptr = match position.as_ptr() {
    //    //                Ok(ptr) => ptr,
    //    //                Err(_) => return None,
    //    //            };
    //    //            Some(Position {
    //    //                ptr: NonNull::new(ptr),
    //    //                _phantom: PhantomData,
    //    //            })
    //    //        }
    //    //        None => None,
    //    //    }
    //    //}
    //}

    // /** Adds a child to a parent's children field represented as Vec<Pos<T>> */
    //pub fn add_child(&mut self, ancestor: Position<T>, data: T) {
    //    // Create the new child node
    //    let node: Box<Node<T>> = Box::new(Node { 
    //        parent: Some(ancestor),
    //        children: Vec::new(),
    //        data: Some(data)
    //    });
    //    let node_ptr: *mut Node<T> = Box::into_raw(node);
    //    let pos = unsafe { Position::new(node_ptr) };

    //    // Add the new child to the parent's child list
    //    unsafe {
    //        if let Some(p) = ancestor {
    //            (*p).children.push(Some(node_ptr));

    //            // Links the node's parent Pos<T> to the correct ancestor
    //            //(*node_ptr).parent = ancestor;
    //        }
    //        self.size += 1;
    //    }
    //    //Some(node_ptr)
    //}

    // /** Returns a reference to the collection of children for a given position, if any */
    //pub fn children(&self, node: Pos<T>) -> Option<&Vec<Pos<T>>> {
    //    if let Some(c) = node {
    //        Some(unsafe { (*c).children.as_ref() })
    //    } else {
    //        None
    //    }
    //}

    /** Returns true if the given position is the tree's root */
    //pub fn is_root(&self, node: Position<T>) -> bool {
    //    node == self.root
    //}

    /** Creates a `CursorMut<T>` starting at the tree's root */
    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        // Gets the *mut from root
        let ptr = self.root.as_ptr().ok().unwrap();
        // Constructs and returns the Position<T>
        let root = unsafe { Position::new(ptr) };

        CursorMut{
            node: root,
            tree: self
        }
    }

    // /** Returns the depth for a given node */
    //pub fn depth(&self, node: Pos<T>) -> Option<usize> {
    //    let mut d = 1;
    //    let mut cursor = node;
    //    while !self.is_root(cursor) {
    //        cursor = self.get_parent(cursor)?;
    //        d += 1;
    //    }
    //    Some(d)
    //}

    // /** Returns the height of a sub-tree at a given position */
    //pub fn height(&self, node: Pos<T>) -> Option<usize> {
    //    let mut h = 0;
    //    if let Some(n) = node {
    //        for e in unsafe { &(*n).children } {
    //            h = std::cmp::max(h, self.height(Some(e.expect("uh oh")))?)
    //        }
    //    }
    //    Some(h + 1)
    //}

}

//impl<T> Drop for GenTree<T> {
//    fn drop(&mut self) {
//        /** Recursive tree destructor */
//        // TODO: Update implementation with NonNull
//        // to avoid null pointer dereference check
//        unsafe fn drop_node_recursive<T>(node_ptr: *mut Node<T>) {
//            // Avoids a null pointer dereference
//            if node_ptr.is_null() {
//                return;
//            }
//
//            // Dereference the pointer and process its children
//            let node = &mut *node_ptr;
//            for &child_ptr in node.children.iter() {
//                if let Some(child_ptr) = child_ptr {
//                    drop_node_recursive(child_ptr);
//                }
//            }
//
//            // Deallocate the current node
//            let _ = Box::from_raw(node_ptr);
//        }
//
//        unsafe {
//            if let Some(root_ptr) = self.root {
//                drop_node_recursive(root_ptr);
//            }
//        }
//    }
//}

/** A cursor over mutable data that operates with the safe `Position<T>` handle over raw pointers. 

This is how we win with raw pointers */
pub struct CursorMut<'a, T> {
    //node: Pos<T>, // Option<*mut Node<T>>
    node: Position<T>, // ptr: Option<NonNull<Node<T>>>
    tree: &'a mut GenTree<T>,
}

impl<'a, T> CursorMut<'a, T> {

    // NAVIGATION
    /////////////

    /// Navigates down a generation, if Some
    ///
    /// WARNING: Unimplemented
    pub fn descendant(&self) -> Option<Position<T>> {
        unsafe { Some(Position::new(Box::into_raw(Node::build(None)))) }
        //let ptr = self.node.as_ptr().ok().unwrap();
        //unsafe {
        //    if let Some(children) = (*ptr).children { // Unsafe deref
        //        let parent_ptr = children.as_ptr().ok().unwrap();
        //        Some(Position::new(parent_ptr)) // Unsafe constructor
        //    } else { None }
        //}
    }
    /// Jumps the cursor to the current Node's first child
    ///
    /// TODO: split into first_child() and last_child()?
    /// Moves the cursor to the first child of the current node.
    //pub fn child(&mut self) {
    //    if let Some(ptr) = self.node {
    //        unsafe {
    //            let node = &*ptr;
    //            if let Some(&first_child) = node.children.first() {
    //                self.node = first_child;
    //            } else {
    //                self.node = None;
    //            }
    //        }
    //    }
    //}


    /** Navigates up a generation, if Some */
    pub fn ancestor(&self) -> Option<Position<T>> {
        let ptr = self.node.as_ptr().ok().unwrap();
        unsafe {
            if let Some(parent) = (*ptr).parent.take() { // Unsafe deref
                let parent_ptr = parent.as_ptr().ok().unwrap();
                Some(Position::new(parent_ptr)) // Unsafe constructor
            } else { None }
        }
    }

    /// Moves the cursor to the Node's next sibling
    ///
    /// TODO: split into first_sibling() and last_sibling()?
    /// Moves the cursor to the next sibling of the current node.
    //pub fn siblings(&mut self) -> Option<Position<T>> {
    //pub fn siblings(&mut self) -> Vec<Position<T>> {

    //    if let Ok(ptr) = self.node.as_ptr() {
    //        unsafe {
    //            let node = &*ptr;
    //            if let Some(parent_ptr) = node.parent {
    //                let parent = &*parent_ptr;
    //                if let Some(index) = parent.children.iter().position(|&child| child == Some(ptr)) {
    //                    if index + 1 < parent.children.len() {
    //                        self.node = parent.children[index + 1];
    //                        return;
    //                    }
    //                }
    //            }
    //            self.node = None;
    //        }
    //    }
    //}

    /** Every node has a children Vec, so the only safety concern is if theres 
    actually a Node at the `Position<T>`, which is handled by an internal
    as_ptr() method on `Position<T>`. */
    pub fn children(&self) -> &Vec<Position<T>> {
        let node_ptr = self.node.as_ptr().expect("Error dereferencing Node pointer");
        unsafe { (*node_ptr).children.as_ref() }
    }


    pub fn dummy(&self) -> Position<T> {
        // Just return tree.dummy()
        self.tree.dummy()
    }

    // METADATA
    ///////////

    /// Returns true if the current Node has data
    pub fn is_some(&self) -> bool {
        if let Some(node) = self.node.ptr {
            let ptr = node.as_ptr();
            unsafe { (*ptr).data.is_some() }
        } else { false }
    }

    /// Returns true if the current Node is empty
    pub fn is_none(&self) -> bool {
        if let Some(node) = self.node.ptr {
            let ptr = node.as_ptr();
            unsafe { (*ptr).data.is_none() }
        } else { false }
    }

    /** Returns a handle to the root Node, all trees have roots 

    WARNING: unimplemented */
    pub fn is_root(&self) -> Position<T> {
        // Gets the *mut from root
        let ptr = self.node.as_ptr().ok().unwrap();
        
        // Constructs and returns the Position<T>
        unsafe { Position::new(ptr) }
    }

    /** Returns the size of the Node's children Vec as usize */
    pub fn num_children(&self) -> Option<usize> {
        if let Ok(ptr) = self.node.as_ptr() {
            unsafe { Some((*ptr).children.len()) }
        } else { None }
    }

    // ACCESSORS AND MUTATORS
    /////////////////////////

    /// Gets an immutable reference to the data for the current Node
    //pub fn get(&self) -> Option<&T> {
    //    if let Some(ptr) = self.node {
    //        unsafe {
    //            (*ptr).data.as_ref()
    //        }
    //    } else {
    //        None
    //    }
    //}
    pub fn get(&self) -> Option<&T> {
        // Imperative approach
        if let Ok(n) = self.node.as_ptr() {
            unsafe { (*n).data.as_ref() }
        } else {
            None
        }
        // Functional approach
        //node.as_ref().and_then(|n| unsafe { (*(*n)).data.as_ref() })
    }

    /** Gets an immutable reference to the data for a supplied Position */
    pub fn get_for_pos(&self, pos: &Position<T>) -> Option<&'a T> {
        if let Ok(n) = pos.as_ptr() {
            unsafe { (*n).data.as_ref() }
        } else {
            None
        }
    }

    /// Overwrites the data for the current Node without affecting its position,
    /// returns the old data, if Some
    pub fn set(&mut self, data: T) -> Option<T> {
        if let Ok(n) = self.node.as_ptr() {
            unsafe { 
                let old = (*n).data.take();
                (*n).data = Some(data);
                return old;
            }
        } else {
            None
        }
    }

    /** Creates and adds a child Node at the cursor's Position, 
    returns the Position of the new node. */
    pub fn add_child(&mut self, data: T) -> Position<T> {
        // Get the *mut out of the parent's Position<T>
        let parent_ptr = self.node.as_ptr().ok().unwrap();

        // Create the new child node and give it a Position
        let new_boxed_node: Box<Node<T>> = Box::new(Node { 
            parent: Some(unsafe {Position::new(parent_ptr) }),
            children: Vec::new(),
            data: Some(data)
        });
        let node_ptr: *mut Node<T> = Box::into_raw(new_boxed_node);
        let new_node = unsafe { Position::new(node_ptr) };

        // Add the new child to the parent's child list
        unsafe { (*parent_ptr).children.push(new_node); }

        // Increment the size of the tree
        self.tree.size += 1;

        // Returns the Position<T> of the new node
        unsafe { Position::new(node_ptr) }
    }

    /** Returns an immutable reference to the parent position for the Node at 
    the current cursor position, if Some. */
    pub fn parent(&self) -> Option<&Position<T>> {
        if let Ok(ptr) = self.node.as_ptr() {
            unsafe {
                (*ptr).parent.as_ref()
            }
        } else { None }
    }

    /// Return a safe opaque handle to the current node's position
    pub fn get_position(&self) -> Position<T> {
        // We can be cavalier with safety because we know the Node exists,
        // we're already at it!
        let ptr = self.node.as_ptr().ok().unwrap();
        unsafe { Position::new(ptr) }
    }

    /** Jump the cursor to the defined Position */
    pub fn set_position(&mut self, new: &Position<T>) {
        let ptr = new.as_ptr().ok().unwrap();
        self.node = unsafe { Position::new(ptr) };
    }

    /// Jump to a previously saved position
    //pub fn move_to(&mut self, pos: Position<T>) {
    //    self.node = Some(pos.as_ptr());
    //}

    pub fn insert_sibling() {}

    pub fn insert_parent() {}

    pub fn insert_child() {}
}

pub mod builder {

    use regex::Regex;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use std::path::Path;
    use crate::trees::unsafe_linked_general_tree::{GenTree, CursorMut};

    struct Heading {
        level: usize,
        title: String,
    }
    impl Heading {
        /** Just a humble Heading builder */
        fn new(title: String, level: usize) -> Heading {
            Heading { level, title }
        }
    }

    /** Takes a path to a Markdown file, parses it for title and headings,
    and returns a tuple containing the document title and a vector of
    headings.
    
    Note: The document title portion of the tuple is specifically
    designed for the Astro-formatted frontmatter of each MD document. */
    fn parse(root: &Path) -> (String, Vec<Heading>) {
        // Regex for capturing the title from front matter
        let t = Regex::new(r"(?ms)^---.*?^title:\s*(.+?)\s*$.*?^---").unwrap();
        let mut doc_title = String::new();
        // Regex for capturing headings H1-H6 as #-######
        let h = Regex::new(r"^(#{1,6})\s+(.*)").unwrap();
        let mut headings: Vec<Heading> = Vec::new();
    
        // Read input
        let file_path = root;
        let file = File::open(file_path).unwrap(); // TODO: Fix lazy error handling
        let reader = BufReader::new(file);
    
        // Read the entire file into a single string
        // Imperative style
        let mut content = String::new();
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(&line);
        }
        // Functional style
        //let content: String = reader
        //    .lines()
        //    .map(|l| l.unwrap())
        //    .collect::<Vec<_>>()
        //    .join("\n");
    
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
    fn construct(level: usize, data: Vec<Heading>) -> GenTree<Heading> {
        // Debug print
        println!("Constructing tree...");

        // Instantiates a Tree with a generic root and traversal positioning
        let mut tree: GenTree<Heading> = GenTree::<Heading>::new();
        let mut level_cursor = level;
        // NOTE: Unwrap is safe here because we literally just created the tree
        let mut cursor = tree.cursor_mut();
        let mut temp = cursor.dummy();
    
        // Constructs tree from Vec<T>
        for heading in data {
            let current_level = heading.level;
    
            // Case 1: Adds a child to the current parent and sets level cursor
            if current_level == level_cursor + 1 {

                // Debug print
                println!("Case 1: {}: h{}", &heading.title, &heading.level);

                temp = cursor.add_child(heading);

                // Debug print
                //if cursor.get().is_some() {
                //    println!("cursor: {}", cursor.get().unwrap().title)
                //} else {
                //    println!("cursor is empty")
                //}

                // Debug print
                //match cursor.get_for_pos(&temp) {
                //    Some(val) => println!("Added new node: {}", val.title),
                //    None => println!("temp node is empty")
                //}

                level_cursor = cursor.get_for_pos(&temp).unwrap().level;
            }
    
            // Case 2: Adds a child with multi-generational skips with empty nodes
            else if current_level > level_cursor + 1 {
            
                // Debug print
                println!("Case 2: {}: h{}", &heading.title, &heading.level);

                let diff = current_level - level_cursor;
                for _ in 1..diff {
                    let heading = Heading::new("[]".to_string(), 0);
                    cursor.add_child(heading);
                    level_cursor += 1;
                }
                temp = cursor.add_child(heading);
                //let node_data = cursor.get();
                //level_cursor = node_data.unwrap().level;
                level_cursor = cursor.get_for_pos(&temp).unwrap().level;
            }
    
            // Case 3: Adds sibling to current parent
            else if current_level == level_cursor {
                // Debug print
                println!("Case 3: {}: h{}", &heading.title, &heading.level);

                //tree.add_child(tree.parent(position_cursor).expect("No parent"), node);
                cursor.add_child(heading);
            }
    
            // Case 4: Adds a child to the appropriate ancestor,
            // ensuring proper generational skips
            else {
                // Debug print
                println!("Case 4: {}: h{}", &heading.title, &heading.level);

                let diff = level_cursor - current_level;
                //cursor.parent();
                for _ in 0..diff {
                    cursor.ancestor();
                    level_cursor -= 1;
                }
                temp = cursor.add_child(heading);
                let node_data = cursor.get();
                level_cursor = node_data.unwrap().level;
            }
    
            // Updates the most recent addition
            cursor.set_position(&temp);
        }
        tree
    }
    
    // /** Modified preorder traversal function that walks the tree recursively 
    // printing each node's title and children with appropriate box drawing components */
    fn preorder(node: &CursorMut<Heading>, prefix: &str) {

        // Recursively visit each child, if they exist
        if node.children().len() > 0 {
            let mut index = node.children().len();

            for _ in node.children() {
                println!("index len: {index}");

                if let Some(node_data) = node.get() {
                    index -= 1;
                    if index == 0 {
                        println!("\t{}└── {}", prefix, node_data.title);
                        preorder(node, &format!("{}    ", prefix));
                    } else {
                        println!("\t{}├── {}", prefix, node_data.title);
                        preorder(node, &format!("{}│   ", prefix));
                    }
                }
            }
        }
    }

    /** A wrapper for a recursive preorder(ish) traversal function;
    Contains logic to print [] on empty trees for more appealing presentation */
    fn pretty_print(name: &str, position: CursorMut<Heading>) {
            //let children: &Vec<Pos<Heading>> = unsafe { (*(*p)).children.as_ref() };
            let children = &position.children();
            if children.len() == 0 {
                println!("📄 {}\n\t[]\n", name); // Empty trees
            } else {
                println!("📄 {}\n\t│", name);
                preorder(&position, "");
                println!("");
            }
    }
    
    /** A recursive function that chains the module's utility functions to 
    pretty-print a table of contents for each Markdown file in the specified 
    directory; The is_file() path contains logic to build a tree from filtered 
    values, skipping headers above the user-supplied level argument;
    The function also substitues the file name (if any) for all MD files
    not formatted with Astro's frontmatter */
    pub fn navigator(level: usize, path: &Path) {
        if path.is_dir() {
            for e in path.read_dir().expect("read_dir call failed") {
                let entry = e.expect("failure to deconstruct value");
                navigator(level, &entry.path()); // Recursive call
            }
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                match ext.to_str() { 
                    Some("md") | Some("mdx") => {
                        println!("{}", path.display());
                        let parsed = parse(path);
                        let mut name: String = parsed.0;
                        if name == "" {
                            if let Some(n) = path
                                .file_name()
                                .expect("Error extracting file name")
                                .to_str()
                            {
                                name = n.to_string()
                            }
                        }
                        let filtered = parsed.1.into_iter().filter(|h| h.level > level).collect();
                        let mut tree = construct(level, filtered);
                        pretty_print(&name, tree.cursor_mut());
                    }
                    _ => ()
                }
            }
        }
    }
    
    //#[cfg(test)]
    //mod tests{
    //
    //    use super::*;
    //
    //    #[test]
    //    fn basic_function_test() {
    //        use std::ptr; // Used by test
    //    
    //        // Creates a tree with a default ROOT node
    //        let mut tree = GenTree::<Heading>::new();
    //        if let Some(r) = tree.root {
    //            if let Some(h) = unsafe { (*r).data.as_ref() } {
    //                assert_eq!(&h.title, "ROOT");
    //            } else {
    //                panic!("Data is None!");
    //            }
    //        }
    //    
    //        // Builds a Heading that simulates an H2, converts it to a Node,
    //        // and finally converts it to a position Pos<Heading> as raw pointer "a"
    //        let h2 = Heading::new("H2".to_string(), 2);
    //        let node_a: Box<Node<Heading>> = Node::build(Some(h2));
    //        let node_a_ptr: Pos<Heading> = Some(Box::into_raw(node_a));
    //    
    //        // Adds a to root
    //        tree.add_child(tree.root, node_a_ptr);
    //    
    //        // Checks that add_child() assigns correct parent for the node
    //        assert_eq!(tree.root, tree.parent(node_a_ptr).expect("No parent"));
    //        // Checks that the parent (ROOT) has exactly one child as the "a" node
    //        assert_eq!(tree._children(tree.root), Some(&vec![node_a_ptr]));
    //        // Checks that the ROOT's children list _contains_ the "a" node
    //        assert!(tree._children(tree.root).unwrap().iter().any(|&item| {
    //            if let Some(ptr) = item {
    //                ptr::eq(ptr, node_a_ptr.unwrap())
    //            } else {
    //                false
    //            }
    //        }));
    //    
    //        // At this point there should be one node with one default ROOT node
    //        assert_eq!(tree.size, 2);
    //    
    //        // Builds a Heading that simulates an H3, converts it to a Node,
    //        // and finally converts it to a position Pos<Heading> as raw pointer "b"
    //        let h3 = Heading::new("H3".to_string(), 3);
    //        let node_b: Box<Node<Heading>> = Node::build(Some(h3));
    //        let node_b_ptr: Pos<Heading> = Some(Box::into_raw(node_b));
    //    
    //        // Adds "b" to "a"
    //        tree.add_child(node_a_ptr, node_b_ptr);
    //    
    //        // Checks the tree's size, height, and depth of "b"
    //        // NOTE: size, height, and depth include the ROOT node
    //        assert_eq!(tree.size, 3);
    //        assert_eq!(tree._height(tree.root), Some(3));
    //        assert_eq!(tree._depth(node_b_ptr), Some(3));
    //    }
    //    
    //    #[test]
    //    /** Creates this tree to test properties
    //        [] Lorem Ipsum Test
    //        │    An ordered look at MD parsing
    //        │
    //        ├── Landlocked
    //        │   ├── Switzerland
    //        │   │   └── Geneva
    //        │   │       └── Old Town
    //        │   │           └── Cathédrale Saint-Pierre
    //        │   └── Bolivia
    //        └── Island
    //          ├── Marine
    //          │   └── Australiae
    //          └── Fresh Water
    //    */
    //    fn n_ary_algorithm_test() {
    //    
    //        // Checks that the height is 4
    //    
    //        // Checks that the depth of the H5 is 4
    //    
    //        // Empty doc test
    //    }
    //}

}
