/////////////////////////////////////////////////////
/** A horribly unsafe (generic) doubly-linked list */
/////////////////////////////////////////////////////

use std::ptr::NonNull;
// Recommended when using raw pointers over generic types
use std::marker::PhantomData;

// Creates a raw pointer to some Node
//type Link = Option<*mut Node>;
type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    prev: Link<T>,
    next: Link<T>,
}
impl<T> Node<T> 
where 
    T: PartialEq {
    // Creates a new node with a unique, heap-allocated address via Box
    pub fn new(data: T) -> Node<T> {
        Node {
            data,
            prev: None,
            next: None,
        }
    }
}
/// List contains operations for a basic, unsorted, doubly-linked 
/// list that can act as a stack or a queue; Nodes are pushed to the end 
/// of the list and popped off the front; This implementation also contains 
/// operations for a positional list for applications that require sorted items;
/// In this implementation Link<T> is essentially the position of a node

/** The basic list's public API includes the following operations: 
 - new() -> List<T>
 - push_head(Node<T>) aka add(k)
 - push_tail(Node<T>) aka enqueue(k)
 - pop_head() -> Node<T> aka pop(k)/dequeue(k)
 - pop_tail() -> Node<T> 
 - peek_next() -> &Node<T>
 - peek_prev() -> &Node<T>
 - contains(&Node<T>) -> bool

 - find(&Node<T>) -> Link<T>

The positional list contains the following operations:
 - insert_ith(Node<T>, p) 
 - insert_after(Node<T>, p)
 - insert_before(Node<T>, p)
 - set(Node<T>, p) -> Node<T>
 - remove_ith(p) -> Node<T>
 - remove_after(p) -> Node<T>
 - remove_before(p) -> Node<T>
 - peek_ith(p) -> &Node<T>

All lists contain the following utilities
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
    _phantom: PhantomData<T> // Used to associate T with List<T>
}
impl<T> List<T> {
    // Creates a new list
    pub fn new() -> List<T> {
        List {
            head: None,
            tail: None,
            len: 0,
            _phantom: PhantomData
        }
    }

    /** Inserts a node at the head of the list;
    Use like a push(k) or add(k) operation for a stack */
    pub fn push_front(&mut self, node: Node<T>) {
        unsafe {
            // Creates a NonNull wrapper to the (new) unique heap object
            let new_node_wrapper: NonNull<Node<T>> =
                NonNull::new_unchecked(Box::into_raw(Box::new(node)));

            // Special case for empty list
            if self.head.is_none() {
                // Sets the new node's pointers to None
                (*new_node_wrapper.as_ptr()).next = None;
                (*new_node_wrapper.as_ptr()).prev = None;

                println!("Inserts head");

                // Resets the list's initial head and tail pointers, increments 
                // the list size
                self.head = Some(new_node_wrapper);
                self.tail = Some(new_node_wrapper);
                self.len += 1;
                return;
            }
            // Inserts new node at head
            // Sets the new node's next pointer to the current head
            (*new_node_wrapper.as_ptr()).next = self.head;
            // Sets the original head's prev pointer to the new node
            (*self.head.unwrap().as_ptr()).prev = Some(new_node_wrapper);

            println!("Inserts new head");
            // Resets the list's head and increments the list size
            self.head = Some(new_node_wrapper);
            self.len += 1;
            return;
        }
    }

    /** Returns an owned value */
    //TODO: make this work
    pub fn pop_head(&mut self) -> Option<T> {
        None
    }

        // Special case for empty list

        // Removes head, resents pointers, returns node
        //if let Some(node) = self.head {
        //    let node_ptr = node.as_ptr();

        //    //self.head = self.head.unwrap().as_ptr().next;
        //    
        //    // Move without invalidating pointer
        //    let deref = unsafe { std::ptr::read(&(*node_ptr).data) };
        //    return Some(deref);
        //} None
    //}
    //pub fn pop_tail() -> Node<T> {}
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|&ptr| ptr ),
        }
    }
}
pub struct Iter<T> {
    next: Option<NonNull<Node<T>>>,
}
impl<'a, T> Iterator for Iter<T> {
    type Item = NonNull<Node<T>>;
    /** Returns each Node in the list until there are None */
    fn next(&mut self) -> Option<Self::Item> {
        // Update the iterator to point to the next node, return the current one,
        // and if there aren't any left, its done
        if let Some(current) = self.next {
            self.next = unsafe { (*current.as_ptr()).next };
            Some(current)
        } else {
            None
        }
    }
}
impl<T> Drop for List<T> {
    /** List destructor */
    fn drop(&mut self) {
        unsafe {
            let mut current_node_ptr = self.head;
            while let Some(ptr) = current_node_ptr {
                // Store a pointer to the next Node before deallocating the current one
                let next_node_ptr = (*ptr.as_ptr()).next;

                // Deallocate the current node
                let _ = Box::from_raw(ptr.as_ptr());

                // Advance the Node pointer
                current_node_ptr = next_node_ptr;
            }
        }
    }
}

