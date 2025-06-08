/*! # About
The purpose of this library is to explore building classical data structures and algorithms in Rust. In keeping with the classic curricula, many simpler structures rely on pointer-backed structures. It is almost always better so simply use `Vec` for these backing structures to take advantage of minimal allocations and cache locality despite occasional O(n) resize operations.

# Lists
These structures/modules represent a basic exploration of foundational, sequence-based structures. The concepts illustrated here can be used for more advanced data structures. I originally wrote 9+ lists, but have only decided to include 3 in this library due to overlapping utility. Attempting to produce anything like a decent index-based list (like `[T; N]` or `Vec<T>`) requires knowledge of allocators, which is still beyond me. All lists still exist in the repo for historical reference purposes. The [array-based list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/array_list.rs) gets a special mention as an example of how to use arrays in Rust, due to the necessity of compile-time sizing and initialization.

- [Singly-linked list](crate::lists::singly_linked_list): A safe, owned, singly-linked structure to illustrate `Box`. This implementation contains operations for simple stack and queue structures as well.

- [Doubly-linked list](crate::lists::doubly_linked_list): An unsafe doubly-linked list implementation with an added cursor to create a positional list that provides list splitting/splicing and supports in-place sorting with a merge sort algorithm (if you really wanna write a sorted list on top of it)

- [Vector-based circular queue](crate::lists::queues::vec_circ_queue): This structure was included simply because its fun, but it's still just a `Vec` with capacity constraints and wrapping logic

# Trees
Building off the lessons learned in the Core Lists module this section contains examples of hierarchical data structures.
The content currently combines general trees and search trees. This section is in active development.

- [Linked n-ary tree](crate::trees::safe_linked_gentree): A safe, undirected, unweighted, unconnected acyclic graph... err, tree. This implementation takes a traditional link-based approach to hierarchical structures. To avoid dangling pointers and reference cycles this implementation uses `Rc` and `RefCell`, and utilizes `Weak` pointers for proper drop semantics. The project repo contains [sample code](https://github.com/p5chmitz/dsa-rust/blob/main/src/trees/safe_linked_gentree_builder.rs) that illustrates a `tree`-like Markdown parser for heading values.

- [Indexed n-ary tree](crate::trees::arena_gentree): A safe, `Vec`-backed (indexed) general tree. The arena-style allocation used for this implementation is much easier than using `Rc`/`RefCell`. This implementation uses [Vec::swap_remove()] for `O(1)` removal operations while maintaining spatial efficiency by avoiding "leaked" or "orphaned" nodes with delete operations.

- [Linked binary search tree](crate::trees::linked_bst): Safe and easy; Nice

# Maps
One of the most useful structures in the real world. Maps are essentially just lists of key-value pairs with potentially
better expected asymptotics.

- [Chaining hash table](crate::maps::chaining_hash_table): Simple, easy, unsorted fun for the whole family; This implementation uses Vec-based backing and chaining structures with simple division compression;

- [Probing hash table](crate::maps::probing_hash_table): A little more complex, still unsorted, but arguably more performant by taking advantage of cache locality through a flattened structure; This Vec-based structure uses MAD compression and quadratic probing as well as a fun little secondary byte mask to distinguish available, occupied, and defunct indexes

- [Simple sorted map](crate::maps::sorted_map): No hashing, just simple tricks; This map is really just a vector of `Entry<K, V>`, but uses a binary search algorithm which reduces queries from O(n) to O(log n) time

# Algorithms

An exploration on some searching, sorting, and graph algorithms.

- Simple binary search

*/

// Declaring only what we want to surface
pub mod lists {
    pub mod doubly_linked_list;
    pub mod singly_linked_list;
    pub mod queues {
        pub mod vec_circ_queue;
    }
}
pub mod trees {
    pub mod linked_bst;
    pub mod safe_linked_gentree;
    pub mod safe_linked_gentree_builder;
    pub mod traits; // Necessary for gen tree
                    //pub mod unsafe_linked_general_tree;
    pub mod arena_gentree;
}
pub mod maps {
    pub mod chaining_hash_table;
    pub mod hash_lib; // Necessary for maps
    pub mod probing_hash_table;
    pub mod sorted_map;
}
pub mod maw;
pub mod tgg;
