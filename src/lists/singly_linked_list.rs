/*! A safe, owned, singly-linked list

# About
The primary goal with this implementation is to offer a simple linked list with no `unsafe` code. The list does this with `Box`-type pointers to owned, heap-allocated objects.

#### Design
This exceedingly simple, safe, singly-linked list consists of one primary [LinkedList] struct that contains stack and queue operations. 

The list operates on a single, private `Node` struct that contains only a `data` field (generic over `T`) and a `next` pointer as `Option<Box<Node<T>>>`. Due to the owned nature of the pointers and safe-code-only design restriction, _it is not possible to sort this list in place_. 

#### Iterators
This list does not have any positional implementation, meaning that you cannot get pointers to arbitrary nodes within the list. The list _does_ provide an `iter()` method that yields an iterator over immutable references to `Node` data, consistent with Rust naming convention. Providing an `iter_mut()` that yields mutable references to the underlying `Node` data in safe Rust is currently beyond the scope of this structure. See the [doubly-linked variant](crate::lists::doubly_linked_list) which uses a `CursorMut` API for mutable references to `Node` data.

**TODO**: 
- Implement add/remove operations at arbitrary "indexes" (non-positional)

# Examples

This example illustrates stack (FILO) operations
```rust
    use dsa_rust::lists::singly_linked_list::LinkedList;

    let mut list: LinkedList<char> = LinkedList::new();

    list.push('c');
    list.push('b');
    list.push('a'); // list: a, b, c

    for e in list.iter() {
        println!("{e}")
    }

    // Tests that the head is really the head
    assert_eq!(list.peek().unwrap(), &'a');
    assert_eq!(list.pop().unwrap(), 'a'); // list: b, c

    list.push('d'); // list: d, b, c

    list.pop(); // list: b, c
    list.pop(); // list: c
    assert_eq!(list.peek().unwrap(), &'c');
    assert_eq!(list.pop().unwrap(), 'c'); // list: None

    assert_eq!(list.peek(), None); // Looks like theres None left
    assert_eq!(list.pop(), None); // dont wanna unwrap() on None!
```

<br>

This example illustrates queue (FIFO) operations
```rust
    use dsa_rust::lists::singly_linked_list::LinkedList;

    let mut list: LinkedList<char> = LinkedList::new();

    list.enqueue('a');
    list.enqueue('b');
    list.enqueue('c'); // list: a, b, c

    // Tests that the head is really the head
    assert_eq!(list.peek().unwrap(), &'a');
    assert_eq!(list.dequeue().unwrap(), 'a'); // list: b, c

    list.enqueue('d'); // list: b, c, d

    list.dequeue(); // list: c, d
    list.dequeue(); // list: d
    assert_eq!(list.peek().unwrap(), &'d');
    assert_eq!(list.dequeue().unwrap(), 'd'); // list: None

    assert_eq!(list.peek(), None); // Looks like theres None left
    assert_eq!(list.dequeue(), None); // dont wanna unwrap() on None!
                            
```

<br>

This example illustrates a sorting workaround
```rust
    use dsa_rust::lists::singly_linked_list::LinkedList;

    // 1) Create a new list of u8 values
    let mut list: LinkedList<u8> = LinkedList::new();

    // 2) (Add elements to the list)

    // 3) Push the list to an easily-sortable structure
    let mut data: Vec<_> = list.iter().cloned().collect();

    // 4) Sort the Vec in O(n log n) time
    data.sort();
    
    // 5) Reconstruct the list as a sorted variant
    let mut sorted_list = LinkedList::new();
    for node in data {
        // 
        // Use push() to create list in descending order
        // Use enqueue() to create list in ascending order
        sorted_list.enqueue(node);
    }
```
*/

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {

    // Creates a new Node with an optional next pointer
    fn new(data: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { data, next }
    }

    // Returns owned data from the node
    fn take(self) -> T {
        self.data
    }

    // Returns a reference to the data in a node
    fn _peek(&self) -> &T {
        &self.data
    }

    // Returns a reference to the data at the node's next pointer, if Some
    fn _peek_next(&self) -> Option<&T> {
        let next = &self.next;
        if let Some(node) = next {
            Some(&node.data)
        } else { None }
    }
}

