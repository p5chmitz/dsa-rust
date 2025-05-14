/*! # About
The purpose of this crate is to explore building  classical data structures and algorithms in Rust.

# Core Lists
These structures/modules represent a basic exploration of foundational, sequence-based structures. The concepts illustrated here can be used for more advanced data structures. I originally wrote 9+ lists, but have only decided to include 3 in this library due to overlapping utility. Attempting to produce anything like a decent index-based list (like `[T; N]` or `Vec<T>`) requires knowledge of allocators, which is still beyond me. All lists still exist in the repo for historical reference purposes. The [array-based list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/array_list.rs) gets a special mention as an example of how to use arrays in Rust, due to the necessity of compile-time sizing and initialization.

- [Singly-linked list](crate::lists::singly_linked_list): A safe, owned, singly-linked structure to illustrate `Box`. This implementation contains operations for simple stack and queue structures as well.

- [Doubly-linked list](crate::lists::doubly_linked_list): An unsafe doubly-linked list implementation with an added cursor to create a positional list that provides list splitting/splicing and supports in-place sorting with a merge sort algorithm (if you really wanna write a sorted list on top of it)

- [Vector-based circular queue](crate::lists::queues::vec_circ_queue): This structure was included simply because its fun, but it's still just a `Vec` with capacity constraints and wrapping logic

# Trees
Building off the lessons learned in the Core Lists module this section contains examples of hierarchical data structures. 
The content currently combines general trees and search trees. This section is in active development.

- [Unsafe linked N-ary tree](): An undirected acyclic graph... err, tree. This 
exercise also illustrates a `tree`-like Markdown parser; This is a labor of love for me as a professional documentarian as 
it generates a fancy hierarchical rendering of a Markdown document's headings, also known as a table of contents

- [Linked binary search tree](): Safe and easy; Nice

# Maps
One of the most useful structures in the real world. Maps are essentially just lists of key-value pairs with potentially 
better expected asymptotics.

- [Chaining hash table](crate::maps::chaining_hash_table): Simple, easy, unsorted fun for the whole family; This 
implementation uses Vec-based backing and chaining structures with simple division compression;

- [Probing hash table](crate::maps::probing_hash_table): A little more complex, still unsorted, but arguably more performant 
by taking advantage of cache locality through a flattened structure; This Vec-based structure uses MAD compression and 
quadratic probing as well as a fun little secondary byte mask to distinguish available, occupied, and defunct indexes

- [Simple sorted map](crate::maps::sorted_map): No hashing, just simple tricks; This map is really just a vector of 
`Entry<K, V>`, but uses a binary search algorithm which reduces queries from O(n) to O(log n) time

# Algorithms

An exploration on some searching, sorting, and graph algorithms.

- Simple binary search

*/


// Declaring only what we want to surface; This includes lib tests and dependencies for declared
// modules (such as crate::maps::hash_lib.rs)
pub mod lists{
    //pub mod array_list;
    //pub mod vector_list;
    //pub mod dynamic_array_list;
    pub mod singly_linked_list; // ✔️
    pub mod doubly_linked_list; // ✔️
    //pub mod stacks{
    //    pub mod safe_linked_stack; 
    //    pub mod unsafe_linked_stack;
    //}
    pub mod queues{
        pub mod vec_circ_queue; // ✔️
        //pub mod singly_linked_queue;
    }
}
pub mod trees{
    //pub mod unsafe_linked_general_tree; // ✔️
    //pub mod linked_bst; // ✔️
}
pub mod maps{
    pub mod chaining_hash_table; // ✔️
    pub mod probing_hash_table; // ✔️
    pub mod sorted_map;
    pub mod hash_lib;
}
pub mod maw;
pub mod tgg;


// TODO: Cull the library down to essentials 
// LISTS
// - Doubly-linked list (unsafe)
// - Singly-linked list/stack/queue (safe)
// - Circular queue (Vec-based, safe)
// - ? Priority queue
// MAPS
// - Chaining hash map
// - Probing hash map
// - ? Skip list
// TREES
// - Linked n-ary tree
// - Linked BST
// TODO: Provid links to repo illustrations/examples
// - Array list
// - Vector list
// - Dynamic array list
// - Singly linked list 
// - Singly linked stack (unsafe)
// - Singly linked queue (finish with proper last & next_last refs)
//
