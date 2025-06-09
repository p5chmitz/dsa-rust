#![doc(hidden)]

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::trees::safe_linked_gentree::{CursorMut, GenTree};

#[derive(Clone, Debug, PartialEq)]
pub struct Heading {
    pub level: usize,
    pub title: String,
}
impl Heading {
    // /** Just a humble Heading builder */
    //fn new(title: String, level: usize) -> Heading {
    //    Heading { level, title }
    //}
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
pub fn construct(mut cur_level: usize, data: Vec<Heading>) -> GenTree<Heading> {
    // Instantiates a Tree with a generic root and traversal positioning
    let mut tree: GenTree<Heading> = GenTree::<Heading>::new();
    let mut cursor = tree.cursor_mut(); // Sets cursor to tree.root

    // Constructs tree from Vec<T>
    for node in data {
        let data_level = node.level;

        // Case 1: Adds a child to the current parent
        if data_level == cur_level + 1 {
            cursor.add_child(node);
            cur_level += 1;
        }
        // Case 2: Adds a child with multi-generational skips
        else if data_level > cur_level {
            let diff = data_level - cur_level;
            for _ in 1..diff {
                //let empty = Heading::new("[]".to_string(), 0);
                let empty = Heading {
                    title: "[]".to_string(),
                    level: 0,
                };
                cursor.add_child(empty);
                cur_level += 1;
            }
            cursor.add_child(node);
            cur_level += 1;
        }
        // Case 3: Adds sibling to current parent
        else if data_level == cur_level {
            cursor.ascend().ok();
            cursor.add_child(node);
        }
        // Case 4: Adds a child to the appropriate ancestor,
        // ensuring proper generational skips
        else {
            let diff = cur_level - data_level;
            for _ in 0..=diff {
                cursor.ascend().ok();
                cur_level -= 1;
            }
            cursor.add_child(node);
            cur_level += 1;
        }
    }
    tree
}

/** Modified preorder traversal function that walks the tree recursively
printing each node's title and children with appropriate box drawing components */
fn preorder(cursor: &mut CursorMut<Heading>, prefix: &str) {
    let children = cursor.children().clone();

    if !children.is_empty() {
        let mut index = children.len();

        for child_pos in children {
            index -= 1;

            cursor.jump(&child_pos);

            // Cannot borrow immutably with get_data() and pass
            // the cursor as mutable recursively so this logic
            // needs to be separated into blocks
            if let Some(child_data) = cursor.get_data() {
                if index == 0 {
                    println!("\t{}└── {}", prefix, child_data.title);
                } else {
                    println!("\t{}├── {}", prefix, child_data.title);
                }
            } // child_cursor.get_data() immutable borrow ends
            if index == 0 {
                preorder(cursor, &format!("{}    ", prefix));
            } else {
                preorder(cursor, &format!("{}│   ", prefix));
            }
        }
    }
}

/** A wrapper for a recursive preorder(ish) traversal function;
Contains logic to print [] on empty trees for more appealing presentation */
//fn pretty_print(name: &str, position: &mut CursorMut<Heading>) {
fn pretty_print(name: &str, position: &mut CursorMut<Heading>) {
    let children = &position.children();
    if children.is_empty() {
        println!("📄 {}\n\t[]\n", name); // Empty trees
    } else {
        println!("📄 {}\n\t│", name);
        preorder(position, "");
        println!();
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
        for component in path.read_dir().expect("read_dir call failed") {
            let entry = component.expect("failure to deconstruct value");
            navigator(level, &entry.path()); // Recursive call
        }
    } else if path.is_file() {
        if let Some(ext) = path.extension() {
            match ext.to_str() {
                Some("md") | Some("mdx") => {
                    println!("{}", path.display());
                    let parsed = parse(path);
                    let mut name: String = parsed.0;
                    if name.is_empty() {
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
                    pretty_print(&name, &mut tree.cursor_mut());
                }
                _ => (),
            }
        }
    }
}
