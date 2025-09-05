/*! A doubly-linked list implementation over raw pointers

# About
Even in 2025 you still hear stories about coding interviews that involve linked lists despite the fact that the world has mostly moved on from them. Most programs opt instead to use contiguous storage structures that take advantage of cache locality and minimal allocations despite some hard coping with amortized `O(1)` "add" operations. So why do linked lists remain? What gives these simple structures such rich lore? The reality is that its probably just a simple litmus test to ensure you were awake in your CS courses. Linked lists are traditionally introduced early on in CS programs because they provide a good introduction to the foundational concepts required to build more complex structures, such as managing references (and/or pointers) correctly (and soundly in languages that use pointers).

Rust takes a notoriously different approach to pointers and memory safety from languages like C/C++ which can make otherwise simple pointer-based structures unusually difficult for Rust novices. Singly-linked lists are easy to build with safe, beginner-friendly Rust code because each node only requires a single reference in an adjacent node. Using the [`Box`] type to create pointers in singly-linked lists neatly follows Rust's "one mutable reference or many immutable references" borrowing rules. It's in doubly-linked lists where you have to start keeping multiple mutable references to an object where things get necessarily tricky.

One option for safe code is to use smart pointers like [`RefCell`](std::cell::RefCell) that provide [interior mutability](https://doc.rust-lang.org/reference/interior-mutability.html). You may even choose to wrap it in a [`Rc`](std::rc::Rc) type for multiple ownership, but this approach gets unwieldy fast and requires runtime checks which may incur undesirable performance hits<sup>[1]</sup>.

Pragmatically speaking, the humble linked list only really excels in situations where you need _lots_ of list alterations (think splices and splits), so you want a structure that is as performant as possible. The natural conclusion is that you must dip into the shadows of `unsafe` Rust with either [`*mut T`](https://doc.rust-lang.org/std/primitive.pointer.html) or [`NonNull`](std::ptr::NonNull) pointers to get those _blazingly fast_... linked lists.

This module attempts to illustrate the delicate operations necessary to safely manage multiple sets of raw, `unsafe` pointers in Rust. Your friends all told you this was a bad idea, but I made the sacrifice to find out why so you don't have to.

<sup>[1]</sup> See the chapter on `Rc<RefCell<T>>` in the famous [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/fourth.html) book for details.

# Design
The module consists of two primary structs; [`LinkedList`] and [`CursorMut`]. The list works by storing and managing pointer-based links to (private) `Node` structs. The `LinkedList` struct itself contains links to the list's head and its tail, and stores the list's length, which is simply the number of nodes that make up the list. You do not operate on nodes directly, but rather through list and cursor operations. Each node contains data, a link to the previous node, and a link to the  next node. An empty list contains no nodes, but as soon as you add a single node, that node becomes the list's head _and_ tail node. Because each node contains links to previous and next nodes, a list with a single node effectively contains two "ghost" (sentinel) nodes in front of and behind the single list node.

```text
    None <- A -> None
```

The "ghost" nodes dont have any data or any links, which provides a natural stopping point for attempts to move beyond the head or tail of the list. You can remove or replace the head and tail nodes, but you cannot define what lays beyond until you get there. 👻 Ultimately, this is an overly poetic and long-winded way to say that the list does not wrap or provide circular buffering.

The list takes on a more familiar shape once you start adding nodes.

```text
    None <- A <-> B <-> C -> None
```

#### The `LinkedList` Struct
The [`LinkedList`] struct contains methods for basic list operations. These operations allow you to use
the list as an unsorted linked list, a stack, or a queue with insert and remove operations
on both the front and tail of the list in `O(1)` time.

```rust
   use dsa_rust::sequences::doubly_linked_list::LinkedList;

   let mut list = LinkedList::new();
   list.push_tail("a");
   list.push_tail("b");
   list.push_tail("c");

   print!("First:  [");
   let mut i = 0;
   for e in list.iter() {
       print!("{e}");
       if i == list.len() - 1 {
           print!("]");
       } else {
           print!(", ");
       }
       i += 1;
   }
```

#### The `CursorMut` Struct
Rob Pike has famously claimed to have never written a program with cursors. Unfortunately for me, I'm not as clever as Rob Pike, so this module's second major struct is [`CursorMut`]. This mutable cursor type adds positional functionality to the list and contains functions for splitting and splicing lists, and range operations.

```rust
    use dsa_rust::sequences::doubly_linked_list::LinkedList;

    // First list
    let mut first = LinkedList::new();
    first.push_tail("a");
    first.push_tail("b");
    first.push_tail("c");
    assert_eq!(first.peek_head(), Some(&"a"));
    assert_eq!(first.peek_tail(), Some(&"c"));

    // Second list
    let mut second = LinkedList::new();
    second.push_tail("1");
    second.push_tail("2");
    second.push_tail("3");
    second.push_tail("4");
    assert_eq!(second.peek_head(), Some(&"1"));
    assert_eq!(second.peek_tail(), Some(&"4"));

    // Spliced
    // Postcondition: [1, 2, a, b, c, 3, 4]
    let mut cur = second.cursor_mut();
    cur.move_next(); // 0
    cur.move_next(); // 1
    cur.splice_after(first);
    assert_eq!(second.peek_head(), Some(&"1"));
    assert_eq!(second.peek_tail(), Some(&"4"));

    eprint!("All together now:  [");
    for (i, e) in second.iter().enumerate() {
        eprint!("{e}");
        if i == second.len() - 1 {
            eprintln!("]");
        } else {
            eprint!(", ");
        }
    }

    // Range split
    // Postcondition: [1, 2] [a, b, c, 3, 4]
    let mut cur = second.cursor_mut();
    cur.move_next(); // 0
    cur.move_next(); // 1
    let mut new_list = cur.split_after(); // splits after "2"
    assert_eq!(new_list.peek_head(), Some(&"a"));
    assert_eq!(new_list.peek_tail(), Some(&"4"));

    // Postcondition: [1, 2] [a, b, c] [3, 4]
    let mut new_cur = new_list.cursor_mut();
    new_cur.move_next(); // 0
    new_cur.move_next(); // 1
    new_cur.move_next(); // 2
    let new_back = new_cur.split_after(); // splits again after "c"

    // Reassembles the two numbered lists
    // Postcondition: [1, 2, 3, 4] [a, b, c]
    cur.splice_after(new_back);
    drop(cur);

    // Makes sure everything is as it seems
    assert_eq!(new_list.peek_head(), Some(&"a"));
    assert_eq!(new_list.peek_tail(), Some(&"c"));
    assert_eq!(second.peek_head(), Some(&"1"));
    assert_eq!(second.peek_tail(), Some(&"4"));

    // Prints everything, in case you're a visual learner like me
    eprint!("Split numbers:  [");
    for (i, e) in second.iter().enumerate() {
        eprint!("{e}");
        if i == second.len() - 1 {
            eprintln!("]");
        } else {
            eprint!(", ");
        }
    }
    eprint!("Split letters:  [");
    for (i, e) in new_list.iter().enumerate() {
        eprint!("{e}");
        if i == new_list.len() - 1 {
            eprintln!("]");
        } else {
            eprint!(", ");
        }
    }
    //panic!("Uncomment to show me them beautiful lists");
```
*/

