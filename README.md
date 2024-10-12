## Data Structures & Algorithms
This is probably a terrible idea that Im going to regret, but with a bit of luck I hope to learn a thing or two _and_ retain my capacity for basic function in the process.

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

- Array list
A two-fer of dirt-simple array and Vector-based lists

- Singly-linked list
Simple and safe

- Doubly-linked list
A horribly unsafe linked list with just enough Miri testing to not immediately set the computer on fire
</details>

<details> 
<summary> Stacks & Queues </summary>
This section builds on the structures established in the Lists section by implementing a stack with Vectors and as a singly-linked list design.

- Stack
A Vector stack and an unsafe stack based on the doubly-linked list implementation

- Queue

</details>

