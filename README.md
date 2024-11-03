## Data Structures & Algorithms
This is probably a terrible idea, but with a bit of luck hopefully I'll learn a thing or two and retain my capacity for basic function in the process.

## Contents
For detailed explanations and analysis of the structures and algorithms presented here check out the (error-filled) ramblings I publish on the [headyimage](https://www.headyimage.com/cs/dsa/dsa-intro/).

<details> 

<summary> Core Lists </summary>

The lists section centers around solutions to create a simple podium of sorted entries consisting of names and associated scores.

- [Array-based list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/array_list.rs): A basic introduction to lists (and arrays) 
- [Vector-based list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/vector_list.rs): Safe, easy, convenient... but SUPER naive and gross
- [Dynamic array list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/dynamic_array_list.rs): Safe, a little harder, a little less convenient, but addresses the multiple mutability issue skirted by the simple vec-based list... and also represents a half-assed lie
- [Singly-linked list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/singly_linked_list.rs): A safe, singly-owned, singly-linked list
- [Doubly-linked list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/doubly_linked_list_2.rs): A naive and horribly unsafe doubly-linked list with raw, mutable (and null-able) pointers everywhere balanced out with just enough Miri testing to not immediately cause concern among friends and loved ones

</details>

<details> 

<summary> Stacks </summary>

This section builds on the structures and approaches established in the Lists section. Instead of featuring solutions to implement a podium, this section features slightly more pragmatic solutions including a symbol balancer for the stack implementation. As it turns out, Rust's `Vec` type can serve as a fully-funcitonal stack implementation right out of the box with `push`, `pop`, and `last` included methods. This module is all about learning though, so it starts with a wrapper to illustrate concepts.

- [Vector-based stack (wrapper)](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/stacks/vector_stack.rs): Simple and effective, likely the preferred approach; This crate includes two modules that implement a symbol-balancer; One uses a raw `Vec` implementation and the other implements a `Vec` wrapper for funsies
- [Safe singly-linked stack](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/stacks/safe_linked_stack.rs): The easiest of the custom options; Seriously, just use `Vec`
- [Unsafe singly-linked stack](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/stacks/unsafe_linked_stack.rs): Just because its possible and we somehow thrive on making things more difficult than they have to be
- [Vector-based queue (wrapper)](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/queues/vec_queue.rs): Its a queue, why aren't you using `Vec` or `VecDeque`?

</details>

<details> 

<summary> Queues </summary>

This section builds on the structures and approaches established in the Lists section. Instead of featuring solutions to implement a podium, this section features slightly more pragmatic solutions including a symbol balancer for the stack implementation. As it turns out, Rust's `Vec` type can serve as a fully-funcitonal stack implementation right out of the box with `push`, `pop`, and `last` included methods. This module is all about learning though, so it starts with a wrapper to illustrate concepts.

- [Vector-based circular queue](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/queues/vec_circ_queue.rs): Its a Vec with capacity and wrapping logic
- [A simple linked-list queue](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/queues/singly_linked_queue.rs): Kind of busted because enqueue() runs in O(n); Might be good for something though? IDK
- [A VecDeque-based queue (wrapper)](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/queues/vecdeque_queue.rs): Something something both ends, but also the proper tool for the job
- [Unsafe doubly-linked queue]() This is the end of my lists, I swear

</details>

<details> 

<summary> Trees </summary>

This section explores tree structures.

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