// Creates a raw pointer to some Node
type Link<T> = Option<*mut Node<T>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    prev: Link<T>,
    next: Link<T>,
}
/// # About
/// All operations run in `O(1)` time unless otherwise noted. See the [module-level documentation](`crate::lists::doubly_linked_list`) for more information.
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> LinkedList<T> {
    /** Creates a new list */
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Inserts a node at the head of the list.
    ///
    /// Can be used like a `push(k)` or `add(k)` operation for a stack.
    pub fn push_head(&mut self, element: T) {
        // Creates a NonNull<Node<T>> wrapper from a *mut pointer to the
        // (new) unique heap object
        let new_node_wrapper: *mut Node<T> = Box::into_raw(Box::new(Node {
            data: element,
            prev: None,
            next: None,
        })); // Unsafe

        // If there are already elements in the list...
        if let Some(node) = self.head {
            // Sets the new node's next pointer to the current head
            // SAFETY: New node was just allocated and is not aliased
            unsafe { (*new_node_wrapper).next = self.head };
            // Sets the original head's prev pointer to the new node
            // SAFETY: self.head guaranteed to be non-null
            unsafe { (*node).prev = Some(new_node_wrapper) };
        }
        // Inserts into empty list
        else {
            //println!("Inserts at head");
            // Sets the list's head and tail pointers to the new node
            self.tail = Some(new_node_wrapper);
        }

        // Resets the list's head and increments the list size
        self.head = Some(new_node_wrapper);
        self.len += 1;
    }

    /// Returns a reference to the data at the list's head,
    /// if the list has a head node.
    pub fn peek_head(&self) -> Option<&T> {
        if let Some(node_ptr) = self.head {
            // SAFETY: Creates an immutable reference to a guaranteed non-null
            // allocation with properly initialized data field
            let node = unsafe { &(*node_ptr).data };
            Some(node)
        } else {
            None
        }
    }

    /// Returns a reference to the data at the list's tail, if the list has a tail.
    pub fn peek_tail(&self) -> Option<&T> {
        unsafe {
            if let Some(node_ptr) = self.tail {
                let node = &(*node_ptr).data;
                Some(node)
            } else {
                None
            }
        }
    }

    /// Returns an owned value of the head Node's data.
    /// Use like a `pop()` or a `dequeue()` operation for a stack or queue.
    pub fn pop_head(&mut self) -> Option<T> {
        if let Some(head_ptr) = self.head {
            // Takes ownership
            // SAFETY:
            let boxed_node: Box<Node<T>> = unsafe { Box::from_raw(head_ptr) };
            // Resets the list head to the original head's next pointer
            self.head = boxed_node.next;
            self.len -= 1;
            // Returns the old head's data
            return Some(boxed_node.data);
        }
        None
    }

    /// Inserts a node at the tail of the list. Use like an `enqueue()`
    /// operation for a queue.
    pub fn push_tail(&mut self, element: T) {
        // Creates a raw *mut pointer to the (new) unique heap object
        let new_node_wrapper: *mut Node<T> = Box::into_raw(Box::new(Node {
            data: element,
            prev: None,
            next: None,
        }));

        // Case 1: If there are already elements in the list...
        if let Some(node) = self.tail {
            // Sets the new node's prev pointer to the current tail
            // SAFETY:
            unsafe { (*new_node_wrapper).prev = self.tail };
            // Sets the original tail's next pointer to the new node
            // SAFETY:
            unsafe { (*node).next = Some(new_node_wrapper) };
        }
        // Case 2: Inserts into empty list
        else {
            // Sets the list's head and tail pointers to the new node
            self.head = Some(new_node_wrapper);
        }

        // Resets the list's tail and increments the list size
        self.tail = Some(new_node_wrapper);
        self.len += 1;
    }

    /// Removes and returns the tail node of the list.
    pub fn pop_tail(&mut self) -> Option<T> {
        if let Some(tail_ptr) = self.tail {
            // Takes ownership
            let boxed_node: Box<Node<T>> = unsafe { Box::from_raw(tail_ptr) };

            // Update list tail pointer
            self.tail = boxed_node.prev;

            // If the new tail exists, its next pointer should be None
            if let Some(new_tail_ptr) = self.tail {
                unsafe { (*new_tail_ptr).next = None };
            } else {
                // If there was only one element, we must also update head
                self.head = None;
            }

            self.len -= 1;
            return Some(boxed_node.data); // Return the old tail's data
        }
        None
    }

    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns a Boolean indicating whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Clears all elements from the list in `O(n)` time.
    pub fn clear(&mut self) {
        //while self.iter().next.is_some() {
        //    self.pop_head();
        //}
        while self.pop_head().is_some() {}
    }

    /// Returns an iterator of references to data in the list's nodes as
    /// `list.iter()`.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head,
            // Needed for lifetime tracking
            _marker: std::marker::PhantomData,
        }
    }

    //iter_rev() -> Iter
    //pub fn contains(&T) -> bool {}
    //pub fn find(&T) -> Link<T> {}

    /// Acts like a constructor for a cursor.
    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            cursor: None,
            list: self,
            index: None,
        }
    }
}

pub struct Iter<'a, T> {
    // Link, aka Option<*mut Node<T>>
    next: Link<T>,
    // Ensures correct lifetime tracking
    _marker: std::marker::PhantomData<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.next {
            // SAFETY:
            unsafe {
                self.next = (*current).next; // Move to next node
                Some(&(*current).data) // Return reference to data
            }
        } else {
            None
        }
    }
}

