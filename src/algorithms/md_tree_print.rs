// To make testing/comparing functionality easier,
// this module imports an existing Heading struct
// which looks like:
// pub struct Heading<'a> {
//    level: usize,
//    title: &'a str,
// }
use crate::hierarchies::safe_linked_gentree_builder::Heading;

pub fn print_tree_diagram(list: &[Heading]) {
    // Initial empty list check with early return
    // Why waste effort on an empty list?
    if list.is_empty() {
        println!("\t[]");
        return;
    }

    // Phase 1: Creates a normalized list of cloned values (because
    // list is borrowed) by inserting missing/skipped heading levels
    // with empty "[]" placeholders.
    let mut normalized: Vec<Heading> = Vec::new();
    let mut cur_level = 0;
    for heading in list {
        // If there's a generational gap, synthesize intermediate nodes
        while cur_level + 1 < heading.level {
            cur_level += 1;
            normalized.push(Heading {
                level: cur_level,
                title: "[]".to_string(),
            });
        }
        // Requires Clone because Heading contains String
        // which cannot be Copy
        normalized.push(heading.clone());
        cur_level = heading.level;
    }

    // Initial stand-alone leader
    println!("   │");

    // Phase 2: Match normalized node listing with a stack-based
    // prefix tracking where each stack entry is a tuple containing
    // heading level and whether its the last sibling
    let mut ancestors: Vec<(usize, bool)> = Vec::new();

    // Iterate through the normalized list of nodes
    for (i, heading) in normalized.iter().enumerate() {
        // Pop ancestors deeper than or equal to current level
        while ancestors
            .last()
            .is_some_and(|&(lvl, _)| lvl >= heading.level)
        {
            ancestors.pop();
        }

        // Takes the normalized node listing and "peeks" ahead to see
        // if the current node is the "last" sibling.
        // If the "next" node.level is < current.level, the node is the last sibling.
        let is_last = !normalized[i + 1..]
            .iter()
            .take_while(|h| h.level >= heading.level)
            .any(|h| h.level == heading.level);

        // Build box plot prefix/leader
        let mut prefix = String::new();
        for &(_, is_last) in &ancestors {
            prefix.push_str(if is_last { "    " } else { "│   " });
        }

        // Print the node with initial indentation for visual effect
        println!(
            "   {}{}{}",
            prefix,
            if is_last { "└── " } else { "├── " },
            heading.title
        );

        ancestors.push((heading.level, is_last));
    }
}
