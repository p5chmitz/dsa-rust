use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use dsa_rust::associative::{
    avl_tree_map::TreeMap as AVLMap,
    chaining_hash_table::ChainingHashTable as ChainMap,
    probing_hash_table::HashMap as HashMap,
};
use std::collections::{BTreeMap, HashMap as StdHashMap};

//pub fn bench_associative(c: &mut Criterion) {
//    let mut group = c.benchmark_group("associative_collections");
//
//    let text = "Custom AVLMap";
//    println!("\n{text}");
//    underline(text.len());
//    println!();
//
//    // AVLMap
//    group.bench_function("avl_map_insert_1000", |b| {
//        b.iter(|| {
//            let mut map = AVLMap::new();
//            for i in 0..1000 {
//                map.put(black_box(i), black_box("value"));
//            }
//            black_box(map);
//        })
//    });
//
//    group.bench_function("avl_map_remove_1000", |b| {
//        b.iter(|| {
//            let mut map = {
//                let mut m = AVLMap::new();
//                for i in 0..1000 { m.put(i, "value"); }
//                m
//            };
//            for i in 0..1000 {
//                black_box(map.remove(i));
//            }
//        });
//    });
//
//    group.bench_function("avl_map_search_1000", |b| {
//        let mut map = AVLMap::new();
//        for i in 0..1000 {
//            map.put(i, "value");
//        }
//        b.iter(|| {
//            for i in 0..1000 {
//                black_box(map.contains(i));
//            }
//        })
//    });
//
//    let text = "std BTreeMap";
//    println!("\n{text}");
//    underline(text.len());
//    println!();
//
//    // std BTreeMap
//    group.bench_function("std_btreemap_insert_1000", |b| {
//        b.iter(|| {
//            let mut map = BTreeMap::new();
//            for i in 0..1000 {
//                map.insert(black_box(i), "value");
//            }
//            black_box(map);
//        })
//    });
//
//    group.bench_function("std_btreemap_remove_1000", |b| {
//        b.iter(|| {
//            let mut map = {
//                let mut m = BTreeMap::new();
//                for i in 0..1000 { m.insert(i, "value"); }
//                m
//            };
//            for i in 0..1000 {
//                black_box(map.remove(&i));
//            }
//        });
//    });
//
//    group.bench_function("std_btreemap_search_1000", |b| {
//        let mut map = BTreeMap::new();
//        for i in 0..1000 {
//            map.insert(i, "value");
//        }
//        b.iter(|| {
//            for i in 0..1000 {
//                black_box(map.contains_key(&i));
//            }
//        })
//    });
//
//    let text = "Custom (probing) HashMap";
//    println!("\n{text}");
//    underline(text.len());
//    println!();
//
//    // HashMap
//    group.bench_function("hashmap_insert_1000", |b| {
//        b.iter(|| {
//            let mut map = HashMap::new();
//            for i in 0..1000 {
//                map.put(black_box(i), "value");
//            }
//            black_box(map);
//        })
//    });
//
//    group.bench_function("hashmap_remove_1000", |b| {
//        b.iter(|| {
//            let mut map = {
//                let mut m = HashMap::new();
//                for i in 0..1000 { m.put(i, "value"); }
//                m
//            };
//            for i in 0..1000 {
//                black_box(map.remove(&i));
//            }
//        });
//    });
//
//    group.bench_function("hashmap_search_1000", |b| {
//        let mut map = HashMap::new();
//        for i in 0..1000 {
//            map.put(i, "value");
//        }
//        b.iter(|| {
//            for i in 0..1000 {
//                black_box(map.contains(&i));
//            }
//        })
//    });
//
//    let text = "Custom (chaining) HashMap";
//    println!("\n{text}");
//    underline(text.len());
//    println!();
//
//    // ChainMap
//    group.bench_function("chainmap_insert_1000", |b| {
//        b.iter(|| {
//            let mut map = ChainMap::new();
//            for i in 0..1000 {
//                map.put(black_box(i), "value");
//            }
//            black_box(map);
//        })
//    });
//
//    group.bench_function("chainmap_remove_1000", |b| {
//        b.iter(|| {
//            let mut map = {
//                let mut m = ChainMap::new();
//                for i in 0..1000 { m.put(i, "value"); }
//                m
//            };
//            for i in 0..1000 {
//                //black_box(map.remove(i));
//                black_box(i);
//                map.remove(i);
//            }
//        });
//    });
//
//    group.bench_function("chainmap_search_1000", |b| {
//        let mut map = ChainMap::new();
//        for i in 0..1000 {
//            map.put(i, "value");
//        }
//        b.iter(|| {
//            for i in 0..1000 {
//                black_box(map.contains(i));
//            }
//        })
//    });
//
//    let text = "std HashMap";
//    println!("\n{text}");
//    underline(text.len());
//    println!();
//
//    // std HashMap
//    group.bench_function("std_hashmap_insert_1000", |b| {
//        b.iter(|| {
//            let mut map = StdHashMap::new();
//            for i in 0..1000 {
//                map.insert(black_box(i), "value");
//            }
//            black_box(map);
//        })
//    });
//
//    group.bench_function("std_hashmap_remove_1000", |b| {
//        b.iter(|| {
//            let mut map = {
//                let mut m = StdHashMap::new();
//                for i in 0..1000 { m.insert(i, "value"); }
//                m
//            };
//            for i in 0..1000 {
//                black_box(map.remove(&i));
//            }
//        });
//    });
//
//    group.bench_function("std_hashmap_search_1000", |b| {
//        let mut map = StdHashMap::new();
//        for i in 0..1000 {
//            map.insert(i, "value");
//        }
//        b.iter(|| {
//            for i in 0..1000 {
//                black_box(map.contains_key(&i));
//            }
//        })
//    });
//
//    group.finish();
//}
pub fn bench_associative(c: &mut Criterion) {
    let mut group = c.benchmark_group("associative_collections");

    // You can easily adjust this set
    let sizes = [10, 100, 1_000, 10_000];

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

    //
    // Custom (probing) HashMap
    //
    let text = "Custom (probing) HashMap";
    println!("\n{text}");
    underline(text.len());
    println!();

    for &n in &sizes {
        group.bench_with_input(format!("hashmap_insert_{n}"), &n, |b, &n| {
            b.iter(|| {
                let mut map = HashMap::new();
                for i in 0..n {
                    map.put(black_box(i), "value");
                }
                black_box(map);
            })
        });

        group.bench_with_input(format!("hashmap_remove_{n}"), &n, |b, &n| {
            b.iter(|| {
                let mut map = {
                    let mut m = HashMap::new();
                    for i in 0..n {
                        m.put(i, "value");
                    }
                    m
                };
                for i in 0..n {
                    black_box(map.remove(&i));
                }
            });
        });

        group.bench_with_input(format!("hashmap_search_{n}"), &n, |b, &n| {
            let mut map = HashMap::new();
            for i in 0..n {
                map.put(i, "value");
            }
            b.iter(|| {
                for i in 0..n {
                    black_box(map.contains(&i));
                }
            })
        });
    }

    //
    // Custom (chaining) HashMap
    //
    let text = "Custom (chaining) HashMap";
    println!("\n{text}");
    underline(text.len());
    println!();

    for &n in &sizes {
        group.bench_with_input(format!("chainmap_insert_{n}"), &n, |b, &n| {
            b.iter(|| {
                let mut map = ChainMap::new();
                for i in 0..n {
                    map.put(black_box(i), "value");
                }
                black_box(map);
            })
        });

        group.bench_with_input(format!("chainmap_remove_{n}"), &n, |b, &n| {
            b.iter(|| {
                let mut map = {
                    let mut m = ChainMap::new();
                    for i in 0..n {
                        m.put(i, "value");
                    }
                    m
                };
                for i in 0..n {
                    map.remove(i);
                }
            });
        });

        group.bench_with_input(format!("chainmap_search_{n}"), &n, |b, &n| {
            let mut map = ChainMap::new();
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
    // std::collections::HashMap
    //
    let text = "std HashMap";
    println!("\n{text}");
    underline(text.len());
    println!();

    for &n in &sizes {
        group.bench_with_input(format!("std_hashmap_insert_{n}"), &n, |b, &n| {
            b.iter(|| {
                let mut map = StdHashMap::new();
                for i in 0..n {
                    map.insert(black_box(i), "value");
                }
                black_box(map);
            })
        });

        group.bench_with_input(format!("std_hashmap_remove_{n}"), &n, |b, &n| {
            b.iter(|| {
                let mut map = {
                    let mut m = StdHashMap::new();
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

        group.bench_with_input(format!("std_hashmap_search_{n}"), &n, |b, &n| {
            let mut map = StdHashMap::new();
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