impl<T> Drop for LinkedList<T> {
    // LinkedList destructor works by popping each node into a Box
    // which contains its own Drop semantics and cleans everything
    // up nicely for us.
    fn drop(&mut self) {
        while self.pop_head().is_some() {}
        // Manual implementation
        //unsafe {
        //    let mut current_node_ptr = self.head;
        //    while let Some(ptr) = current_node_ptr {
        //        // Store a pointer to the next Node before deallocating the
        //        // current one
        //        let next_node_ptr = (*ptr.as_ptr()).next;

        //        // Deallocate the current node
        //        let _ = Box::from_raw(ptr.as_ptr());

        //        // Advance the Node pointer
        //        current_node_ptr = next_node_ptr;
        //    }
        //}
    }
}

/// # About
/// Its the great cursor, Charlie Brown!
///
/// See the [module-level documentation](`crate::lists::doubly_linked_list`) for more information.
pub struct CursorMut<'a, T> {
    cursor: Link<T>,
    list: &'a mut LinkedList<T>,
    index: Option<usize>,
}
impl<T> CursorMut<'_, T> {
    /// Returns `true` if the cursor points to Some, indicating that the cursor
    /// is on a valid Node.
    pub fn is_some(&self) -> bool {
        self.cursor.is_some()
    }

    /// Returns `true` if the cursor points to None, indicating that the cursor
    /// is on a ghost node.
    pub fn is_none(&self) -> bool {
        self.cursor.is_none()
    }

    /// Returns the "index" to the current list node in a zero-indexed sequence
    fn _index(&self) -> Option<usize> {
        self.index
    }

    /// Advances the cursor to the next node in the list, moving from head to tail.
    /// If the cursor is at the ghost (pre-head) position and the list is
    /// non-empty, it moves to the head. The function is a no-op if the cursor
    /// is already at the tail or the list is empty.
    pub fn move_next(&mut self) {
        // Case 1) The current position is real, follow the next pointer
        if let Some(cur) = self.cursor {
            // SAFETY: The next pointer is to either Some or None,
            // but always valid.
            self.cursor = unsafe { (*cur).next };
            // If Some, go there
            if self.cursor.is_some() {
                *self.index.as_mut().unwrap() += 1;
            // If None, do nothing
            } else {
                self.index = None;
                // no-op
            }
        // Case 2) The current position isn't real and the list is not
        // empty, the next logical position must be the list's head
        } else if !self.list.is_empty() {
            self.cursor = self.list.head;
            self.index = Some(0)
        // Case 3) The cursor is at the ghost, but the list is empty,
        // so theres nowhere to go and nothing to do
        } else {
            // no-op
        }
    }

    /// Advances the cursor to the previous node in the list,
    /// moving from head to tail. The function is a no-op if the cursor
    /// is already at the head or the list is empty.
    pub fn move_prev(&mut self) {
        // Case 1) The current position is real, follow the prev pointer
        if let Some(cur) = self.cursor {
            // SAFETY: The prev pointer is to either Some or None,
            // but always valid.
            self.cursor = unsafe { (*cur).prev };
            // If Some, go there
            if self.cursor.is_some() {
                *self.index.as_mut().unwrap() -= 1;
            // If None, do nothing
            } else {
                self.index = None;
                // no-op
            }
        // Case 2) The current position isn't real and the list is not empty,
        // the next logical position must be the list's tail
        } else if !self.list.is_empty() {
            self.cursor = self.list.tail;
            self.index = Some(self.list.len - 1)
        // Case 3) The cursor is at the ghost, but theres nowhere to go
        } else {
            // no-op
        }
    }

    /// Returns a mutable reference to the data at the current position.
    /// It is necessary to return a _mutable_ reference to retain the elision
    /// rule checks.
    pub fn current(&mut self) -> Option<&mut T> {
        self.cursor.map(|node| {
            // SAFETY: node only gets dereferenced if cursor is Some,
            // so its assumed to point to a valid and properly initialized Node
            unsafe { &mut (*node).data }
        })
    }

    /// Returns a mutable reference to the data at the next node's position.
    /// It's necessary to return a _mutable_ reference to retain the elision
    /// rule checks.
    pub fn peek_next(&mut self) -> Option<&mut T> {
        let next = if let Some(cur) = self.cursor {
            // SAFETY:
            unsafe { (*cur).next }
        } else {
            // Ghost case, try to use the list's head pointer
            self.list.head
        };
        // Yield the data if the next node exists
        // SAFETY:
        unsafe { next.map(|node| &mut (*node).data) }
    }

    /** Returns a mutable reference to the data at the previous node's position;
    It is necessary to return a _mutable_ reference to retain the elision
    rule checks */
    pub fn peek_prev(&mut self) -> Option<&mut T> {
        unsafe {
            let prev = if let Some(cur) = self.cursor {
                (*cur).prev
            } else {
                // If you're at the ghost, point to the tail
                self.list.tail
            };
            // Yield the data if the prev node exists
            prev.map(|node| &mut (*node).data)
        }
    }

    /// Inserts a node before the cursor;
    ///
    /// - If the cursor is on the ghost node of an empty list,
    ///   the new node becomes the new head and tail;
    ///
    /// - If the cursor is on the ghost node of a non-empty list,
    ///   the new node becomes the new tail;
    ///
    /// - If the cursor is on the head, the new node is the new head;
    ///
    /// Precondition:
    ///
    /// ```text
    ///     self.head -> A <-> C <- self.tail
    ///                        ^
    ///                     cursor
    /// ```
    ///
    /// Postcondition:
    ///
    /// ```text
    ///     self.head -> A <-> B <-> C <- self.tail
    ///                              ^
    ///                           cursor
    /// ```
    pub fn insert_before(&mut self, element: T) {
        let new_node_wrapper: *mut Node<T> = Box::into_raw(Box::new(Node {
            data: element,
            prev: None,
            next: None,
        }));

        // Case 1) The cursor is at the ghost of an empty list;
        // new head/tail
        if self.cursor.is_none() && self.list.head.is_none() {
            self.list.head = Some(new_node_wrapper);
            self.list.tail = Some(new_node_wrapper);
        }
        // Case 2) The cursor is at the ghost of a non-empty list; new tail
        else if self.cursor.is_none() && self.list.head.is_some() {
            // Update the old tail's next pointer
            let old_tail = self.list.tail.unwrap();
            unsafe {
                (*old_tail).next = Some(new_node_wrapper);
            }

            // Update the new node's prev pointer
            unsafe {
                (*new_node_wrapper).prev = Some(old_tail);
            }

            // Update the list's tail
            self.list.tail = Some(new_node_wrapper);
        }
        // Case 3) The cursor is at the head of a non-empty list; new head
        else if self.cursor == self.list.head && self.list.head.is_some() {
            // Update the old head's prev pointer
            let old_head = self.list.head.unwrap();
            unsafe {
                (*old_head).prev = Some(new_node_wrapper);
            }

            // Update the new node's next pointer
            unsafe {
                (*new_node_wrapper).next = Some(old_head);
            }

            // Update the list.head pointer
            self.list.tail = Some(new_node_wrapper);
        }
        // Case 4) The cursor is somewhere between head+1 and tail of
        // a non-empty list; reset all four pointers
        else {
            unsafe {
                // Capture the node prior to the cursor node
                // and start orgy of pointer swapping
                let cursor_node = self.cursor.unwrap();
                let old_prev = (*cursor_node).prev.unwrap();
                (*old_prev).next = Some(new_node_wrapper);
                (*new_node_wrapper).prev = Some(old_prev);
                (*new_node_wrapper).next = Some(cursor_node);
                (*cursor_node).prev = Some(new_node_wrapper);
            }
        }

        // All cases
        // - Adjust the cursor's index
        // - Increment list len
        if self.index.is_some() {
            let mut new_index = self.index.unwrap();
            new_index += 1;
            self.index = Some(new_index);
        } else {
            self.index = Some(0)
        };
        self.list.len += 1;
    }

    /// Inserts a node after the cursor;
    ///
    /// - If the cursor is on the ghost node of an empty list,
    ///   the new node becomes the new head and tail;
    ///
    /// - If the cursor is on the ghost node of a non-empty list,
    ///   the new node becomes the new head;
    ///
    /// - If the cursor is on the tail, the new node is the new tail;
    ///
    /// Precondition:
    ///
    /// ```text
    ///     self.head -> A <-> C <- self.tail
    ///                  ^
    ///               cursor
    /// ```
    ///
    /// Postcondition:
    ///
    /// ```text
    ///     self.head -> A <-> B <-> C <- self.tail
    ///                  ^
    ///               cursor
    /// ```
    pub fn insert_after(&mut self, element: T) {
        let new_node_wrapper: *mut Node<T> = Box::into_raw(Box::new(Node {
            data: element,
            prev: None,
            next: None,
        }));

        // Case 1) The cursor is at the ghost node in an empty list;
        // new head/tail
        if self.cursor.is_none() && self.list.head.is_none() {
            self.list.head = Some(new_node_wrapper);
            self.list.tail = Some(new_node_wrapper);
        }
        // Case 2) The cursor is at the ghost node in a non-empty list; new head
        else if self.cursor.is_none() && self.list.head.is_some() {
            // Capture the old head node
            let old_head = self.list.head.unwrap();

            // Update the old_head.prev to new node
            unsafe {
                (*old_head).prev = Some(new_node_wrapper);
            }

            // Update the new node.next to old_head
            unsafe {
                (*new_node_wrapper).next = Some(old_head);
            }

            // Update the list's head
            self.list.head = Some(new_node_wrapper);
        }
        // Case 3) The cursor is at the tail of a non-empty list; new tail
        else if self.cursor == self.list.tail && self.list.tail.is_some() {
            // Capture the old tail
            let old_tail = self.list.tail.unwrap();

            // Update the old_tail.next to the new_node
            unsafe {
                (*old_tail).next = Some(new_node_wrapper);
            }

            // Update the new_node.prev to old_tail
            unsafe {
                (*new_node_wrapper).prev = Some(old_tail);
            }

            // Update the list.tail
            self.list.tail = Some(new_node_wrapper);
        }
        // Case 4) The cursor is somewhere between head and tail-1 of
        // a non-empty list
        else {
            unsafe {
                // Capture the node after the current cursor node
                // and start orgy of pointer swapping
                let cursor_node = self.cursor.unwrap();
                let old_next = (*cursor_node).next.unwrap();
                (*old_next).prev = Some(new_node_wrapper);
                (*new_node_wrapper).prev = Some(cursor_node);
                (*new_node_wrapper).next = Some(old_next);
                (*cursor_node).next = Some(new_node_wrapper);
            }
        }

        // All cases
        // - Increment list len
        self.list.len += 1;
    }

    /// Removes and returns the data under the cursor's current position
    /// and moves the cursor back one position
    ///
    /// Precondition:
    ///
    /// ```text
    ///     self.head -> A <-> B <-> C <-> D <- self.tail
    ///                              ^
    ///                           cursor
    /// ```
    ///
    /// Postcondition:
    ///
    /// ```text
    ///     self.head -> A <-> B <-> D <- self.tail
    ///                        ^
    ///                     cursor
    /// ```
    pub fn remove_current(&mut self) -> Option<T> {
        unsafe {
            // Case 1) You're at the ghost, do nothing
            //if self.current().is_none() { return None; }

            // Case 2) You're in a non-empty list
            let node_data = if let Some(node_ptr) = self.cursor {
                // 2.1 You're at the head
                if (*node_ptr).prev.is_none() {
                    // Reset the head and its pointers
                    if let Some(next_node) = (*node_ptr).next {
                        (*next_node).prev = None;
                        self.list.head = (*node_ptr).next;
                    }
                }
                // 2.2 You're at the tail
                else if (*node_ptr).next.is_none() {
                    // Reset the head and its pointers
                    if let Some(next_node) = (*node_ptr).prev {
                        (*next_node).next = None;
                        self.list.tail = (*node_ptr).prev;
                    }

                // 2.3 You're somewhere mid-list
                } else {
                    // Reset the previous node's next
                    if let Some(prev_node) = (*node_ptr).prev {
                        (*prev_node).next = (*node_ptr).next;
                    }
                    // Resent the next node's prev
                    if let Some(next_node) = (*node_ptr).next {
                        (*next_node).prev = (*node_ptr).prev;
                    }
                };
                // For all non-empty list cases,
                // take the cursor's underlying raw pointer data
                // and adjust the cursor's position
                self.move_prev(); // handles index placement too
                let data: Box<Node<T>> = Box::from_raw(node_ptr);
                Some(data.data)

            // Just in case self.cursor is None (ghost)
            } else {
                None
            };
            // Decrement the underlying list's length and return
            self.list.len -= 1;
            node_data
        }
    }

    /// Returns a new list containing all nodes before the cursor,
    /// modifying `self` to become all nodes after (and including) the cursor.
    ///
    /// Precondition:
    ///
    /// ```text
    ///     self.head -> A <-> B <-> C <-> D <- self.tail
    ///                              ^
    ///                           cursor
    /// ```
    ///
    /// Postcondition:
    ///
    /// ```text
    ///     self.head -> C <-> D <- self.tail
    ///                        ^
    ///                      cursor
    ///
    ///     return.head -> A <-> B <- return.tail
    /// ```
    pub fn split_before(&mut self) -> LinkedList<T> {
        // Case 1) The cursor is on an element of a non-empty list
        if let Some(node_ptr) = self.cursor {
            unsafe {
                // Captures current state of self
                let old_len = self.list.len;
                let old_index = self.index.unwrap();
                let prev = (*node_ptr).prev;

                // What self will become
                let new_len = old_len - old_index;
                let new_head = self.cursor;
                let new_index = Some(0);

                // What the output will become
                let output_len = old_len - new_len;
                let output_head = self.list.head;
                let output_tail = prev;

                // If the cursor is NOT at a ghost node,
                // break pointer links at split:
                // - cursor_next is the new list's head
                // - node_ptr is the origial list's tail
                if let Some(prev) = prev {
                    (*node_ptr).prev = None;
                    (*prev).next = None;
                }

                // Modify self
                self.list.len = new_len;
                self.list.head = new_head;
                self.index = new_index;

                // Return the new list
                LinkedList {
                    head: output_head,
                    tail: output_tail,
                    len: output_len,
                    //_marker: PhantomData,
                }
            }

        // Case 2) The cursor is on the ghost node;
        // replace self with an empty list, return the original list
        // EDIT: replace self with None, return the original list
        } else {
            //std::mem::replace(self.list, LinkedList::new())
            std::mem::take(self.list)
        }
    }

    /// Returns a new list that includes all nodes after the cursor,
    /// modifying self to become all nodes before (and including) the cursor
    ///
    /// Precondition:
    ///
    /// ```text
    ///     self.head -> A <-> B <-> C <-> D <- self.tail
    ///                        ^
    ///                      cursor
    /// ```
    ///
    /// Postcondition:
    ///
    /// ```text
    ///     self.head -> A <-> B <- self.tail
    ///                        ^
    ///                      cursor
    ///
    ///     return.head -> C <-> D <- return.tail
    /// ```
    pub fn split_after(&mut self) -> LinkedList<T> {
        // Case 1) The cursor is not on the ghost node
        if let Some(node_ptr) = self.cursor {
            unsafe {
                // Captures current state of self
                let old_len = self.list.len;
                let old_index = self.index.unwrap();
                let cursor_next = (*node_ptr).next;

                // What self will become
                let new_len = old_index + 1;
                let new_tail = self.cursor;
                let new_index = Some(old_index);

                // What the output will become
                let output_len = old_len - new_len;
                let output_head = cursor_next;
                let output_tail = self.list.tail;

                // If the cursor is NOT at the end of the list,
                // break pointer links at split:
                // - cursor_next is the new list's head
                // - node_ptr is the origial list's tail
                if let Some(next_ptr_unwrapped) = cursor_next {
                    (*node_ptr).next = None;
                    (*next_ptr_unwrapped).prev = None;
                }

                // Modify self
                self.list.len = new_len;
                self.list.tail = new_tail;
                self.index = new_index;

                // Return the new list
                LinkedList {
                    head: output_head,
                    tail: output_tail,
                    len: output_len,
                    //_marker: PhantomData,
                }
            }

        // Case 2) The cursor is on the ghost node;
        // replace self with an empty list, return the original list
        // EDIT: replace self with None, return the original list
        } else {
            //std::mem::replace(self.list, LinkedList::new())
            std::mem::take(self.list)
        }
    }

    /// Splices in a new list between the cursor node and the previous node
    ///
    /// Precondition:
    ///
    /// ```text
    ///     self.head -> A <-> B <-> C <- self.tail
    ///                        ^
    ///                      cursor
    ///
    ///     input.head -> 1 <-> 2 <- input.tail
    /// ```
    ///
    /// Postcondition:
    ///
    /// ```text
    ///     self.head -> A <-> 1 <-> 2 <-> B <-> C <- self.back
    ///                                    ^
    ///                                  cursor
    /// ```
    pub fn splice_before(&mut self, mut input: LinkedList<T>) {
        unsafe {
            // Case 1) Input list is empty; do nothing. Easy!
            if input.is_empty() {
            }
            // Case 2) Self is non-empty
            else if let Some(cur) = self.cursor {
                // Per the Too Many Linked LinkedLists Book:
                // We can either `take` the input's pointers or `mem::forget`
                // it [sic]. Using `take` is more responsible in case we ever
                // do custom allocators or something that also needs to be
                // cleaned up!
                let input_head = input.head.take().unwrap();
                let input_tail = input.tail.take().unwrap();

                // 2.1) Cursor is somewhere inside a non-empty self;
                // puttin a list inside another list
                if let Some(prev) = (*cur).prev {
                    (*prev).next = Some(input_head);
                    (*input_head).prev = Some(prev);
                    (*cur).prev = Some(input_tail);
                    (*input_tail).next = Some(cur);

                // 2.2) Cursor is at self.head, prepend self with input list
                } else {
                    (*cur).prev = Some(input_tail);
                    (*input_tail).next = Some(cur);
                    self.list.head = Some(input_head);
                }
                // Index moves forward by input length
                *self.index.as_mut().unwrap() += input.len;

            // Case 3) Cursor is on the ghost node for a non-empty list,
            // prepend self with input
            } else if let Some(_back) = self.list.tail {
                let input_head = input.head.take().unwrap();
                let input_tail = input.tail.take().unwrap();
                self.list.head = Some(input_head);
                self.list.tail = Some(input_tail);
            } else {
                // Self is empty, swap in the input list, remain on the ghost
                std::mem::swap(self.list, &mut input);
            }

            self.list.len += input.len;
            // Not necessary but Polite To Do
            input.len = 0; // Input dropped at end of block
        }
    }

    /// Splices in a new list between the cursor node and the next node
    ///
    /// Precondition
    ///
    /// ```text
    ///     self.head -> A <-> B <-> C <- self.tail
    ///                        ^
    ///                     cursor
    ///
    ///     input.head -> 1 <-> 2 <- input.tail
    /// ```
    ///
    /// Postcondition
    ///
    /// ```text
    ///     self.head -> A <-> B <-> 1 <-> 2 <-> C <- self.tail
    ///                        ^
    ///                     cursor
    /// ```
    pub fn splice_after(&mut self, mut input: LinkedList<T>) {
        unsafe {
            // Case 1) Input list is empty, do nothing! Easy.
            if input.is_empty() {
            }
            // Case 2) Self is non-empty
            else if let Some(cur) = self.cursor {
                // From the Too Many Linked LinkedList book:
                // We can either `take` the input's pointers or `mem::forget`
                // it [sic]. Using `take` is more responsible in case we ever
                // do custom allocators or something that also needs to be
                // cleaned up!
                let input_head = input.head.take().unwrap();
                let input_tail = input.tail.take().unwrap();

                // 2.1) Cursor is somewhere in a non-empty list; swap pointers ;)
                if let Some(next) = (*cur).next {
                    (*next).prev = Some(input_tail);
                    (*input_tail).next = Some(next);
                    (*cur).next = Some(input_head);
                    (*input_head).prev = Some(cur);

                // 2.2) Cursor is at tail, append self
                } else {
                    (*cur).next = Some(input_head);
                    (*input_head).prev = Some(cur);
                    self.list.tail = Some(input_tail);
                }

            // Case 3) Cursor is at the ghost node in a non-empty self, prepend self
            } else if let Some(head) = self.list.head {
                let input_head = input.head.take().unwrap();
                let input_tail = input.tail.take().unwrap();

                (*head).next = Some(input_tail);
                (*input_tail).prev = Some(head);
                self.list.head = Some(input_head);

            // Case 4) Cursor is at the ghost node for an empty self, do a swaperoo
            } else {
                std::mem::swap(self.list, &mut input);
            }
            // Increase the list's lenth value
            self.list.len += input.len;
        }
    }
}

