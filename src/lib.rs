/*! # About
This library explores implementations for classical data structures and algorithms in Rust. In keeping with the classic curricula, many simpler structures rely on pointer-backed structures. It is almost always better so simply use [Vec] for these backing structures to take advantage of minimal allocations and cache locality despite occasional _O(n)_ resize operations.

Many of the experiments in the creation of this library were left on the cutting room floor. The most common reasons for omission are naive implementation and specialized application which can be adequately addressed by more sophisticated or generic structures. See the [project repo]() to explore the full list of implementations.

# Sequences
These structures/modules represent a basic exploration of foundational, sequence-based structures. The concepts illustrated here can be used for more advanced data structures.

- [Singly-linked list](crate::sequences::singly_linked_list): A singly-linked sequence structure of owned values written entirely in safe Rust. This implementation illustrates the basic `Box` pointer to heap-allocate node data and contains operations for simple stack and queue structures.

- [Doubly-linked list](crate::sequences::doubly_linked_list): A doubly-linked sequence structure of owned values written with ample use of raw pointers (and hopefully enough [Miri](https://github.com/rust-lang/miri) testing to prove that its safe and sound). This list includes a cursor for a positional implementation that provides list splitting/splicing out of the box. These additions allow the list to support in-place sorting, if you choose to implement it.

- [Vector-based circular queue](crate::sequences::queues::vec_circ_queue): This structure is a simple, fun illustration of a fixed-sized circular buffer. This list currently represents the only index-based list in the group, because its fun, but it's still just a `Vec` with capacity constraints and wrapping logic

# Hierarchies
Building off the lessons learned in the Core Lists module this section contains examples of hierarchical data structures.
The content currently combines general trees and search trees. This section is in active development.

#### General Trees
- [Linked n-ary tree](crate::hierarchies::safe_linked_gentree): A safe, undirected, unweighted, unconnected acyclic graph... err, tree. This implementation takes a traditional link-based approach to hierarchical structures. To avoid dangling pointers and reference cycles this implementation uses `Rc` and `RefCell`, and utilizes `Weak` pointers for proper drop semantics. The project repo contains [sample code](https://github.com/p5chmitz/dsa-rust/blob/main/src/trees/safe_linked_gentree_builder.rs) that illustrates a `tree`-like Markdown parser for heading values.

- [Indexed n-ary tree](crate::hierarchies::arena_gentree): A safe, `Vec`-backed (indexed) general tree. The arena-style allocation used for this implementation is much easier than using `Rc`/`RefCell`. This implementation uses a "free list" to track node removal to ensure "leaked" arena nodes are kept to an absolute minimum.

#### Search Trees
- [Linked binary search tree](crate::hierarchies::linked_bst): Safe and easy. Nice.

- [Arena-backed AVL tree](crate::hierarchies::avl_tree): A naive arena-backed AVL tree with an in-order snapshot iterator. This structure is guarantees _O(log(n))_ search, insert, and delete operations.

#### Heaps
- [Indexed binary heap](crate::hierarchies::bin_heap): A simple (min) binary heap. All sifting happens in _O(log(n))_ time. The structure also contains a generalized heap sort operation that takes any (coercable) slice over orderable elements.

# Associative Structures
One of the most useful structures in the real world. Maps are essentially just lists of key-value pairs with potentially
better expected asymptotics.

#### Maps
- [Chaining hash table](crate::associative::chaining_hash_table): Simple, easy, unsorted fun for the whole family; This implementation uses a Vec-based backing structure with a simple division compression and chaining scheme to address collisions.

- [Probing hash table](crate::associative::probing_hash_table): A little more complex, still unsorted, but arguably more performant by taking advantage of cache locality through a flattened backing structure. This Vec-backed structure uses MAD compression and quadratic probing as well as a fun little secondary byte mask to distinguish available, occupied, and defunct indexes.

- [Simple sorted map](crate::associative::sorted_map): No hashing, just simple tricks; This naive map is really just a vector of `Entry<K, V>`, but uses a binary search algorithm in `find_index()` operations which reduces queries from _O(n)_ to _O(log(n))_ time. Insert operations are still _O(n)_ because it relies on the [Vec::insert] operation to maintain a sorted invariant.

- [Sorted AVL tree map](): Coming soon!

#### Sets
- [Simple Set](crate::associative::hash_set): A simple unsorted set based on this library's probing hash table implementation. This structure includes basic mutating and non-mutation versions for union, intersection, and subtraction operations.

- [Sorted tree set](): Coming soon!

# Composite Structures
This category contains "miscelaneous" data structures that do not neatly fall into any of the other categories.

- [Adaptable priority queue](crate::composite::priority_queue): The notoriously hard-to-categorize adaptable priority queue combines two implementations from in this library; the [binary heap](crate::hierarchies::bin_heap) and the [hash map](crate::associative::probing_hash_table). This structure provides the best of both worlds with fast _O(1)_ key-based lookups and _O(log(n))_ additions, removals, and key/value mutations.

# Algorithms
An exploration on some searching, sorting, and graph algorithms.

- Heap sort
- Binary search

*/

// Declaring only what we want to surface
pub mod sequences {
    pub mod doubly_linked_list;
    pub mod singly_linked_list;
    pub mod queues {
        pub mod vec_circ_queue;
    }
    //pub mod skip_list;
}
pub mod hierarchies {
    pub mod avl_tree;
    pub mod linked_bst;
    pub mod safe_linked_gentree;
    pub mod safe_linked_gentree_builder;
    pub mod traits; // Necessary for gen tree
                    //pub mod unsafe_linked_general_tree;
    pub mod arena_bst;
    pub mod arena_gentree;
    pub mod bin_heap;
}
pub mod associative {
    pub mod chaining_hash_table;
    pub mod hash_lib; // Necessary for maps
    pub mod probing_hash_table;
    pub mod skip_list;
    pub mod sorted_map;
    pub mod hash_set;
    //pub mod skip_map;
    //pub mod skip_set;
}
pub mod composite {
    pub mod priority_queue;
}
pub mod maw;
pub mod tgg;
