////////////////////////////////////////////////////
/** An unsafe, owned, generic, doubly-linked list */
////////////////////////////////////////////////////

// NonNull is essentially a wrapper for *mut
// You can deref a NonNull with as_ptr()
//use std::ptr::NonNull;
// Recommended when using raw pointers over generic types
//use std::marker::PhantomData;

// Creates a raw pointer to some Node
type Link<T> = Option<*mut Node<T>>;
//type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    prev: Link<T>,
    next: Link<T>,
}
/// The List contains operations for a basic, unsorted, doubly-linked
/// list that can act as a stack or a queue with operations to insert and remove
/// from both the front and tail of the list in O(1) time;
/// The List also supports a cursor which supports operations for a positional list 
/// to support applications that require a sorted list;
/** The basic list's API includes the following operations:
 - pub new() -> List<T>
 - pub push_head(T) aka add(k)
 - pub push_tail(T) aka enqueue(k)
 - pub pop_head() -> Option<T> aka pop(k)/dequeue(k)
 - pub pop_tail() -> Option<T>

The positional list adds the following operations:
 - peek_next() -> &T
 - peek_prev() -> &T
 - find(&T) -> Link<T>
 - insert_ith(Node<T>, p)
 - insert_after(Node<T>, p)
 - insert_before(Node<T>, p)
 - set(Node<T>, p) -> Node<T>
 - remove_ith(p) -> Node<T>
 - remove_after(p) -> Node<T>
 - remove_before(p) -> Node<T>
 - peek_ith(p) -> &Node<T>

All lists contain the following non-public utilities
 - contains(&T) -> bool
 - head() -> Option<&T>
 - tail() -> Option<&T>
 - len() -> usize
 - is_empty() -> bool
 - clear()
 - iter() -> Iter
 - iter_rev() -> Iter

Positional List (Sorted List Support)
*/
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}
impl<T> List<T> {
    // Creates a new list
    pub fn new() -> List<T> {
        List {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /** Inserts a node at the head of the list in O(1) time;
    Use like a push(k) or add(k) operation for a stack */
    pub fn push_head(&mut self, element: T) {
        unsafe {
            // Creates a NonNull<Node<T>> wrapper from a *mut pointer to the (new) unique heap object
            let new_node_wrapper: *mut Node<T> =
                Box::into_raw(Box::new(Node {
                    data: element,
                    prev: None,
                    next: None,
                })); // Unsafe

            // If there are already elements in the list...
            if let Some(node) = self.head {
                //println!("Inserts new head");
                // Sets the new node's next pointer to the current head
                (*new_node_wrapper).next = self.head;
                // Sets the original head's prev pointer to the new node
                (*node).prev = Some(new_node_wrapper); // Unsafe
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
            return;
        }
    }

    /** Returns an owned value of the head Node's data in O(1) time;
    Use like a pop() or a dequeue() operation for a stack or queue */
    pub fn pop_head(&mut self) -> Option<T> {
        if let Some(head_ptr) = self.head {
            let boxed_node: Box<Node<T>> = unsafe { Box::from_raw(head_ptr) }; // takes ownership
            self.head = boxed_node.next; // Resets the List head to the original head's next pointer
            self.len -= 1;
            return Some(boxed_node.data); // Returns the old head's data
        }
        None
    }

    /** Inserts a node at the tail of the list in O(1) time;
    Use like an enqueue(k) operation for a queue */
    pub fn push_tail(&mut self, element: T) {
        unsafe {
            // Creates a NonNull<Node<T>> wrapper from a *mut pointer to the (new) unique heap object
            let new_node_wrapper: *mut Node<T> =
                Box::into_raw(Box::new(Node {
                    data: element,
                    prev: None,
                    next: None,
                })); // Unsafe

            // If there are already elements in the list...
            if let Some(node) = self.tail {
                println!("Inserts new tail");
                // Sets the new node's prev pointer to the current tail
                (*new_node_wrapper).prev = self.tail;
                // Sets the original tail's next pointer to the new node
                (*node).next = Some(new_node_wrapper); // Unsafe
            }

            // Inserts into empty list
            else {
                println!("Inserts at tail");
                // Sets the list's head and tail pointers to the new node
                self.head = Some(new_node_wrapper);
            }

            // Resets the list's tail and increments the list size
            self.tail = Some(new_node_wrapper);
            self.len += 1;
            return;
        }
    }

    /** Removes and returns the tail node of the list in O(1) time */
    //pub fn pop_tail(&mut self) -> Option<T> {
    //    if let Some(tail_ptr) = self.tail {
    //        let boxed_node: Box<Node<T>> = unsafe { Box::from_raw(tail_ptr) }; // takes ownership
    //        self.tail = boxed_node.prev; // Resets the List tail to the original tail's next pointer
    //        self.len -= 1;
    //        return Some(boxed_node.data); // Returns the old tails's data
    //    }
    //    None
    //}
    pub fn pop_tail(&mut self) -> Option<T> {
        if let Some(tail_ptr) = self.tail {
            let boxed_node: Box<Node<T>> = unsafe { Box::from_raw(tail_ptr) }; // Takes ownership
            
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

    /** Returns the length of the list */
    pub fn len(&self) -> usize {
        self.len
    }

    /** Returns a Boolean indicating whether the list is empty */
    pub fn is_empty(&self) -> bool {
        if self.head.is_none() { 
            return true;
        } false
    }

    /** Clears all elements from the list in O(n) time */
    pub fn clear(&mut self) {
        //while self.iter().next.is_some() {
        //    self.pop_head();
        //}
        while self.pop_head().is_some() {}
    }

    /** Returns an iterator of references to data in the list's nodes as list.iter() */
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head, // Correct: Uses the same type as List<T>
            _marker: std::marker::PhantomData, // Needed for lifetime tracking
        }
    }

    //iter_rev() -> Iter
    
    // Positional list utilities
    ////////////////////////////

    //pub fn next() -> Cursor<&T>
    //pub fn peek_next() -> &T {}
    //pub fn peek_prev() -> &T {}
    //pub fn contains(&T) -> bool {}
    //pub fn find(&T) -> Link<T> {}

    /** Acts like a constructor for a cursor */
    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut { 
            cursor: None, 
            list: self, 
            index: None,
        }
    }
}

pub struct Iter<'a, T> {
    next: Link<T>, // aka Option<*mut Node<T>>
    _marker: std::marker::PhantomData<&'a T>, // Ensures correct lifetime tracking
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.next {
            unsafe {
                self.next = (*current).next; // Move to next node
                Some(&(*current).data) // Return reference to data
            }
        } else {
            None
        }
    }
}
impl<T> Drop for List<T> {
    /** List destructor */
    fn drop(&mut self) {
        while self.pop_head().is_some() {}
        //unsafe {
        //    let mut current_node_ptr = self.head;
        //    while let Some(ptr) = current_node_ptr {
        //        // Store a pointer to the next Node before deallocating the current one
        //        let next_node_ptr = (*ptr.as_ptr()).next;

        //        // Deallocate the current node
        //        let _ = Box::from_raw(ptr.as_ptr());

        //        // Advance the Node pointer
        //        current_node_ptr = next_node_ptr;
        //    }
        //}
    }
}