#[cfg(test)]
#[allow(clippy::items_after_test_module)] // Example runner comes after tests
mod list_tests {
    use super::*;

    #[test]
    fn list_test() {
        use crate::sequences::doubly_linked_list::{CursorMut, LinkedList};

        // Operational tests
        // Creates a new doubly-linked list
        // and pushes some elements to it
        let mut list = LinkedList::new();
        assert!(list.is_empty()); // New list is empty
        list.push_head("a");
        list.push_head("b");
        list.push_head("c"); // Postcondition: [c, b, a]
        assert!(!list.is_empty()); // LinkedList now has stuff

        // Tests the cursor which starts at a non-existant "ghost" node
        // Remember: only one mutable reference at a time!
        let mut cur: CursorMut<'_, &str> = list.cursor_mut();
        cur.move_next(); // 0
        cur.move_next(); // 1
        cur.move_next(); // 2
                         //assert_eq!(cur.index(), Some(2));
        assert_eq!(cur.current(), Some(&mut "a"));

        // Tests the pop operations
        let mut popped_head: &str = list.pop_head().unwrap();
        assert_eq!(popped_head, "c");
        popped_head = list.pop_head().unwrap();
        assert_eq!(popped_head, "b");
        // Postcondition: [a]

        list.push_head("b");
        list.push_head("c");
        // Postcondition: [c, b, a]

        assert_eq!(list.len, 3);

        let mut popped_tail: &str = list.pop_tail().unwrap();
        assert_eq!(popped_tail, "a");
        popped_tail = list.pop_tail().unwrap();
        assert_eq!(popped_tail, "b");
        // Postcondition: [c]

        assert_eq!(list.len(), 1);

        // Adds some elements for the pointer tests
        list.push_tail("b");
        list.push_tail("a");
        // Postcondition: [c, b, a]

        assert_eq!(list.len(), 3);

        // Creates a new doubly-linked list
        // and pushes some elements to it
        let mut list = LinkedList::new();
        list.push_head("a");
        list.push_head("b");
        list.push_head("c"); // Postcondition: [c, b, a]

        // Checks that head -> c
        let head_ptr: *mut Node<&str> = list.head.unwrap();
        let head: &str = unsafe { (*head_ptr).data }; // Unsafe deref
        assert_eq!(head, "c");

        // Checks that head.next -> b
        let next_ptr: *mut Node<&str> = unsafe { (*list.head.unwrap()).next.unwrap() };
        let next: &str = unsafe { (*next_ptr).data }; // Unsafe deref
        assert_eq!(next, "b");

        // Checks that head.prev -> None
        let prev_ptr: Option<*mut Node<&str>> = unsafe { (*list.head.unwrap()).prev };
        assert!(prev_ptr.is_none());

        // Checks that b.prev -> head
        let prev_ptr: *mut Node<&str> = unsafe { (*next_ptr).prev.unwrap() };
        let prev: &str = unsafe { (*prev_ptr).data }; // Unsafe deref
        assert_eq!(prev, "c");

        // Checks that tail -> a
        let tail_ptr: *mut Node<&str> = list.tail.unwrap();
        let tail: &str = unsafe { (*tail_ptr).data }; // Unsafe deref
        assert_eq!(tail, "a");

        // Checks that tail.prev -> b
        let prev_ptr: *mut Node<&str> = unsafe { (*tail_ptr).prev.unwrap() };
        let prev: &str = unsafe { (*prev_ptr).data }; // Unsafe deref
        assert_eq!(prev, "b");

        // Checks that tail.next -> None
        let next_ptr: Option<*mut Node<&str>> = unsafe { (*list.tail.unwrap()).next };
        assert!(next_ptr.is_none());

        // Clears list and checks that its empty
        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn cursor_test() {
        use crate::sequences::doubly_linked_list::{CursorMut, LinkedList};

        // Creates a new doubly-linked list
        // and pushes some elements to it
        let mut list = LinkedList::new();
        assert!(list.is_empty()); // New list is empty
        list.push_head("a");
        list.push_head("b");
        list.push_head("c"); // Postcondition: [c, b, a]
        assert!(!list.is_empty()); // LinkedList now has stuff

        // Tests the cursor which starts at a non-existant "ghost" node
        // Remember: only one mutable reference at a time!
        let mut cur: CursorMut<'_, &str> = list.cursor_mut();
        cur.move_next(); // 0
        cur.move_next(); // 1
        cur.move_next(); // 2
        assert_eq!(cur._index(), Some(2));
        assert_eq!(cur.current(), Some(&mut "a"));

        cur.move_prev(); // 1
        cur.move_prev(); // 0
        assert_eq!(cur._index(), Some(0));
        assert_eq!(cur.current(), Some(&mut "c"));

        // Moves beyond the list.head to the ghost node
        cur.move_prev(); // Boo! 👻
        assert_eq!(cur._index(), None);
        assert_eq!(cur.current(), None);

        // Wraps back around!
        cur.move_prev(); // 2
        assert_eq!(cur._index(), Some(2));
        assert_eq!(cur.current(), Some(&mut "a"));

        // Next is the ghost, but peek doesn't change the current position
        let peek = cur.peek_next();
        assert_eq!(peek, None);
        assert_eq!(cur._index(), Some(2));
        assert_eq!(cur.current(), Some(&mut "a"));

        let peek = cur.peek_prev();
        assert_eq!(peek, Some(&mut "b"));
        assert_eq!(cur._index(), Some(2));
        assert_eq!(cur.current(), Some(&mut "a"));
    }

    #[test]
    fn split_after() {
        use crate::sequences::doubly_linked_list::LinkedList;

        // Creates a new doubly-linked list
        // and pushes some elements to it
        let mut list = LinkedList::new();
        list.push_head("a");
        list.push_head("b");
        list.push_head("c");
        list.push_head("d");
        list.push_head("e"); // Postcondition: [e, d, c, b, a]

        let mut cur = list.cursor_mut();
        cur.move_next(); // 0
        cur.move_next(); // 1
        cur.move_next(); // 2

        // Split the list!
        let mut a = cur.split_after();

        // The new list's head is now b
        let tail = a.pop_head().unwrap();
        assert_eq!(tail, "b");

        // The original list's tail is now c
        let tail = list.pop_tail().unwrap();
        assert_eq!(tail, "c");
    }

    #[test]
    fn split_before() {
        use crate::sequences::doubly_linked_list::LinkedList;

        // Creates a new doubly-linked list
        // and pushes some elements to it
        let mut list = LinkedList::new();
        list.push_head("a");
        list.push_head("b");
        list.push_head("c");
        list.push_head("d");
        list.push_head("e"); // Postcondition: [e, d, c, b, a]

        let mut cur = list.cursor_mut();
        cur.move_next(); // 0
        cur.move_next(); // 1
        cur.move_next(); // 2

        // Split the list!
        let mut new = cur.split_before();

        // The new list's head is now b
        let tail = new.pop_tail().unwrap();
        assert_eq!(tail, "d");

        // The original list's tail is now c
        let tail = list.pop_head().unwrap();
        assert_eq!(tail, "c");
    }

    #[test]
    fn splice_before() {
        use crate::sequences::doubly_linked_list::LinkedList;

        // Creates a new doubly-linked list
        // and pushes some elements to it
        let mut list = LinkedList::<&str>::new();
        list.push_tail("a");
        list.push_tail("b");
        list.push_tail("c"); // Postcondition: [a, b, c]

        let mut cur = list.cursor_mut();
        cur.move_next(); // a
        cur.move_next(); // b
        cur.move_next(); // c

        let mut new_list = LinkedList::<&str>::new();
        new_list.push_tail("1");
        new_list.push_tail("2"); // Postcondition: [1, 2]

        // Splice the list!
        cur.splice_before(new_list); // Postcondition: [a, b, 1, 2, c]

        let mut tail = list.pop_tail();
        assert_eq!(tail, Some("c"));
        tail = list.pop_tail();
        assert_eq!(tail, Some("2"));
        tail = list.pop_tail();
        assert_eq!(tail, Some("1"));
        tail = list.pop_tail();
        assert_eq!(tail, Some("b"));
    }

    #[test]
    fn splice_after() {
        // Creates a new doubly-linked list
        // and pushes some elements to it
        let mut list = LinkedList::<&str>::new();
        list.push_head("a");
        list.push_head("b");
        list.push_head("c"); // Postcondition: [c, b, a]

        let mut cur = list.cursor_mut();
        cur.move_next(); // c
        cur.move_next(); // b

        let mut new_list = LinkedList::<&str>::new();
        new_list.push_head("1");
        new_list.push_head("2"); // Postcondition: [2, 1]

        // Splice the list!
        cur.splice_after(new_list); // Postcondition: [c, b, 2, 1, a]

        let mut tail = list.pop_tail();
        assert_eq!(tail, Some("a"));
        tail = list.pop_tail();
        assert_eq!(tail, Some("1"));
        tail = list.pop_tail();
        assert_eq!(tail, Some("2"));
        tail = list.pop_tail();
        assert_eq!(tail, Some("b"));
    }

    #[test]
    fn remove_current() {
        use crate::sequences::doubly_linked_list::LinkedList;

        // Creates a new doubly-linked list
        // and pushes some elements to it
        let mut list = LinkedList::<&str>::new();
        list.push_tail("a");
        list.push_tail("b");
        list.push_tail("c"); // Postcondition: [a, b, c]
        assert_eq!(list.len(), 3);

        let mut cur = list.cursor_mut();
        cur.move_next(); // a
        assert_eq!(cur.index, Some(0));
        cur.move_next(); // b
        assert_eq!(cur.index, Some(1));

        // Remove the node
        cur.remove_current(); // Postcondition: [a, c]

        // Tests that the remove operation backs
        // the cursor up and decrements the list length
        assert_eq!(cur.index, Some(0));
        assert_eq!(list.len(), 2);

        let mut head = list.pop_head();
        assert_eq!(head, Some("a"));
        head = list.pop_head();
        assert_eq!(head, Some("c"));
        head = list.pop_head();
        assert_eq!(head, None);

        // New list test
        let mut list = LinkedList::<&str>::new();
        list.push_tail("P");
        list.push_tail("E");
        list.push_tail("T");
        list.push_tail("E");
        list.push_tail("R"); // Postcondition [P, E, T, E, R]

        let mut cur = list.cursor_mut();
        cur.move_prev(); // 4
                         // Removes tail
        let r = cur.remove_current().unwrap(); // Postcondition [P, E, T, E]
        assert_eq!(r, "R");
        cur.move_next(); // boo! 👻
        cur.move_next(); // 0
                         // Removes head
        let p = cur.remove_current().unwrap(); // Postcondition [E, T, E]
        assert_eq!(p, "P");
        assert_eq!(list.peek_head(), Some(&"E"));
        assert_eq!(list.peek_tail(), Some(&"E"));
    }

    #[test]
    fn insert_before() {
        use crate::sequences::doubly_linked_list::LinkedList;

        // Creates a new doubly-linked list
        // and pushes some elements to it
        let mut list = LinkedList::<&str>::new();
        list.push_tail("a");
        list.push_tail("c"); // Postcondition: [a, c]
        assert_eq!(list.len(), 2);

        let mut cur = list.cursor_mut();
        cur.move_next(); // a
        assert_eq!(cur.index, Some(0));
        cur.move_next(); // b
        assert_eq!(cur.index, Some(1));

        // Insert a node
        cur.insert_before("b"); // Postcondition: [a, b, c]

        // Tests that the insert operation increments
        // the cursor's index and list length
        assert_eq!(cur.index, Some(2));
        assert_eq!(list.len(), 3);

        // Pointer tests
        // Checks that head -> a
        let head_ptr: *mut Node<&str> = list.head.unwrap();
        let head: &str = unsafe { (*head_ptr).data }; // Unsafe deref
        assert_eq!(head, "a");

        // Checks that head.next -> b
        let next_ptr: *mut Node<&str> = unsafe { (*list.head.unwrap()).next.unwrap() };
        let next: &str = unsafe { (*next_ptr).data }; // Unsafe deref
        assert_eq!(next, "b");

        // Checks that b.next -> c
        let next_ptr: *mut Node<&str> = unsafe { (*next_ptr).next.unwrap() };
        let next: &str = unsafe { (*next_ptr).data }; // Unsafe deref
        assert_eq!(next, "c");

        // Checks that b.next -> tail
        let tail_ptr = list.tail.unwrap();
        let tail_data = unsafe { (*tail_ptr).data };
        assert_eq!(next, tail_data);

        // Functional tests
        let mut head = list.pop_head();
        assert_eq!(head, Some("a"));
        head = list.pop_head();
        assert_eq!(head, Some("b"));
        head = list.pop_head();
        assert_eq!(head, Some("c"));
    }

    #[test]
    fn insert_after() {
        use crate::sequences::doubly_linked_list::LinkedList;

        // Creates a new doubly-linked list
        // and pushes some elements to it
        let mut list = LinkedList::<&str>::new();
        list.push_tail("a");
        list.push_tail("c"); // Postcondition: [a, c]
        assert_eq!(list.len(), 2);

        let mut cur = list.cursor_mut();
        cur.move_next(); // a
        assert_eq!(cur.index, Some(0));

        // Insert a node
        cur.insert_after("b"); // Postcondition: [a, b, c]

        // Tests that the insert operation does NOT
        // increment the cursor's index, but does increment
        // the list length
        assert_eq!(cur.index, Some(0));
        assert_eq!(list.len(), 3);

        // Pointer tests
        // Checks that head -> a
        let head_ptr: *mut Node<&str> = list.head.unwrap();
        let head: &str = unsafe { (*head_ptr).data }; // Unsafe deref
        assert_eq!(head, "a");

        // Checks that head.next -> b
        let next_ptr: *mut Node<&str> = unsafe { (*list.head.unwrap()).next.unwrap() };
        let next: &str = unsafe { (*next_ptr).data }; // Unsafe deref
        assert_eq!(next, "b");

        // Checks that b.next -> c
        let next_ptr: *mut Node<&str> = unsafe { (*next_ptr).next.unwrap() };
        let next: &str = unsafe { (*next_ptr).data }; // Unsafe deref
        assert_eq!(next, "c");

        // Checks that b.next -> tail
        let tail_ptr = list.tail.unwrap();
        let tail_data = unsafe { (*tail_ptr).data };
        assert_eq!(next, tail_data);

        // Functional tests
        let mut head = list.pop_head();
        assert_eq!(head, Some("a"));
        head = list.pop_head();
        assert_eq!(head, Some("b"));
        head = list.pop_head();
        assert_eq!(head, Some("c"));
    }
}

pub fn example() {
    //use super::*;

    // First
    let mut first = LinkedList::new();
    first.push_tail("a");
    first.push_tail("b");
    first.push_tail("c");

    print!("First:  [");
    for (i, e) in first.iter().enumerate() {
        print!("{e}");
        if i == first.len() - 1 {
            print!("]");
        } else {
            print!(", ");
        }
    }
    println!();

    // Second
    let mut second = LinkedList::new();
    second.push_tail("1");
    second.push_tail("2");
    second.push_tail("3");
    second.push_tail("4");

    print!("Second:  [");
    for (i, e) in second.iter().enumerate() {
        print!("{e}");
        if i == second.len() - 1 {
            println!("]");
        } else {
            print!(", ");
        }
    }

    // Spliced
    let mut cur = second.cursor_mut();
    cur.move_next(); // 0
    cur.move_next(); // 1
    cur.splice_after(first);
    // Postcondition: [1, 2, a, b, c, 3, 4]

    print!("Spliced:  [");
    for (i, e) in second.iter().enumerate() {
        print!("{e}");
        if i == second.len() - 1 {
            println!("]");
        } else {
            print!(", ");
        }
    }

    // Range split
    let mut cur = second.cursor_mut();
    cur.move_next(); // 0
    cur.move_next(); // 1
    let mut new_front = cur.split_after(); // splits after "2"
                                           // Postcondition: [1, 2] [a, b, c, 3, 4]
    let mut new_cur = new_front.cursor_mut();
    new_cur.move_next(); // 0
    new_cur.move_next(); // 1
    new_cur.move_next(); // 2
    let new_back = new_cur.split_after(); // splits again after "c"
                                          // Postcondition: [1, 2] [a, b, c] [3, 4]
    cur.splice_after(new_back); // reassembles the numbers
                                // Postcondition: [1, 2, 3, 4] [a, b, c]

    print!("Split numbers:  [");
    for (i, e) in second.iter().enumerate() {
        print!("{e}");
        if i == second.len() - 1 {
            println!("]");
        } else {
            print!(", ");
        }
    }
    print!("Split letters:  [");
    for (i, e) in new_front.iter().enumerate() {
        print!("{e}");
        if i == new_front.len() - 1 {
            println!("]");
        } else {
            print!(", ");
        }
    }

    //Interleaved
    let mut first = LinkedList::new();
    first.push_tail("a");
    first.push_tail("b");
    first.push_tail("c");

    let mut second = LinkedList::new();
    second.push_tail("1");
    second.push_tail("2");
    second.push_tail("3");
    second.push_tail("4");
    second.push_tail("5");

    let max = std::cmp::max(first.len(), second.len());

    let mut first_iter = first.iter();
    let mut second_iter = second.iter();
    print!("Interleaved:  [");
    for (i, _) in (0..max).enumerate() {
        // If the first list has a value, print it
        if let Some(s) = second_iter.next() {
            print!("{s}");
            if i < max - 1 {
                print!(", ");
            }
        }
        // If the second list has a value, print it
        if let Some(f) = first_iter.next() {
            print!("{f}");
            if i < max - 1 {
                print!(", ");
            }
        }
    }
    println!("]");
}
