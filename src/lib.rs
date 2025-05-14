/*! # About
The purpose of this crate is to explore building  classical data structures and algorithms in Rust.

# Core Lists
This is a basic exploration of foundational, sequence-based structures. The concepts
illustrated here can be used for more advanced data structures.

- [Array-based list](crate::lists::array_list): Contiguous data structures like arrays utilize cache locality for optimized 
storage and access; This module provides a basic introduction to lists (and arrays)

- [Vector-based list](crate::lists::vector_list): Vectors are Rust's dynamic array implementation; Vectors are often 
much more convenient than arrays with no operational downsides aside from periodic `O(n)` resize operations; 
Most of the time you'll want to use a `Vec` for a contiguous, sequential backing structure

- [Dynamic array list](crate::lists::dynamic_array_list): This module explores geometric re-sizing logic on top of 
`Vec` while maintaining a sorted invariant to illustrate the dynamic array structure

- [Singly-linked list](crate::lists::singly_linked_list): A safe, singly-linked implementation of a map-like (podium) 
structure to illustrate `Box`

- [Unsafe generic doubly-linked list](crate::lists::doubly_linked_list): A basic, unsorted, doubly-linked list 
implementation with an added cursor to create a positional list that provides list splitting/splicing and supports in-place 
sorting with a merge sort algorithm (if you really wanna write a sorted list on top of it)

## Stacks
Builds on the core lists by providing some fun stack-based explorations, including a symbol
balancer to check that an input string contains proper opening and closing symbols.

- [Singly-linked stack](crate::lists::stacks::safe_linked_stack): This module implements a stack-based symbol balancer for 
funsies using a home-rolled singly-linked list; `Vec` is more robust and takes advantage of cache locality
- [Unsafe singly-linked stack](crate::lists::stacks::unsafe_linked_stack): Just because its possible and we somehow thrive 
on making things more difficult than they have to be

## Queues

This section also utilizes the structures and approaches established in the Lists section. This section gets a little 
more fun with the idea of a circular queue and presents the final boss of linked-lists; an unsafe, doubly-linked deque 
that can be used as a stack, a queue, or some other generic linked list.

- [Vector-based circular queue](crate::lists::queues::vec_circ_queue): Probably the second most useful of all of these 
horridly useless lists; A little more fun/interesting, but it's still just a `Vec` with capacity constraints and wrapping logic

- [A simple linked-list queue](crate::lists::queues::singly_linked_queue): Kind of a busted implementation because 
`enqueue()` runs in O(n); It was a fun exercise though!

- [Unsafe doubly-linked queue]() This is the end of my lists, I swear; Why aren't you using `Vec` or `VecDeque` yet?!

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
    pub mod array_list;
    pub mod vector_list;
    pub mod dynamic_array_list;
    pub mod singly_linked_list; // ✔️
    pub mod doubly_linked_list; // ✔️
    pub mod stacks{
        pub mod safe_linked_stack; // ✔️
        pub mod unsafe_linked_stack;
    }
    pub mod queues{
        pub mod vec_circ_queue; // ✔️
        pub mod singly_linked_queue;
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