/** Its the great cursor, Charlie Brown! */
// Just a reminder: Link<T> = Option<*mut Node<T>>
pub struct CursorMut<'a, T> {
    cursor: Link<T>,
    list: &'a mut List<T>,
    index: Option<usize>
}
impl<'a, T> CursorMut<'a, T> {

    /** Returns the "index" to the current list node in a zero-indexed list;
    The cursor technically sits _between_ nodes, so this references the "next" (current)
    node's position */
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    /** Moves "forward" (from head to tail) starting from a sentinel node */
    pub fn move_next(&mut self) {
        if let Some(cur) = self.cursor {
            unsafe {
                // We're on a real element, go to its next
                self.cursor = (*cur).next;
                if self.cursor.is_some() {
                    *self.index.as_mut().unwrap() += 1;
                } else {
                    // We just walked to the ghost, no more index
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            // The cursor is at the "ghost" sentinel, and theres nowhere else to go but to the head
            self.cursor = self.list.head;
            self.index = Some(0)
        } else {
            // The cursor is at the "ghost" sentinel, but theres nowhere to go
        }
    }

    /** Moves "backward" (from tail to head) */
    pub fn move_prev(&mut self) {
        if let Some(cur) = self.cursor {
            unsafe {
                // We're on a real element, go to its previous (front)
                self.cursor = (*cur).prev;
                if self.cursor.is_some() {
                    *self.index.as_mut().unwrap() -= 1;
                } else {
                    // We just walked to the ghost, no more index
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            // The cursor is at the "ghost" sentinel, and theres nowhere else to go but to the tail
            self.cursor = self.list.tail;
            self.index = Some(self.list.len - 1)
        } else {
            // The cursor is at the "ghost" sentinel, but theres nowhere to go
        }
    }

    /** Returns a mutable reference to the data at the current position;
    It is necessary to return a _mutable_ reference to retain the elision rule checks */
    pub fn current(&mut self) -> Option<&mut T> {
        unsafe {
            self.cursor.map(|node| &mut (*node).data)
        }
    }
    
    /** Returns a mutable reference to the data at the next node's position;
    It is necessary to return a _mutable_ reference to retain the elision rule checks */
    pub fn peek_next(&mut self) -> Option<&mut T> {
        unsafe {
            let next = if let Some(cur) = self.cursor {
                (*cur).next
            } else {
                // Ghost case, try to use the list's head pointer
                self.list.head
            };
            // Yield the data if the next node exists
            next.map(|node| &mut (*node).data)
        }
    }
    
    /** Returns a mutable reference to the data at the previous node's position;
    It is necessary to return a _mutable_ reference to retain the elision rule checks */
    pub fn peek_prev(&mut self) -> Option<&mut T> {
        unsafe {
            let prev = if let Some(cur) = self.cursor {
                (*cur).prev
            } else {
                // If you're at the sentinel, point to the tail
                self.list.tail
            };
            // Yield the data if the prev node exists
            prev.map(|node| &mut (*node).data)
        }
    }

    //pub fn split_before(&mut self) -> List<T> {}
    //pub fn split_after(&mut self) -> List<T> {}
    //pub fn splice_before(&mut self, mut input: List<T>) {}
    //pub fn splice_after(&mut self, mut input: List<T>) {}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations_test() {
    
        use crate::lists::generic_doubly_linked_list::{List, CursorMut};
    
        // Creates a new doubly-linked list 
        // and pushes some elements to it
        let mut list = List::new();
        assert!(list.is_empty()); // New list is empty
        list.push_head("a");
        list.push_head("b");
        list.push_head("c"); // Postcondition: [c, b, a]
        assert!(!list.is_empty()); // List now has stuff
    
        // Tests the cursor which starts at a non-existant "ghost" node
        // Remember: only one mutable reference at a time!
        let mut cur: CursorMut<'_, &str> = list.cursor_mut(); 
        cur.move_next(); // 0
        cur.move_next(); // 1
        cur.move_next(); // 2
        assert_eq!(cur.index(), Some(2));
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
    }

    #[test]
    fn pointer_test() {

        use crate::lists::generic_doubly_linked_list::{List, CursorMut};
    
        // Creates a new doubly-linked list 
        // and pushes some elements to it
        let mut list = List::new();
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
    
        use crate::lists::generic_doubly_linked_list::{List, CursorMut};
    
        // Creates a new doubly-linked list 
        // and pushes some elements to it
        let mut list = List::new();
        assert!(list.is_empty()); // New list is empty
        list.push_head("a");
        list.push_head("b");
        list.push_head("c"); // Postcondition: [c, b, a]
        assert!(!list.is_empty()); // List now has stuff
    
        // Tests the cursor which starts at a non-existant "ghost" node
        // Remember: only one mutable reference at a time!
        let mut cur: CursorMut<'_, &str> = list.cursor_mut(); 
        cur.move_next(); // 0
        cur.move_next(); // 1
        cur.move_next(); // 2
        assert_eq!(cur.index(), Some(2));
        assert_eq!(cur.current(), Some(&mut "a"));

        cur.move_prev(); // 1
        cur.move_prev(); // 0
        assert_eq!(cur.index(), Some(0));
        assert_eq!(cur.current(), Some(&mut "c"));

        // Moves beyond the list.head to the sentinel
        cur.move_prev(); // Boo! 👻
        assert_eq!(cur.index(), None);
        assert_eq!(cur.current(), None);

        // Wraps back around!
        cur.move_prev(); // 2
        assert_eq!(cur.index(), Some(2));
        assert_eq!(cur.current(), Some(&mut "a"));

        // Next is the ghost, but peek doesn't change the current position
        let peek = cur.peek_next();
        assert_eq!(peek, None);
        assert_eq!(cur.index(), Some(2));
        assert_eq!(cur.current(), Some(&mut "a"));

        let peek = cur.peek_prev();
        assert_eq!(peek, Some(&mut "b"));
        assert_eq!(cur.index(), Some(2));
        assert_eq!(cur.current(), Some(&mut "a"));

    }

}

// Example of a random T
#[derive(Debug)]
pub struct Whatever {
    name: String,
    age: usize,
    notes: String,
}
pub fn example() {
    //use doubly_linked_list::{List, Node};

    // Illustrates the structure with a simple &str
    let mut list = List::new();
    list.push_head("Peter");
    list.push_head("Brain");
    list.push_head("Remus");

    println!("Iter test:");
    let mut counter = 1;
    for e in list.iter() {
        println!("{:>2}: {:<8?}", counter, e);
        counter += 1;
    }

    // Illustrates the structure with a custom struct Whatever
    let first = Whatever {
        name: "Peter".to_string(),
        age: 41,
        notes: "lol".to_string(),
    };
    let second = Whatever {
        name: "Brain".to_string(),
        age: 39,
        notes: "homie".to_string(),
    };
    let third = Whatever {
        name: "Nathan".to_string(),
        age: 38,
        notes: "RIP buddy".to_string(),
    };
    let fourth = Whatever {
        name: "Bobson".to_string(),
        age: 23,
        notes: "Dumbass".to_string(),
    };
    let fifth = Whatever {
        name: "Remus".to_string(),
        age: 45,
        notes: "Ruler".to_string(),
    };
    let mut list = List::new();
    list.push_head(first);
    list.push_head(second);
    list.push_head(third);
    list.pop_head();
    list.push_head(fourth);
    list.pop_tail();
    list.push_tail(fifth);

    for e in list.iter() {
        println!("{:?}", e)
    }

}
