#![doc(hidden)]

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::hierarchies::arena_gentree::{GenTree, Position};

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

    let mut cursor: Position = tree.root().clone();

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
            cursor = tree
                .parent(&cursor)
                .expect("Error: Cannot add sibling to root!")
                .clone();
            cursor = tree.add_child(&cursor, heading);
            cur_level += 1;
            //eprintln!("Case 3: {:#?}", tree.get_data(&cursor).unwrap().title);
        }
        // Case 4: Adds a ancestor with mutli-generational skips
        else {
            let diff = cur_level - title_level;
            for _ in 1..diff {
                cursor = tree
                    .parent(&cursor)
                    .expect("Error: Cannot traverse beyond root!")
                    .clone();
                cur_level -= 1;
            }
            cursor = tree.add_child(&cursor, heading);
            cur_level += 1;
            //eprintln!("Case 4: {:#?}", tree.get_data(&cursor).unwrap().title);
        }
    }
    tree
}

/// Takes the parsed file name and the root of a GenTree type as
/// a Position and pretty-prints the tree structure
/// Modified preorder traversal function that walks the tree recursively
/// printing each node's title and children with appropriate box drawing components
fn preorder(tree: &GenTree<Heading>, mut cursor: Position, prefix: &str) {
    let children: std::cell::Ref<'_, Vec<Position>> = tree.children(&cursor);

    if !children.is_empty() {
        let mut index = children.len();

        for child_pos in children.iter() {
            index -= 1;
            cursor = child_pos.clone();

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

#[allow(unused)]
fn preorder2(tree: &GenTree<Heading>, cursor: Position, prefix: String) {
    let children = tree.children(&cursor);

    let len = children.len();

    for (i, child_pos) in children.iter().enumerate() {
        let is_last = i == len - 1;

        if let Some(child_data) = tree.get_data(child_pos) {
            if is_last {
                println!("{}└── {}", prefix, child_data.title);
            } else {
                println!("{}├── {}", prefix, child_data.title);
            }

            let mut next_prefix = prefix.clone();

            if is_last {
                next_prefix.push_str("    ");
            } else {
                next_prefix.push_str("│   ");
            }

            preorder(tree, child_pos.clone(), &next_prefix);
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
        preorder(tree, position.clone(), "");
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

    //#[test]
    // /// Creates this tree to test properties
    //fn build_logic() {
    //    use crate::hierarchies::arena_gentree::GenTree;
    //    use crate::hierarchies::arena_gentree_builder::{construct, Heading};

    //    let tree_vec = vec![
    //        Heading {
    //            level: 1,
    //            title: "Landlocked".to_string(),
    //        },
    //        Heading {
    //            level: 2,
    //            title: "Switzerland".to_string(),
    //        },
    //        //Heading {
    //        //    level: 4,
    //        //    title: "Geneva".to_string(),
    //        //},
    //        //Heading {
    //        //    level: 5,
    //        //    title: "Old Town".to_string(),
    //        //},
    //        //Heading {
    //        //    level: 6,
    //        //    title: "Cathédrale Saint-Pierre".to_string(),
    //        //},
    //        //Heading {
    //        //    level: 3,
    //        //    title: "Bolivia".to_string(),
    //        //},
    //        //Heading {
    //        //    level: 6,
    //        //    title: "Puerta del Sol".to_string(),
    //        //},
    //        //Heading {
    //        //    level: 6,
    //        //    title: "Puerta de la Luna".to_string(),
    //        //},
    //        Heading {
    //            level: 1,
    //            title: "Islands".to_string(),
    //        },
    //        //Heading {
    //        //    level: 3,
    //        //    title: "Marine".to_string(),
    //        //},
    //        //Heading {
    //        //    level: 4,
    //        //    title: "Australia".to_string(),
    //        //},
    //        //Heading {
    //        //    level: 3,
    //        //    title: "Fresh Water".to_string(),
    //        //},
    //    ];

    //    // Constructs tree ignoring the first heading
    //    let tree: GenTree<Heading> = construct(0, tree_vec);
    //    //let mut tree: GenTree<Heading> = construct_from(tree_vec);
    //    // The root starts at 0, but the data starts at 1,
    //    // so you have to navigate to the children of the root
    //    // to access the "perceived" root
    //    let cursor = tree.root();
    //    //let kids = tree.children(&cursor);
    //    //let cursor = &kids[0];
    //    //assert_eq!(tree.num_children(cursor), 2); // Root has [Landlocked, Islands]
    //    assert_eq!(tree.num_children(cursor), 1); // Root has a single, empty node []
    //    assert_eq!(tree.get_data(cursor).unwrap().title, "[]");

    //    //eprintln!("{tree:#?}");
    //    //panic!("MANUAL TEST FAILURE");

    //    // Tests that the tree/root contains data
    //    assert!(tree.get_data(cursor).is_some());
    //    assert!(tree.is_some(cursor));
    //    assert!(!tree.is_none(cursor));

    //    // Advances the cursor to the root's single child
    //    //cursor = tree.get_for_pos(cursor);

    //    // Tests children() and get_data()
    //    let kids = tree.children(cursor);
    //    let _kids_iter = kids.iter();

    //    // // Steps through the first node's children
    //    // cursor = kids_iter.next().unwrap();
    //    // let data = tree.get_data(cursor).unwrap();
    //    // assert_eq!(*data.title, "Landlocked".to_string());
    //    // cursor = kids_iter.next().unwrap();
    //    // let data = tree.get_data(cursor).unwrap();
    //    // assert_eq!(*data.title, "Islands".to_string());

    //    // // Jumps down a generation to Islands' kids [Marine, Fresh Water]
    //    // let new_kids = tree.children(cursor);
    //    // kids_iter = new_kids.iter();
    //    // cursor = kids_iter.next().unwrap();
    //    // let data = tree.get_data(cursor).unwrap();
    //    // assert_eq!(*data.title, "Marine".to_string());

    //    // // Jumps down another generation, for fun
    //    // let new_kids = tree.children(cursor).clone(); // Gets cursor's chidlren
    //    // kids_iter = new_kids.iter(); // Creates an iterator
    //    // cursor = kids_iter.next().unwrap(); // Moves to first child
    //    // let data = tree.get_data(cursor).unwrap();
    //    // assert_eq!(*data.title, "Australia".to_string());

    //    // // Tests parent()
    //    // let parent = tree.parent(cursor).unwrap(); // Marine
    //    // let data = tree.get_data(&parent).unwrap();
    //    // assert_eq!(*data.title, "Marine".to_string());
    //    // cursor = &parent;
    //    // let parent = tree.parent(cursor).unwrap(); // Islands
    //    // let data = tree.get_data(&parent).unwrap();
    //    // assert_eq!(*data.title, "Islands".to_string());
    //    // cursor = &parent;
    //    // let binding = tree.parent(cursor).unwrap();
    //    // cursor = &binding; // []
    //    // assert!(tree.parent(cursor).is_none()); // The root doesn't have any parents

    //    // // Descends to Islands to test delete()
    //    // let kids = tree.children(cursor); // Gets cursor's chidlren
    //    // let mut kids_iter = kids.iter(); // Creates an iterator
    //    // let _temp = kids_iter.next().unwrap().clone();
    //    // let temp = kids_iter.next().unwrap().clone();
    //    // cursor = &temp; // Moves to Islands
    //    // {
    //    //     let data = tree.get_data(cursor).unwrap();
    //    //     assert_eq!(*data.title, "Islands".to_string());
    //    // }

    //    // // Tests delete()
    //    // // Before deletion, checks that the childen are correct
    //    // let mut kids = Vec::new();
    //    // for child in tree.children(cursor).iter() {
    //    //     let title = tree.get_data(child).unwrap().title.clone();
    //    //     kids.push(title)
    //    // }
    //    // assert_eq!(kids, ["Marine".to_string(), "Fresh Water".to_string()]);

    //    // // Creates placeholder Heading
    //    // let mut deleted = Heading {
    //    //     title: String::new(),
    //    //     level: 0,
    //    // };
    //    // // Iterates through the child position's under the cursor
    //    // // looking for a matching Heading; Once found, jumps to that position,
    //    // // and deletes the Heading; The delete() operation automatically jumps
    //    // // the cursor to the parent of the deleted position
    //    // for position in tree.children(cursor).iter() {
    //    //     if tree.get_data(position).unwrap().title == *"Marine" {
    //    //         //cursor.jump(&position);
    //    //         deleted = tree.remove(position.clone()).unwrap();
    //    //         break;
    //    //     }
    //    // }
    //    // // Tests that the correct Heading was deleted
    //    // assert_eq!(deleted.level, 3);
    //    // assert_eq!(deleted.title, "Marine".to_string());

    //    // // Tests that the cursor got bumped up to Islands
    //    // let data = tree.get_data(cursor).unwrap();
    //    // assert_eq!(data.title, "Islands".to_string());

    //    // // Tests that the Islands node has the correct children
    //    // assert_eq!(tree.children(cursor).len(), 2);
    //    // let mut kids = Vec::new();
    //    // for child in tree.children(cursor).iter() {
    //    //     let title = tree.get_data(child).unwrap().title.clone();
    //    //     kids.push(title)
    //    // }
    //    // assert_eq!(kids, ["Fresh Water".to_string(), "Australia".to_string()]);

    //    // // Tests deleting the (empty) root
    //    // let parent = tree.parent(cursor); // Points to the (empty) root
    //    // let deleted = tree.remove(parent.unwrap());
    //    // assert_eq!(deleted.unwrap().title, "[]");
    //    // let new_root = tree.root();
    //    // let mut kids = Vec::new();
    //    // for child in tree.children(new_root).iter() {
    //    //     let title = tree.get_data(child).unwrap().title.clone();
    //    //     kids.push(title)
    //    // }
    //    // assert_eq!(
    //    //     kids,
    //    //     [
    //    //         "Switzerland".to_string(),
    //    //         "Bolivia".to_string(),
    //    //         "Islands".to_string()
    //    //     ]
    //    // );
    //}

    // #[test]
    // /**
    // Creates this tree to test properties
    //     []
    //     ├── Landlocked
    //     │   ├── Switzerland
    //     │   │   └── Geneva
    //     │   │       └── Old Town
    //     │   │           └── Cathédrale Saint-Pierre
    //     │   └── Bolivia
    //     │       └── []
    //     │           └── []
    //     │               ├── Puerta del Sol
    //     │               └── Puerta de la Luna
    //     └── Islands
    //         ├── Marine
    //         │   └── Australia
    //         └── Fresh Water
    // */
    //fn md_heading_test() {
    //    use crate::hierarchies::arena_gentree_builder::{
    //        construct, pretty_print, GenTree, Heading,
    //    };

    //    let tree_vec = vec![
    //        Heading {
    //            level: 2,
    //            title: "Landlocked".to_string(),
    //        },
    //        Heading {
    //            level: 3,
    //            title: "Switzerland".to_string(),
    //        },
    //        Heading {
    //            level: 4,
    //            title: "Geneva".to_string(),
    //        },
    //        Heading {
    //            level: 5,
    //            title: "Old Town".to_string(),
    //        },
    //        Heading {
    //            level: 6,
    //            title: "Cathédrale Saint-Pierre".to_string(),
    //        },
    //        Heading {
    //            level: 3,
    //            title: "Bolivia".to_string(),
    //        },
    //        //Heading {
    //        //    level: 1,
    //        //    title: "Places".to_string(),
    //        //},
    //        Heading {
    //            level: 6,
    //            title: "Puerta del Sol".to_string(),
    //        },
    //        Heading {
    //            level: 6,
    //            title: "Puerta de la Luna".to_string(),
    //        },
    //        Heading {
    //            level: 2,
    //            title: "Islands".to_string(),
    //        },
    //        Heading {
    //            level: 3,
    //            title: "Marine".to_string(),
    //        },
    //        Heading {
    //            level: 4,
    //            title: "Australia".to_string(),
    //        },
    //        Heading {
    //            level: 3,
    //            title: "Fresh Water".to_string(),
    //        },
    //    ];

    //    // There is no such thing as heading 0
    //    let tree: GenTree<Heading> = construct(0, tree_vec);
    //    //let tree: GenTree<Heading> = construct_from(tree_vec);

    //    // Debug print the tree with placeholder title
    //    //eprintln!("{tree:#?}");
    //    pretty_print("Test Title", &tree);
    //    //panic!("MANUAL TEST FAILURE");
    //}

    //#[test]
    //fn md_heading_test2() {
    //    use crate::hierarchies::arena_gentree_builder::{
    //        construct, pretty_print, GenTree, Heading,
    //    };

    //    let tree_vec = vec![
    //        Heading {
    //            level: 5,
    //            title: "Peter".to_string(),
    //        },
    //        Heading {
    //            level: 3,
    //            title: "Dingus".to_string(),
    //        },
    //        Heading {
    //            level: 4,
    //            title: "Dangus".to_string(),
    //        },
    //        Heading {
    //            level: 1,
    //            title: "Bobson".to_string(),
    //        },
    //        Heading {
    //            level: 6,
    //            title: "Dorkus".to_string(),
    //        },
    //        Heading {
    //            level: 3,
    //            title: "Flock".to_string(),
    //        },
    //        Heading {
    //            level: 1,
    //            title: "Dichael".to_string(),
    //        },
    //    ];

    //    let tree: GenTree<Heading> = construct(0, tree_vec);
    //    //let tree: GenTree<Heading> = construct_from(tree_vec);

    //    //eprintln!("{tree:#?}");
    //    pretty_print("ARENA TEST TITLE", &tree);

    //    //panic!("MANUAL TEST FAILURE");
    //}

    //#[test]
    //// Fails with md/mdx files with no headings
    //fn navigator_test() {
    //    use std::path::Path;
    //    // Should capture hierarchies/mock_data.md
    //    // and the project's README
    //    let pwd = Path::new(".");
    //    crate::hierarchies::arena_gentree_builder::navigator(0, pwd);

    //    //panic!("MANUAL TEST FAILURE");
    //}
}

pub mod stack_based_tree_printer {

    pub struct Heading {
        pub level: usize,
        pub title: String,
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
        let parsed = vec![
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
            Heading {
                level: 1,
                title: "Places".to_string(),
            },
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

        // Debug print the tree with placeholder title
        pretty_print(&parsed);

        //panic!("MANUAL TEST FAILURE");
    }

    // The internal representation of our tree
    struct Node {
        title: String,
        level: usize,
        children: Vec<usize>,
    }

    pub fn pretty_print(headings: &[Heading]) {
        if headings.is_empty() {
            return;
        }

        // 1. Storage Backend (Index Arena)
        // We use a Virtual Root at level 0 to anchor any top-level items.
        let mut nodes = vec![Node {
            title: "VIRTUAL_ROOT".to_string(),
            level: 0,
            children: Vec::new(),
        }];

        // The stack holds indices to `nodes`, representing the current active path down the tree.
        let mut stack = vec![0];

        for heading in headings {
            let target_level = heading.level;

            // Walk back up the tree if the new heading is at a higher or equal structural level
            // (which translates to a smaller or equal level integer)
            while stack.len() > 1 && nodes[*stack.last().unwrap()].level >= target_level {
                stack.pop();
            }

            let mut parent_idx = *stack.last().unwrap();

            // Structural Gap Fill: If we jump from e.g., Level 3 directly to Level 6,
            // we must insert dummy nodes for Levels 4 and 5.
            let parent_level = nodes[parent_idx].level;
            for l in (parent_level + 1)..target_level {
                let dummy_idx = nodes.len();
                nodes.push(Node {
                    title: "[]".to_string(),
                    level: l,
                    children: Vec::new(),
                });
                nodes[parent_idx].children.push(dummy_idx);
                stack.push(dummy_idx);
                parent_idx = dummy_idx;
            }

            // Insert the actual heading
            let new_idx = nodes.len();
            nodes.push(Node {
                title: heading.title.clone(),
                level: target_level,
                children: Vec::new(),
            });
            nodes[parent_idx].children.push(new_idx);
            stack.push(new_idx);
        }

        // 2. Traversal
        // We skip printing the Virtual Root itself and iterate over its top-level children.
        for &child_idx in &nodes[0].children {
            // Top-level items are passed as `is_root = true` so they don't get branch prefixes
            print_node(&nodes, child_idx, "", true, true);
        }
    }

    fn print_node(nodes: &[Node], idx: usize, prefix: &str, is_last: bool, is_root: bool) {
        let node = &nodes[idx];

        if is_root {
            // Top-level items have no structural prefix lines.
            println!("{}", node.title);
        } else {
            let branch = if is_last { "└── " } else { "├── " };
            println!("{}{}{}", prefix, branch, node.title);
        }

        // Prepare the prefix for the next generation of children
        let child_prefix = if is_root {
            prefix.to_string() // Root children start at column 0
        } else {
            let indent = if is_last { "    " } else { "│   " };
            format!("{prefix}{indent}")
        };

        let child_count = node.children.len();
        for (i, &child_idx) in node.children.iter().enumerate() {
            let child_is_last = i == child_count - 1;
            print_node(nodes, child_idx, &child_prefix, child_is_last, false);
        }
    }

    //fn preorder(list: &[Heading], prefix: &str) {
    //    let mut indent_val: usize = 0;
    //    for (i, val) in list.iter().enumerate() {
    //        if list[i].level < indent_val {// Last sibling
    //        } else if list[i].level == indent_val { // Sibling
    //        } else if list[i].level > indent_val { // Child
    //
    //        }
    //    }
    //}
}

#[allow(unused)]
#[test]
fn overflow_test() {
    fn factorial(n: usize) -> usize {
        if n <= 1 {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    // There is a threshhold at which the
    // function goes from overflowing usize
    // to overflowing its runtime call stack.
    // The threshhold for this example is
    // around 43.2k recursive calls
    //assert_eq!(factorial(100_000), 12); // overflows!!!
}
