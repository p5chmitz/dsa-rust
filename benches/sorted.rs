/*
Mirror, mirror, on the wall; which sorted structure is fastest of them all?
- Adaptable priority queue
- Binary heap
- Indexed skip list
- Indexed AVL tree
- std::BTreeMap
*/
use criterion::{criterion_group, criterion_main, Criterion};

//use dsa_rust::composite::priority_queue::PriorityQueue;
//use dsa_rust::hierarchies::{avl_tree::AVLTree, bin_heap::BinHeap};
//use dsa_rust::sequences::indexed_skip_list::SkipList;

use dsa_rust::associative::avl_tree_map::TreeMap as AVLMap;
use std::collections::BTreeMap;
use std::hint::black_box;

pub fn bench_associative(c: &mut Criterion) {
    let mut group = c.benchmark_group("associative_collections");

    // You can easily adjust this set
    let sizes = [10, 100, 1_000, 10_000, 1_000_000];

    //
    // Custom AVLMap
    //
    let text = "Custom AVLMap";
    println!("\n{text}");
    underline(text.len());
    println!();

    for &n in &sizes {
        group.bench_with_input(format!("avl_map_insert_{n}"), &n, |b, &n| {
            b.iter(|| {
                let mut map = AVLMap::new();
                for i in 0..n {
                    map.put(black_box(i), black_box("value"));
                }
                black_box(map);
            })
        });

        group.bench_with_input(format!("avl_map_remove_{n}"), &n, |b, &n| {
            b.iter(|| {
                let mut map = {
                    let mut m = AVLMap::new();
                    for i in 0..n {
                        m.put(i, "value");
                    }
                    m
                };
                for i in 0..n {
                    black_box(map.remove(i));
                }
            });
        });

        group.bench_with_input(format!("avl_map_search_{n}"), &n, |b, &n| {
            let mut map = AVLMap::new();
            for i in 0..n {
                map.put(i, "value");
            }
            b.iter(|| {
                for i in 0..n {
                    black_box(map.contains(i));
                }
            })
        });
    }

    //
    // std::collections::BTreeMap
    //
    let text = "std BTreeMap";
    println!("\n{text}");
    underline(text.len());
    println!();

    for &n in &sizes {
        group.bench_with_input(format!("std_btreemap_insert_{n}"), &n, |b, &n| {
            b.iter(|| {
                let mut map = BTreeMap::new();
                for i in 0..n {
                    map.insert(black_box(i), "value");
                }
                black_box(map);
            })
        });

        group.bench_with_input(format!("std_btreemap_remove_{n}"), &n, |b, &n| {
            b.iter(|| {
                let mut map = {
                    let mut m = BTreeMap::new();
                    for i in 0..n {
                        m.insert(i, "value");
                    }
                    m
                };
                for i in 0..n {
                    black_box(map.remove(&i));
                }
            });
        });

        group.bench_with_input(format!("std_btreemap_search_{n}"), &n, |b, &n| {
            let mut map = BTreeMap::new();
            for i in 0..n {
                map.insert(i, "value");
            }
            b.iter(|| {
                for i in 0..n {
                    black_box(map.contains_key(&i));
                }
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_associative);
criterion_main!(benches);

fn underline(len: usize) {
    for _ in 0..len {
        print!("=");
    }
    println!();
}
