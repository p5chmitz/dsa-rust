/*! Comparing hierarchies

# About
The first test compares two general (n-ary) tree strctures. The linked tree is a `Rc<RefCell<Node<T>>>` design, and the arena tree uses a `Vec`-based design with generational `Postion` types.

*/
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use dsa_rust::hierarchies::{
    //safe_linked_gentree_builder,
    arena_gentree::{GenTree as ArenaTree, Position},
    avl_tree::AVLTree as ArenaAVL,
    recursive_avl_tree::AVLTree as RecursiveAVL,
    //bin_heap::BinHeap,
    safe_linked_gentree::GenTree as LinkedTree,
};
use rand::rng;
use rand::seq::SliceRandom;
use std::hint::black_box;

pub fn bench_gentree(c: &mut Criterion) {
    // Identifies the group at runtime
    let mut group = c.benchmark_group("gentree");

    //let text = "Safe Linked GenTree (Rc/RefCell)";
    //println!("\n{text}");
    //underline(text.len());
    //println!();

    //let text = "Arena GenTree";
    //println!("\n{text}");
    //underline(text.len());
    //println!();

    let text = "Input random headings:";
    println!("\n{text}");
    underline(text.len());
    println!();

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

pub fn bench_sorted_trees(c: &mut Criterion) {
    let mut group = c.benchmark_group("bst");

    for &n in &[10, 100, 1_000, 10_000, 1_000_000] {
        // Generate exactly N unique random values
        // by shuffling a known sequence
        let mut values: Vec<usize> = (0..n).collect();
        values.shuffle(&mut rng());

        // Arena version
        group.bench_with_input(BenchmarkId::new("arena", n), &n, |b, &_n| {
            b.iter_batched(
                // Setup: Called before the timer starts
                || (ArenaAVL::<usize>::new(), values.clone()),
                // Measured: The timer runs only during this closure
                |(mut tree, vals)| {
                    for e in vals {
                        tree.insert(e);
                    }
                    // Return the tree so deallocation happens AFTER
                    // the timer stops
                    tree
                },
                // SmallInput tells Criterion it's cheap to generate
                // the setup data
                BatchSize::SmallInput,
            )
        });

        // Recursive version
        group.bench_with_input(BenchmarkId::new("recursive", n), &n, |b, &_n| {
            b.iter_batched(
                || (RecursiveAVL::<usize>::new(), values.clone()),
                |(mut tree, vals)| {
                    for e in vals {
                        tree.insert(e);
                    }
                    tree
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

criterion_group!(benches, bench_gentree, bench_sorted_trees);
criterion_main!(benches);

// UTILITY FUNCTIONS
////////////////////

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

// Type reference:
// pub struct Position {
//     ptr: usize,
//     generation: usize,
// }
// struct Node<T> {
//     parent: Option<Position>,
//     children: Vec<Position>,
//     data: Option<T>,
//     generation: usize,
// }
// pub struct GenTree<T> {
//     // RefCell moves structural mutation borrow checks
//     // from compile-time to runtime
//     arena: RefCell<Vec<Node<T>>>,
//     size: RefCell<usize>,
//     root: Position,
//     free_list: RefCell<Vec<usize>>,
// }
fn construct_arena_gentree_n(n: usize) -> ArenaTree<Heading> {
    let mut tree = ArenaTree::<Heading>::new_with_capacity(n);
    let mut cursor: Position = *tree.root();
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
            cursor = *tree
                .parent(&cursor)
                .expect("Error: Cannot add sibling to root!");
            cursor = tree.add_child(&cursor, heading);
            cur_level += 1;
        }
        // Case 3: Adds a ancestor with mutli-generational skips
        else {
            let diff = cur_level - title_level;
            for _ in 1..diff {
                cursor = *tree
                    .parent(&cursor)
                    .expect("Error: Cannot traverse beyond root!");
                cur_level -= 1;
            }
            cursor = tree.add_child(&cursor, heading);
            cur_level += 1;
        }
    }
    tree
}