#[test]
fn test() {
    // Creates a new doubly-linked list
    let mut list = List::new();

    // Creates and insert test nodes
    let a = Node::new("a");
    let b = Node::new("b");
    let c = Node::new("c");
    list.push_front(a);
    list.push_front(b);
    list.push_front(c);

    // Gets pointer to head/c
    let head_ptr: NonNull<Node<&str>> = list.head.unwrap();
    let head: &Node<&str> = unsafe { head_ptr.as_ref() }; // Unsafe deref
    assert_eq!(head.data, "c");

    // Gets pointer to tail/a
    let tail_ptr: NonNull<Node<&str>> = list.tail.unwrap();
    let tail: &Node<&str> = unsafe { tail_ptr.as_ref() }; // Unsafe deref
    assert_eq!(tail.data, "a");

    // Alternative way to get the data
    let tail_ptr: NonNull<Node<&str>> = list.tail.unwrap();
    let tail: *mut Node<&str> = tail_ptr.as_ptr();
    assert_eq!(unsafe { (*tail).data }, "a");

    // Tests the pop_head() method
    let popped_head: &str = list.pop_head().unwrap();
    assert_eq!(popped_head, "c")

    // Follows a.next to b, verifies a.next by checking b's data
    //let b_ptr: *mut Node = a.next.unwrap();
    //let b = &mut *b_ptr; // Unsafe de-ref
    //assert_eq!(b.name, "b");
    //assert_eq!(b.score, 800);

    //// Checks that b.prev -> a
    //assert_eq!(b.prev.unwrap(), head_ptr);

    //// Follows b.next to c, verifies b.next by checking c's data
    //let c_ptr: *mut Node = b.next.unwrap();
    //let c = &mut *c_ptr; // Unsafe de-ref
    //assert_eq!(c.name, "c");
    //assert_eq!(c.score, 600);

    //// Checks that c.prev -> b
    //assert_eq!(c.prev.unwrap(), b_ptr);

    //// Verifies that c == tail || c.next -> None
    //assert!(c.next.is_none());
}

#[derive(PartialEq, Debug)]
pub struct Whatever {
    name: String,
    age: usize,
    notes: String,
}
pub fn example() {

    //use doubly_linked_list::{List, Node};

    let mut list = List::new();
    let mut node = Node::new("Peter");
    list.push_front(node);

    node = Node::new("Brain");
    list.push_front(node);

    node = Node::new("Remus");
    list.push_front(node);

    // Removes tail
    //list.pop_tail("Bobson".to_string());

    // Removes head
    //list.pop_front("Remus".to_string());

    println!("Iter test:");
    let mut counter = 1;
    for e in list.iter() {
        println!("{:>2}: {:<8?}", counter, e);
        counter += 1;
    }

    let first = Whatever { name: "Peter".to_string(), age: 41, notes: "lol".to_string() };
    let second = Whatever { name: "Brain".to_string(), age: 39, notes: "homie".to_string() };
    let mut list = List::new();
    let first = Node::new(first);
    let second = Node::new(second);
    list.push_front(first);
    list.push_front(second);

    if let Some(node) = list.pop_head() {
        println!("Node: {:?}", node)
    };


}