/** The ❤️ of the module */
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}
impl<T: Clone> LinkedList<T> {

    /// Creates a new list
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    /** Adds a node to the head of the list/stack */
    pub fn push(&mut self, data: T) {

        // Create a new Node
        let new_node = Node::new(data, self.head.take());

        // Set the list's head to the new node
        self.head = Some(Box::from(new_node));

        // Increase the list's length
        self.length += 1;
    }

    /** Removes/returns the head of the list/stack (if Some) in O(1) time */
    pub fn pop(&mut self) -> Option<T> {
        Self::remove_front(self)
    }

    /** Adds a node to the tail of the list/queue in O(n) time;

    TODO: Safely implement a way to store a reference to the last node in the 
    list for O(1) enqueue operations */
    pub fn enqueue(&mut self, data: T) {

        // Wraps the data in an Option<Box<Node>>
        let new_node = Some(Box::from(Node::new(data, None)));

        // If the queue is empty, set `self.head` to the new node;
        // Otherwise, traverse to the end of the queue and add the new node
        if self.head.is_none() {
            self.head = new_node;
        } else {
            let mut iter_node = &mut self.head;
            while let Some(ref mut peek) = iter_node {
                if peek.next.is_none() {
                    peek.next = new_node;
                    break;
                }
                iter_node = &mut peek.next;
            }
        }
        self.length += 1;
    }

    /** Removes/returns the head of the list (if Some) in O(1) time */
    //pub fn dequeue(&mut self) -> Option<Node<T>> {
    pub fn dequeue(&mut self) -> Option<T> {
        Self::remove_front(self)
    }

    /** Returns a reference to the head of the list (if Some) in O(1) time */
    pub fn peek(&self) -> Option<&T> {
        //if let Some(s) = &(*self).head {
        //    Some(&s.data)
        //} else {
        //    None
        //}
        self.head.as_ref().map(|node| &node.data)
    }

    /** The meat and potatoes behind both pop() and dequeue() */
    fn remove_front(&mut self) -> Option<T> {
        if let Some(mut boxed_node) = self.head.take() {
            self.head = boxed_node.next.take();
            self.length -= 1;
            let n = *boxed_node;
            Some(n.take())
        } else {
            None
        }
    }

    /** Returns an iterator of references to data in the list's nodes */
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.next {
                self.next = current.next.as_deref(); // Move to next node
                Some(&current.data) // Return reference to data
        } else {
            None
        }
    }
}

#[test]
fn singly_linked_list_sort() {
    
    let mut list: LinkedList<u8> = LinkedList::new();

    // Creates a list by pushing elements to the head
    list.push(4);
    list.push(34); 
    list.push(46);
    list.push(196);
    list.push(98);
    list.push(3); 
    list.push(77);
    list.push(163); // Current head

    // Pushes the list to a sortable structure
    let mut data: Vec<_> = list.iter().cloned().collect();
    data.sort(); // Defaults to ascending order
    
    // Reconstructs the list as a sorted variant
    let mut sorted_list = LinkedList::new();
    for node in data {
        // Use push() to create list in descending order
        // Use enqueue() to create list in ascending order
        sorted_list.enqueue(node);
    }

    // Print debug blocks
    print!("unsorted list: ");
    for e in list.iter() {
        eprint!("{e}, ")
    }
    print!("\nsorted list: ");
    for e in sorted_list.iter() {
        eprint!("{e}, ")
    }
    //assert!(false); // Uncomment for cheap print debug trigger

    assert_eq!(list.peek().unwrap(), &163); // head of unsorted list
    assert_eq!(sorted_list.peek().unwrap(), &3); // head of sorted list

}
