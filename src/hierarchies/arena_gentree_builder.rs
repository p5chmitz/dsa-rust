#![doc(hidden)]

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::hierarchies::arena_gentree::{Position, GenTree};

#[derive(Clone, Debug, PartialEq)]
pub struct Heading {
    pub level: usize,
    pub title: String,
}

 /** Takes a path to a Markdown file, parses it for title and headings,
and returns a tuple containing the document title and a vector of
Heading objects.

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

/// Constructs a tree of Heading types
pub fn construct(mut cur_level: usize, data: Vec<Heading>) -> GenTree<Heading> {
    let mut tree: GenTree<Heading> = GenTree::<Heading>::new();

    let mut cursor: Position = tree.root().clone(); // Sets cursor to tree.root
    //let mut cur_level = 0;

    // Constructs tree from Vec<T>
    for heading in data {
        let title_level = heading.level;

        // Case 1: Adds a child to the current position
        //if title_level == cur_level + 1 {
        //    cursor = tree.add_child(&cursor, heading);
        //    cur_level += 1;
        //    eprintln!("Case 1: {:#?}", tree.get_data(&cursor).unwrap().title);
        //}
        // Case 2: Adds a descendent with multi-generational skips
        if title_level > cur_level {
            let diff = title_level - cur_level;
            for _ in 0..=diff {
                let empty = Heading {
                    title: "[]".to_string(),
                    level: 0,
                };
                cursor = tree.add_child(&cursor, empty);
                cur_level += 1;
            }
            cursor = tree.add_child(&cursor, heading);
            cur_level += 1;
            //eprintln!("Case 2: {:#?}", tree.get_data(&cursor).unwrap().title);
        }
        // Case 3: Adds sibling
        else if title_level == cur_level {
            cursor = tree.parent(&cursor).expect("Error: Cannot add sibling to root!");
            cursor = tree.add_child(&cursor, heading);
            cur_level += 1;
            //eprintln!("Case 3: {:#?}", tree.get_data(&cursor).unwrap().title);
        }
        // Case 4: Adds a ancestor with mutli-generational skips
        else {
            let diff = cur_level - title_level;
            for _ in 1..diff {
                cursor = tree.parent(&cursor).expect("Error: Cannot traverse beyond root!");
                cur_level -= 1;
            }
            cursor = tree.add_child(&cursor, heading);
            cur_level += 1;
            //eprintln!("Case 4: {:#?}", tree.get_data(&cursor).unwrap().title);
        }
    }
    tree
}
// 2
// 1 
// 1 
// 1 
// 1 
// 4
// 4
// 2
// 3
// 4
// 1 
// 1 
// 4

/// Takes the parsed file name and the root of a GenTree type as 
/// a Position and pretty-prints the tree structure
/// Modified preorder traversal function that walks the tree recursively
/// printing each node's title and children with appropriate box drawing components
fn preorder<'a>(tree: &'a GenTree<Heading>, mut cursor: &'a Position, prefix: &str) {
    let children = tree.children(cursor);
    
    if !children.is_empty() {
        let mut index = children.len();
    
        for child_pos in children {
            index -= 1;
            cursor = child_pos;
    
            if let Some(child_data) = tree.get_data(child_pos) {
                if index == 0 {
                    println!("\t{}└── {}", prefix, child_data.title);
                } else {
                    println!("\t{}├── {}", prefix, child_data.title);
                }
            }
            if index == 0 {
                preorder(tree, cursor, &format!("{prefix}    "));
            } else {
                preorder(tree, cursor, &format!("{prefix}│   "));
            }
        }
    }
}

/// A wrapper for a recursive preorder(ish) traversal function;
/// Contains logic to print [] on empty trees for more appealing presentation
fn pretty_print(name: &str, tree: &GenTree<Heading>) {
    if tree.is_empty() {
        println!("📄 {name}\n\t[]\n"); // Empty trees
    } else {
        // TODO: Fix the semantic root impl
        // Current shadow instead of mutation because one is the true root,
        // and the next is the semantic root
        let position = &tree.root();
        let position = &tree.children(position)[0];
        println!("📄 {name}\n\t│");
        preorder(tree, position, "");
        println!();
    }
}

/// A recursive function that chains the module's utility functions to
/// pretty-print a table of contents for each Markdown file in the specified
/// directory; The is_file() path contains logic to build a tree from filtered
/// values, skipping headers above the user-supplied level argument;
/// The function also substitues the file name (if any) for all MD files
/// not formatted with Astro's frontmatter
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
                    //if parsed.1.is_empty() {
                    //    println!("File has no MD headings!\n");
                    //    return
                    //}
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
                    let tree = construct(level, filtered);
                    //let tree = construct_from(filtered);
                    pretty_print(&name, &tree);
                }
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    /** 
    Creates this tree to test properties
    */
    fn build_logic() {
        use crate::hierarchies::arena_gentree::GenTree;
        use crate::hierarchies::arena_gentree_builder::{Heading, construct};

        let tree_vec = vec![
            Heading {
                level: 1,
                title: "Landlocked".to_string(),
            },
            Heading {
                level: 2,
                title: "Switzerland".to_string(),
            },
            //Heading {
            //    level: 4,
            //    title: "Geneva".to_string(),
            //},
            //Heading {
            //    level: 5,
            //    title: "Old Town".to_string(),
            //},
            //Heading {
            //    level: 6,
            //    title: "Cathédrale Saint-Pierre".to_string(),
            //},
            //Heading {
            //    level: 3,
            //    title: "Bolivia".to_string(),
            //},
            //Heading {
            //    level: 6,
            //    title: "Puerta del Sol".to_string(),
            //},
            //Heading {
            //    level: 6,
            //    title: "Puerta de la Luna".to_string(),
            //},
            Heading {
                level: 1,
                title: "Islands".to_string(),
            },
            //Heading {
            //    level: 3,
            //    title: "Marine".to_string(),
            //},
            //Heading {
            //    level: 4,
            //    title: "Australia".to_string(),
            //},
            //Heading {
            //    level: 3,
            //    title: "Fresh Water".to_string(),
            //},
        ];

        // Constructs tree ignoring the first heading
        let tree: GenTree<Heading> = construct(0, tree_vec);
        //let mut tree: GenTree<Heading> = construct_from(tree_vec);
        // The root starts at 0, but the data starts at 1,
        // so you have to navigate to the children of the root
        // to access the "perceived" root
        let cursor = tree.root(); 
        let kids = tree.children(&cursor);
        let cursor = &kids[0];
        assert_eq!(tree.num_children(cursor), 2); // Root has [Landlocked, Islands]
        assert_eq!(tree.get_data(cursor).unwrap().title, "[]");

        //eprintln!("{tree:#?}");
        //panic!("MANUAL TEST FAILURE");

        // Tests that the tree/root contains data
        assert!(tree.get_data(cursor).is_some());
        assert!(tree.is_some(cursor));
        assert!(!tree.is_none(cursor));

        // Advances the cursor to the root's single child
        //cursor = tree.get_for_pos(cursor);

        // Tests children() and get_data()
        let kids = tree.children(cursor);
        let _kids_iter = kids.iter();

        // // Steps through the first node's children
        // cursor = kids_iter.next().unwrap();
        // let data = tree.get_data(cursor).unwrap();
        // assert_eq!(*data.title, "Landlocked".to_string());
        // cursor = kids_iter.next().unwrap();
        // let data = tree.get_data(cursor).unwrap();
        // assert_eq!(*data.title, "Islands".to_string());

        // // Jumps down a generation to Islands' kids [Marine, Fresh Water]
        // let new_kids = tree.children(cursor);
        // kids_iter = new_kids.iter();
        // cursor = kids_iter.next().unwrap();
        // let data = tree.get_data(cursor).unwrap();
        // assert_eq!(*data.title, "Marine".to_string());

        // // Jumps down another generation, for fun
        // let new_kids = tree.children(cursor).clone(); // Gets cursor's chidlren
        // kids_iter = new_kids.iter(); // Creates an iterator
        // cursor = kids_iter.next().unwrap(); // Moves to first child
        // let data = tree.get_data(cursor).unwrap();
        // assert_eq!(*data.title, "Australia".to_string());

        // // Tests parent()
        // let parent = tree.parent(cursor).unwrap(); // Marine
        // let data = tree.get_data(&parent).unwrap();
        // assert_eq!(*data.title, "Marine".to_string());
        // cursor = &parent;
        // let parent = tree.parent(cursor).unwrap(); // Islands
        // let data = tree.get_data(&parent).unwrap();
        // assert_eq!(*data.title, "Islands".to_string());
        // cursor = &parent;
        // let binding = tree.parent(cursor).unwrap();
        // cursor = &binding; // []
        // assert!(tree.parent(cursor).is_none()); // The root doesn't have any parents

        // // Descends to Islands to test delete()
        // let kids = tree.children(cursor); // Gets cursor's chidlren
        // let mut kids_iter = kids.iter(); // Creates an iterator
        // let _temp = kids_iter.next().unwrap().clone();
        // let temp = kids_iter.next().unwrap().clone();
        // cursor = &temp; // Moves to Islands
        // {
        //     let data = tree.get_data(cursor).unwrap();
        //     assert_eq!(*data.title, "Islands".to_string());
        // }

        // // Tests delete()
        // // Before deletion, checks that the childen are correct
        // let mut kids = Vec::new();
        // for child in tree.children(cursor).iter() {
        //     let title = tree.get_data(child).unwrap().title.clone();
        //     kids.push(title)
        // }
        // assert_eq!(kids, ["Marine".to_string(), "Fresh Water".to_string()]);

        // // Creates placeholder Heading
        // let mut deleted = Heading {
        //     title: String::new(),
        //     level: 0,
        // };
        // // Iterates through the child position's under the cursor
        // // looking for a matching Heading; Once found, jumps to that position,
        // // and deletes the Heading; The delete() operation automatically jumps
        // // the cursor to the parent of the deleted position
        // for position in tree.children(cursor).iter() {
        //     if tree.get_data(position).unwrap().title == *"Marine" {
        //         //cursor.jump(&position);
        //         deleted = tree.remove(position.clone()).unwrap();
        //         break;
        //     }
        // }
        // // Tests that the correct Heading was deleted
        // assert_eq!(deleted.level, 3);
        // assert_eq!(deleted.title, "Marine".to_string());

        // // Tests that the cursor got bumped up to Islands
        // let data = tree.get_data(cursor).unwrap();
        // assert_eq!(data.title, "Islands".to_string());

        // // Tests that the Islands node has the correct children
        // assert_eq!(tree.children(cursor).len(), 2);
        // let mut kids = Vec::new();
        // for child in tree.children(cursor).iter() {
        //     let title = tree.get_data(child).unwrap().title.clone();
        //     kids.push(title)
        // }
        // assert_eq!(kids, ["Fresh Water".to_string(), "Australia".to_string()]);

        // // Tests deleting the (empty) root
        // let parent = tree.parent(cursor); // Points to the (empty) root
        // let deleted = tree.remove(parent.unwrap());
        // assert_eq!(deleted.unwrap().title, "[]");
        // let new_root = tree.root();
        // let mut kids = Vec::new();
        // for child in tree.children(new_root).iter() {
        //     let title = tree.get_data(child).unwrap().title.clone();
        //     kids.push(title)
        // }
        // assert_eq!(
        //     kids,
        //     [
        //         "Switzerland".to_string(),
        //         "Bolivia".to_string(),
        //         "Islands".to_string()
        //     ]
        // );
    }

    #[test]
    /** 
    Creates this tree to test properties
        []
        ├── Landlocked
        │   ├── Switzerland
        │   │   └── Geneva
        │   │       └── Old Town
        │   │           └── Cathédrale Saint-Pierre
        │   └── Bolivia
        │       └── []
        │           └── []
        │               ├── Puerta del Sol
        │               └── Puerta de la Luna
        └── Islands
            ├── Marine
            │   └── Australia
            └── Fresh Water
    */
    fn md_heading_test() {
        use crate::hierarchies::arena_gentree_builder::{
            GenTree, 
            Heading, 
            construct, 
            pretty_print
        };

        let tree_vec = vec![
            Heading {
                level: 2,
                title: "Landlocked".to_string(),
            },
            Heading {
                level: 3,
                title: "Switzerland".to_string(),
            },
            Heading {
                level: 4,
                title: "Geneva".to_string(),
            },
            Heading {
                level: 5,
                title: "Old Town".to_string(),
            },
            Heading {
                level: 6,
                title: "Cathédrale Saint-Pierre".to_string(),
            },
            Heading {
                level: 3,
                title: "Bolivia".to_string(),
            },
            //Heading {
            //    level: 1,
            //    title: "Places".to_string(),
            //},
            Heading {
                level: 6,
                title: "Puerta del Sol".to_string(),
            },
            Heading {
                level: 6,
                title: "Puerta de la Luna".to_string(),
            },
            Heading {
                level: 2,
                title: "Islands".to_string(),
            },
            Heading {
                level: 3,
                title: "Marine".to_string(),
            },
            Heading {
                level: 4,
                title: "Australia".to_string(),
            },
            Heading {
                level: 3,
                title: "Fresh Water".to_string(),
            },
        ];

        // There is no such thing as heading 0
        let tree: GenTree<Heading> = construct(0, tree_vec);
        //let tree: GenTree<Heading> = construct_from(tree_vec);

        // Debug print the tree with placeholder title
        //eprintln!("{tree:#?}");
        pretty_print("Test Title", &tree);
        //panic!("MANUAL TEST FAILURE");

    }

    #[test]
        fn md_heading_test2() {
        use crate::hierarchies::arena_gentree_builder::{
            GenTree, 
            Heading, 
            construct, 
            pretty_print
        };

        let tree_vec = vec![
            Heading {
                level: 5,
                title: "Peter".to_string(),
            },
            Heading {
                level: 3,
                title: "Dingus".to_string(),
            },
            Heading {
                level: 4,
                title: "Dangus".to_string(),
            },
            Heading {
                level: 1,
                title: "Bobson".to_string(),
            },
            Heading {
                level: 6,
                title: "Dorkus".to_string(),
            },
            Heading {
                level: 3,
                title: "Flock".to_string(),
            },
            Heading {
                level: 1,
                title: "Dichael".to_string(),
            },
        ];

        let tree: GenTree<Heading> = construct(0, tree_vec);
        //let tree: GenTree<Heading> = construct_from(tree_vec);

        //eprintln!("{tree:#?}");
        pretty_print("ARENA TEST TITLE", &tree);
        
        //panic!("MANUAL TEST FAILURE");

    }


    #[test]
    // Fails with md/mdx files with no headings
    fn navigator_test() {
        use std::path::Path;
        // Should capture hierarchies/mock_data.md
        // and the project's README
        let pwd = Path::new(".");
        crate::hierarchies::arena_gentree_builder::navigator(0, pwd);

        //panic!("MANUAL TEST FAILURE");
    }
}
