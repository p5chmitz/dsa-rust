use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use dsa_rust::sequences::{
    singly_linked_list::LinkedList as SinglyLinkedList,
    doubly_linked_list::LinkedList as DoublyLinkedList,
};

pub fn bench_sequences(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequences");

    let text = "Singly Linked List";
    println!("\n{text}");
    underline(text.len());
    println!();

    group.bench_function("singly_linked_build_1000", |b| {
        b.iter(|| {
            let mut list = SinglyLinkedList::new();
            for e in 0..1000 {
                list.push(e);
            }
            black_box(list);
        })
    });

    //group.bench_function("singly_linked_push_1000", |b| {
    //    b.iter(|| {
    //        let mut list = SinglyLinkedList::new();
    //        for e in 0..=1000 {
    //            list.push(black_box(e));
    //        }
    //    })
    //});
    
    group.bench_function("singly_linked_pop_1000", |b| {
        b.iter(|| {
            let mut list = SinglyLinkedList::new();
            for e in 0..1000 {
                list.push(e);
            }
            for _ in 0..1000 {
                black_box(list.pop());
            }
        })
    });

    let text = "Doubly Linked List";
    println!("\n{text}");
    underline(text.len());
    println!();

    group.bench_function("doubly_linked_build_1000", |b| {
        b.iter(|| {
            let mut list = DoublyLinkedList::new();
            for e in 0..1000 {
                list.push_tail(e);
            }
            black_box(list);
        })
    });

    //group.bench_function("doubly_linked_push_1000", |b| {
    //    b.iter(|| {
    //        let mut list = DoublyLinkedList::new();
    //        for e in 0..=1000 {
    //            list.push_tail(black_box(e));
    //        }
    //    })
    //});
    
    group.bench_function("doubly_linked_pop_1000", |b| {
        b.iter(|| {
            let mut list = DoublyLinkedList::new();
            for e in 0..1000 {
                list.push_tail(e);
            }
            for _ in 0..1000 {
                black_box(list.pop_head());
            }
        })
    });

    let text = "std::Doubly Linked List";
    println!("\n{text}");
    underline(text.len());
    println!();

    group.bench_function("std_linked_build_1000", |b| {
        b.iter(|| {
            let mut list = std::collections::LinkedList::new();
            for e in 0..1000 {
                list.push_back(e);
            }
            black_box(list);
        })
    });

    //group.bench_function("std_linked_push_1000", |b| {
    //    b.iter(|| {
    //        let mut list = std::collections::LinkedList::new();
    //        for e in 0..=1000 {
    //            list.push_back(black_box(e));
    //        }
    //    })
    //});
    
    group.bench_function("std_linked_pop_1000", |b| {
        b.iter(|| {
            let mut list = std::collections::LinkedList::new();
            for e in 0..1000 {
                list.push_back(e);
            }
            for _ in 0..1000 {
                black_box(list.pop_front());
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_sequences);
criterion_main!(benches);

fn underline(len: usize) {
    for _ in 0..len {
        print!("=")
    }
}
