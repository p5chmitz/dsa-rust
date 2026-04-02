/*! # About
This project contains mostly pedagogical implementations of classical data structures as a way to exemplify common patterns to deal with Rust's opinionated safety systems. This library exists as a companion piece to an entry-level suite of [narrative-based documentation](https://www.headyimage.com/cs/dsa/dsa-intro/) on data structures and algorithms, and is intended to provide examples of foundational in-memory data structures and provide implementation hints to common issues encountered when learning the Rust programming language. The material provides patters that (hopefully) address common issues when learning Rust such as the borrow checker, reference counting, complex lifetimes and elision, and even a bit of `unsafe` code when the need for performance and ergonomics eclipse guaranteed safety out-of-the-box.

The material aims for a data-oriented design approach. For example, APIs favor returning iterators instead of copied/mutated collections, and representations prefer contiguous storage layouts. This is in line with Rust's general design approach.

The designs presented here are not intended to compete directly with Rust's `std::collections` library. Even so, the included benchmarking harnesses illustrate that these simplified approaches stay well within acceptable performance envelopes, often indicating only 1-2x slower operational performance for tested workloads as compared to the highly optimized `std::collections` crate.

The material avoids any optimization that obfuscates semantics. The intent is to present code that is easy to reason about, and only includes `unsafe` operations as a matter of example where explicitly necessary. The preference for safety is a conscious choice to appeal to conceptual illustration over niche application and raw performance.

# Sequences
These structures/modules represent a basic exploration of foundational, sequence-based structures. The concepts illustrated here can be used for more advanced data structures.

- [Singly-linked list](crate::sequences::singly_linked_list): A singly-linked sequence structure of owned values written entirely in safe Rust. This implementation illustrates the basic `Box` pointer to heap-allocate node data and contains operations for simple stack and queue structures.

- [Doubly-linked list](crate::sequences::doubly_linked_list): A doubly-linked sequence structure of owned values written with ample use of raw pointers (and hopefully enough [Miri](https://github.com/rust-lang/miri) testing to prove that its safe and sound). This list includes a cursor for a positional implementation that provides list splitting/splicing out of the box. These additions allow the list to support in-place sorting, if you choose to implement it.

- [Vector-based circular queue](crate::sequences::queues::vec_circ_queue): This structure is a simple, fun illustration of a fixed-sized circular buffer. This list currently represents the only index-based list in the group, because its fun, but it's still just a `Vec` with capacity constraints and wrapping logic.

- [Contigous Dynamic Array](crate::sequences::dyn_array): A minimal `Vec` clone that uses contiguous storage, indexing semantics, and dynamic (geometric) growth behavior. NOTE: Under construction

- [Safe Indexed Skip List](crate::sequences::indexed_skip_list): A single-threaded skip list boasting _O(log(n))_ (expected) insert, lookup, and remove operations. This structure also contains functions for retrieving the Kth element and range queries with Rust's `RangeBounds` semantics.

# Hierarchies
Building off the lessons learned with Sequences, this section contains examples of hierarchical data structures.

#### General Trees
- [Linked n-ary tree](crate::hierarchies::safe_linked_gentree): A safe, undirected, unweighted, acyclic graph... err, tree. This implementation takes a traditional link-based approach to hierarchical structures. To avoid dangling pointers and reference cycles this implementation relies on [shared ownership with interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html?highlight=interior%20mutability#allowing-multiple-owners-of-mutable-data-with-rct-and-refcellt) via [Rc](std::rc) and [RefCell](std::cell), and utilizes [Weak](std::rc::Weak) parent pointers for proper drop semantics.

- [Indexed n-ary tree](crate::hierarchies::arena_gentree): A safe, `Vec`-backed (indexed) general tree. This experiment is meant to be easier than using `Rc`/`RefCell`, which comes with its own cost. This implementation uses a "free list" to track node removal to ensure "leaked" arena nodes are kept to an absolute minimum.

#### Heaps
- [Indexed binary heap](crate::hierarchies::bin_heap): A simple Vec-backed (min) binary heap. All sifting happens in _O(log(n))_ time. The structure also contains a generalized heap sort operation that takes any (coercable) slice over orderable elements.

#### Search Trees
- [AVL tree](crate::hierarchies::avl_tree): A self-balancing, Vec-backed binary search tree with an [in-order](https://www.headyimage.com/cs/dsa/trees#depth-first-traversal) snapshot iterator. This structure guarantees _O(log(n))_ search, insert, and delete operations, and is used to implement this library's (sorted) [AVL tree map](crate::associative::avl_tree_map).

# Associative Structures
One of the most useful structures in the real world. Associative structures are essentially just lists of key-value pairs with potentially better expected asymptotics.

#### Maps
- [Chaining hash map](crate::associative::chaining_hash_table): Simple, easy, unsorted fun for the whole family. This hash map uses a Vec-based backing structure with a simple division compression and chaining scheme to address collisions.

- [Probing hash map](crate::associative::probing_hash_table): A little more complex, still unsorted, but arguably more performant by taking advantage of cache locality through a flattened backing structure. This Vec-backed hash map uses MAD compression and quadratic probing as well as a fun little secondary byte mask to distinguish available, occupied, and defunct indexes.

- [AVL tree map](crate::associative::avl_tree_map): A sorted map built on this library's [AVL tree](crate::hierarchies::avl_tree) implementation.

#### Sets
- [Simple Set](crate::associative::hash_set): A simple unsorted set based on this library's probing hash map. This structure includes basic mutating and non-mutation versions basic mathematical set operations, but is mostly an exercise in writing custom iterators.

- [Sorted tree set](): Coming soon!

# Composite Structures
This category contains "miscelaneous" data structures that do not neatly fall into any of the other categories.

- [Adaptable priority queue](crate::composite::priority_queue): The notoriously hard-to-categorize adaptable priority queue combines two implementations from in this library; the [binary heap](crate::hierarchies::bin_heap) and the [hash map](crate::associative::probing_hash_table). This structure provides the best of both worlds with fast _O(1)_ key-based lookups and _O(log(n))_ additions, removals, and key/value mutations.

# Algorithms
An exploration on some searching, sorting, and graph algorithms.

- Heap sort
- Binary search

*/

// Centralized module declaration
// Declaring only what we want to surface
pub mod sequences {
    pub mod doubly_linked_list;
    pub mod singly_linked_list;
    pub mod queues {
        pub mod vec_circ_queue;
    }
    pub mod concurrent_skip_list;
    pub mod indexed_skip_list;
    pub mod dyn_array;
}
pub mod hierarchies {
    pub mod avl_tree;
    pub mod linked_bst;
    pub mod arena_bst;
    pub mod safe_linked_gentree;
    pub mod safe_linked_gentree_builder;
    pub mod traits; // Necessary for gen tree
    //pub mod unsafe_linked_general_tree;
    pub mod arena_gentree;
    pub mod arena_gentree_builder;
    pub mod bin_heap;
}
pub mod associative {
    pub mod avl_tree_map;
    pub mod chaining_hash_table;
    pub mod hash_lib; // Necessary for maps
    pub mod hash_set;
    pub mod probing_hash_table;
    pub mod sorted_map;
    //pub mod skip_map;
    //pub mod skip_set;
}
pub mod composite {
    pub mod priority_queue;
}
pub mod graphs {
    pub mod adjacency_list;
    pub mod edge_list;
}
pub mod util {
    pub mod bump_arena;
}
pub mod maw;
pub mod tgg;

pub mod algorithms{
    pub mod list_processing;
}
