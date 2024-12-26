## Data Structures & Algorithms
This is probably a terrible idea, but with a bit of luck hopefully I'll learn a thing or two and retain my capacity for basic function in the process.

## Contents
For detailed explanations and analysis of the structures and algorithms presented here check out the (error-filled) ramblings I publish on the [headyimage](https://www.headyimage.com/cs/dsa/dsa-intro/).

<details> 

<summary> Core Lists </summary>

The core lists section focuses on solutions to create a simple podium list. In the first phase of the project each list entry contains a name and a score, and the list maintains a sorted invariant. It is actually more efficient to periodically run a O(n log n) sorting function on a list that simply appends entries in O(1) time, but that approach sacrifices the convenience of the sorted invariant. Warning: writing a bunch of O(n) traversals is good practice, but can results in unnecessarily complex and error-prone code.

- [Array-based list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/array_list.rs): A basic introduction to lists (and arrays)
- [Vector-based list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/vector_list.rs): Vectors are more convenient and more powerful; theres no reason _not_ to use `Vec` here
- [Dynamic array list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/dynamic_array_list.rs): A little more involved; This variation imposes some manual logic on top of Rust's dynamic array `Vec` to illustrate geometric re-sizing while maintaining a sorted list invariant; Removes entries by name instead of index
- [Singly-linked list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/singly_linked_list.rs): A safe, singly-owned (via `Box`), singly-linked implementation of the podium 
- [Unsafe doubly-linked list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/doubly_linked_list_2.rs): My first attempt at a naive and horribly unsafe doubly-linked list with raw, mutable (and null-able) pointers everywhere; This implementation is balanced out with just enough Miri testing to not immediately cause concern among friends and loved ones; Removes entries by name instead of index

</details>

<details> 

<summary> Stacks </summary>

This section builds on the structures and approaches established in the Lists section. However, instead of featuring solutions to implement a podium, this section features slightly more pragmatic solutions including a symbol balancer. The real lesson here is that Rust's `Vec` type can serve as a fully-functional stack implementation right out of the box with `push`, `pop`, and `last` included methods. This module is all about learning though, so it starts with a useless wrapper to illustrate concepts and proceeds to get progressively sillier.

- [Vector-based stack (wrapper)](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/stacks/vector_stack.rs): Simple, effective, but deeply dumb; It's just `Vec` with a new jacket and sunglasses; This module includes two sub-modules that each illustrate a stack-based symbol-balancer; One wraps `Vec` and the other illustrates how needless that is by raw-dogging it
- [Singly-linked stack](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/stacks/safe_linked_stack.rs): This is probably the only viable implementation in this whole exercise; This module implements the stack-based symbol balancer for funsies; Unfortunately this list is still kinda dumb because `Vec` is already more robust and takes advantage of cache locality
- [Unsafe singly-linked stack](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/stacks/unsafe_linked_stack.rs): Just because its possible and we somehow thrive on making things more difficult than they have to be

</details>

<details> 

<summary> Queues </summary>

This section also utilizes the structures and approaches established in the Lists section. This section gets a little more fun with the idea of a circular queue and presents the final boss of linked-lists; an unsafe, doubly-linked deque that can be used as a stack, a queue, or some other generic linked list.

- [Vector-based queue (wrapper)](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/queues/vec_queue.rs): This wrapper illustrates the basics of the ADT; You should probably just use `Vec` or `VecDeque` (this list does, so why aren't you?)
- [A VecDeque-based queue (wrapper)](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/queues/vecdeque_queue.rs): Something something both ends; Are you using `VecDeque` yet? 
- [Vector-based circular queue](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/queues/vec_circ_queue.rs): Probably the second most useful of all of these horridly useless lists; A little more fun/interesting, but it's still just a `Vec` with capacity constraints and wrapping logic
- [A simple linked-list queue](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/queues/singly_linked_queue.rs): Kind of a busted implementation because `enqueue()` runs in O(n); It was a fun exercise though!
- [Unsafe doubly-linked queue]() This is the end of my lists, I swear; Why aren't you using `Vec` or `VecDeque` yet?!

</details>

<details> 

<summary> Trees </summary>

Building off the lessons learned in the Core Lists module this section contains examples of hierarchical data structures. The content currently combines general trees and search trees. This section is in active development.

- [Unsafe Linked N-ary Tree](): An undirected acyclic graph... err, tree. This exercise illustrates a `tree`-like Markdown parser; This is a labor of love for me as a professional documentarian as it generates a fancy hierarchical rendering of a Markdown document's headings, also known as a table of contents
- [Linked Binary Search Tree](): Safe and easy; Nice

</details>

<details> 

<summary> Algorithms </summary>

An exploration on some searching, sorting, and graph algorithms.

- Simple binary search

</details>

<details> 

<summary> Funsies </summary>

This section contains all the solutions to remedial problems and examples I collected along the way and liked enough to want to remember.

- Disk usage calculator
- Identifying unique elements in a Vector
- Calculate pre-fix averages of a Vector
- Simple factorial calculator
- Array reversal
- Fibonacci sequence calculator
- Tower of Hanoi solution

</details>

## Sources
So much information has gone into making these implementations work, but these are the primary drivers of the content here:
- [_Data Structures & Algorithms in Java_](https://www.wiley.com/en-au/Data+Structures+and+Algorithms+in+Java%2C+6th+Edition-p-9781118771334) by Goodrich, Tamassia, and Goldwasser
- [_Data Structures and Algorithm Analysis in Java_](https://www.pearson.com/en-us/subject-catalog/p/data-structures-and-algorithm-analysis-in-java/P200000003475/9780137518821) (and an older edition in C) by Weiss
- [_The Algorithm Design Manual_](https://www.algorist.com/) by Skiena
- [_Learn Rust With Entirely Too Many Linked Lists_](https://rust-unofficial.github.io/too-many-lists/index.html) by Aria Desires (with additional contributions by the Rust community)

