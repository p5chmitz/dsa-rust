/////////////////////////////////////////////////////
/** A horribly unsafe (generic) doubly-linked list */
/////////////////////////////////////////////////////


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
//impl<T> Node<T>
//where
//    T: PartialEq,
//{
//    // Creates a new node with a unique, heap-allocated address via Box
//    pub fn new(data: T) -> Node<T> {
//        Node {
//            data,
//            prev: None,
//            next: None,
//        }
//    }
//}
/// List contains operations for a basic, unsorted, doubly-linked
/// list that can act as a stack or a queue; Nodes are pushed to the end
/// of the list and popped off the front; This implementation also contains
/// operations for a positional list for applications that require sorted items;
/// In this implementation Link<T> is essentially the position of a node

/** The basic list's API includes the following operations:
 - new() -> List<T>
 - push_head(T) aka add(k)
 - push_tail(T) aka enqueue(k)
 - pop_head() -> Option<T> aka pop(k)/dequeue(k)
 - pop_tail() -> Option<T>

And the following non-public utilities
 - peek_next() -> &T
 - peek_prev() -> &T
 - find(&T) -> Link<T>

The positional list adds the following operations:
 - insert_ith(Node<T>, p)
 - insert_after(Node<T>, p)
 - insert_before(Node<T>, p)
 - set(Node<T>, p) -> Node<T>
 - remove_ith(p) -> Node<T>
 - remove_after(p) -> Node<T>
 - remove_before(p) -> Node<T>
 - peek_ith(p) -> &Node<T>

All lists contain the following non-public utilities
 - contains(&Node<T>) -> bool
 - head() -> Option<&Node<T>>
 - tail() -> Option<&Node<T>>
 - len() -> usize
 - is_empty() -> bool
 - clear()
 - iter(&self) -> Iter
 - iter_rev(&self) -> Iter

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

    /** Inserts a node at the head of the list;
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
                println!("Inserts new head");
                // Sets the new node's next pointer to the current head
                (*new_node_wrapper).next = self.head;
                // Sets the original head's prev pointer to the new node
                (*node).prev = Some(new_node_wrapper); // Unsafe
            }

            // Inserts into empty list
            else {
                println!("Inserts at head");
                // Sets the list's head and tail pointers to the new node
                self.tail = Some(new_node_wrapper);
            }

            // Resets the list's head and increments the list size
            self.head = Some(new_node_wrapper);
            self.len += 1;
            return;
        }
    }

    /** Returns an owned value of the head Node's data */
    pub fn pop_head(&mut self) -> Option<T> {
        if let Some(head_ptr) = self.head {
            let boxed_node: Box<Node<T>> = unsafe { Box::from_raw(head_ptr) }; // takes ownership
            self.head = boxed_node.next; // Resets the List head to the original head's next pointer
            self.len -= 1;
            return Some(boxed_node.data); // Returns the old head's data
        }
        None
    }

    /***/
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

    /** Inserts a node at the tail of the list in O(1) time;
    Use like a push(k) or add(k) operation for a stack */
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

    //pub fn peek_next() -> &T {}
    //pub fn peek_prev() -> &T {}
    //pub fn contains(&T) -> bool {}
    //pub fn find(&T) -> Link<T> {}

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head, // Correct: Uses the same type as List<T>
            _marker: std::marker::PhantomData, // Needed for lifetime tracking
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

#[test]
fn test() {
    // Creates a new doubly-linked list
    let mut list = List::new();

    // Operations Tests
    ///////////////////

    // Precondition: []
    list.push_head("a");
    list.push_head("b");
    list.push_head("c");
    // Postcondition: [c, b, a]

    let popped_head: &str = list.pop_head().unwrap();
    assert_eq!(popped_head, "c");

    let popped_head: &str = list.pop_head().unwrap();
    assert_eq!(popped_head, "b");
    // Postcondition: [a]

    list.push_head("b");
    list.push_head("c");
    // Postcondition: [c, b, a]

    assert_eq!(list.len, 3);

    let popped_tail: &str = list.pop_tail().unwrap();
    assert_eq!(popped_tail, "a");

    let popped_tail: &str = list.pop_tail().unwrap();
    assert_eq!(popped_tail, "b");
    // Postcondition: [c]

    assert_eq!(list.len, 1);

    list.push_tail("b");
    list.push_tail("a");
    // Postcondition: [c, b, a]

    assert_eq!(list.len, 3);

    // Pointer Tests
    ////////////////

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

}

#[derive(PartialEq, Debug)]
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
    //while let Some(node) = list.pop_head() {
    //    println!("Node: {:?}", node);
    //}
}
