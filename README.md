## Data Structures & Algorithms
This is probably a terrible idea that Im going to regret. Im here learn a thing or two, and with a bit of luck I'll retain my capacity for basic function in the process.

### Sources
So much information has gone into making these implementations work, but these are the primary drivers of the content here:
- [_Data Structures & Algorithms in Java_](https://www.wiley.com/en-au/Data+Structures+and+Algorithms+in+Java%2C+6th+Edition-p-9781118771334) by Goodrich, Tamassia, and Goldwasser
- [_Data Structures and Algorithm Analysis in Java_](https://www.pearson.com/en-us/subject-catalog/p/data-structures-and-algorithm-analysis-in-java/P200000003475/9780137518821) (and an older edition in C) by Weiss
- [_The Algorithm Design Manual_](https://www.algorist.com/) by Skiena
- [_Learn Rust With Entirely Too Many Linked Lists_](https://rust-unofficial.github.io/too-many-lists/index.html) by Aria Desires (with additional contributions by the Rust community)

### Contents
For detailed explanations and analysis of the structures and algorithms presented here check out the (eratta-filled) ramblings I publish on the [headyimage](https://www.headyimage.com/cs/dsa/dsa-intro/).

<details> 

<summary> Lists </summary>

The lists section centers around solutions to create a simple podium of sorted entries consisting of names and associated scores.


- [Array-based list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/array_list.rs): A basic introduction to lists (and arrays) 
- [Vector-based list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/vector_list.rs): Safe, easy, convenient
- [Singly-linked list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/singly_linked_list.rs): A safe, singly-owned, singly-linked list
- [Doubly-linked list](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/doubly_linked_list_2.rs): A horribly unsafe linked list with raw, mutable pointers and just enough Miri testing to not immediately set the computer on fire
- Doubly-linked list: An incremental improvement over the first iteration with NonNull (coming soon)

</details>

<details> 

<summary> Stacks </summary>

This section builds on the structures and approaches established in the Lists section.


- [Vector-based stack](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/vector_stack.rs): Simple and effective, likely the preferred approach
- [Singly-owned linked list stack](https://github.com/p5chmitz/dsa-rust/blob/main/src/lists/linked_stack.rs): Just because a implementing a stack with a safe, simple linked-list is possible

</details>

<details> 

<summary> Algorithms </summary>

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

