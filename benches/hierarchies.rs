use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use dsa_rust::hierarchies::{
    safe_linked_gentree::GenTree as LinkedTree,
    //safe_linked_gentree_builder,
    arena_gentree::{GenTree as ArenaTree, Position},
    //avl_tree::AVLTree,
    //bin_heap::BinHeap,
};

//pub fn bench_hierarchies(c: &mut Criterion) {
//    let mut group = c.benchmark_group("hierarchies");
//
//    let text = "Safe Linked GenTree (Rc/RefCell)";
//    println!("\n{text}");
//    underline(text.len());
//    println!();
//
//    // Safe Linked GenTree build times
//    group.bench_function("safe_linked_gentree", |b| {
//        b.iter(|| {
//            black_box(construct_linked_gentree());
//        })
//    });
//
//    let text = "Arena GenTree";
//    println!("\n{text}");
//    underline(text.len());
//    println!();
//
//    // Safe Linked GenTree build times
//    group.bench_function("arena_gentree", |b| {
//        b.iter(|| {
//            black_box(construct_arena_gentree());
//        })
//    });
//
//    group.finish();
//}
pub fn bench_hierarchies(c: &mut Criterion) {
    let mut group = c.benchmark_group("hierarchies");

    // Test several tree sizes
    for &n in &[10, 100, 1_000, 10_000] {
        // Linked version
        group.bench_with_input(format!("linked_{n}"), &n, |b, &n| {
            b.iter(|| {
                black_box(construct_linked_gentree_n(n));
            })
        });

        // Arena version
        group.bench_with_input(format!("arena_{n}"), &n, |b, &n| {
            b.iter(|| {
                black_box(construct_arena_gentree_n(n));
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_hierarchies);
criterion_main!(benches);

fn underline(len: usize) {
    for _ in 0..len {
        print!("=")
    }
}

#[allow(unused)]
struct Heading {
    level: usize,
    title: String,
}
impl Heading {
    fn new(title: String, level: usize) -> Heading {
        Heading { level, title }
    }
}
fn make_headings(n: usize) -> Vec<Heading> {
    (0..n)
        .map(|i| Heading {
            level: (i % 6) + 1,
            title: format!("Node {i}"),
        })
        .collect()
}

fn construct_linked_gentree_n(n: usize) -> LinkedTree<Heading> {
    let mut tree = LinkedTree::<Heading>::new();
    let mut cursor = tree.cursor_mut();
    let mut cur_level = 0;

    for heading in make_headings(n) {
        let data_level = heading.level;

        // Case 1: Adds a child to the current parent and sets level cursor
        if data_level == cur_level + 1 {
            cursor.add_child(heading);
            cur_level += 1;
        }
        // Case 2: Adds a child with multi-generational skips
        else if data_level > cur_level {
            let diff = data_level - cur_level;
            for _ in 1..diff {
                let empty = Heading::new("[]".to_string(), 0);
                cursor.add_child(empty);
                cur_level += 1;
            }
            cursor.add_child(heading);
            cur_level += 1;
        }
        // Case 3: Adds sibling to current parent
        else if data_level == cur_level {
            cursor.ascend().ok();
            cursor.add_child(heading);
        }
        // Case 4: Adds a child to the appropriate ancestor,
        // ensuring proper generational skips
        else {
            let diff = cur_level - data_level;
            for _ in 0..=diff {
                cursor.ascend().ok();
                cur_level -= 1;
            }
            cursor.add_child(heading);
            cur_level += 1;
        }
    }
    tree
}

fn construct_arena_gentree_n(n: usize) -> ArenaTree<Heading> {
    let mut tree = ArenaTree::<Heading>::new_with_capacity(n);
    let mut cursor = tree.root().clone();
    let mut cur_level = 0;

    for heading in make_headings(n) {
        let title_level = heading.level;

        // Case 1: Adds a descendent with multi-generational skips
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
        }
        // Case 2: Adds sibling
        else if title_level == cur_level {
            cursor = tree.parent(&cursor).expect("Error: Cannot add sibling to root!");
            cursor = tree.add_child(&cursor, heading);
            cur_level += 1;
        }
        // Case 3: Adds a ancestor with mutli-generational skips
        else {
            let diff = cur_level - title_level;
            for _ in 1..diff {
                cursor = tree.parent(&cursor).expect("Error: Cannot traverse beyond root!");
                cur_level -= 1;
            }
            cursor = tree.add_child(&cursor, heading);
            cur_level += 1;
        }
    }
    tree
}

fn tree_vec() -> Vec<Heading> {
    vec![
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
    ]
}

pub const LEVEL: usize = 0;

fn construct_linked_gentree() -> LinkedTree<Heading> {

    let mut cur_level = LEVEL;
    let data = tree_vec();

    // Instantiates a Tree with a generic root and traversal positioning
    let mut tree: LinkedTree<Heading> = LinkedTree::<Heading>::new();
    let mut cursor = tree.cursor_mut(); // Sets cursor to tree.root

    // Constructs tree from Vec<T>
    for heading in data {
        let data_level = heading.level;

        // Case 1: Adds a child to the current parent and sets level cursor
        if data_level == cur_level + 1 {
            cursor.add_child(heading);
            cur_level += 1;
        }
        // Case 2: Adds a child with multi-generational skips
        else if data_level > cur_level {
            let diff = data_level - cur_level;
            for _ in 1..diff {
                let empty = Heading::new("[]".to_string(), 0);
                cursor.add_child(empty);
                cur_level += 1;
            }
            cursor.add_child(heading);
            cur_level += 1;
        }
        // Case 3: Adds sibling to current parent
        else if data_level == cur_level {
            cursor.ascend().ok();
            cursor.add_child(heading);
        }
        // Case 4: Adds a child to the appropriate ancestor,
        // ensuring proper generational skips
        else {
            let diff = cur_level - data_level;
            for _ in 0..=diff {
                cursor.ascend().ok();
                cur_level -= 1;
            }
            cursor.add_child(heading);
            cur_level += 1;
        }
    }
    tree
}

fn construct_arena_gentree() -> ArenaTree<Heading> {

    let mut cur_level = LEVEL;
    let data = tree_vec();

    //let mut tree: ArenaTree<Heading> = ArenaTree::<Heading>::new();
    let mut tree: ArenaTree<Heading> = ArenaTree::<Heading>::new_with_capacity(16);

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

