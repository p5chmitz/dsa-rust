/////////////////////////////////////////////////////
/** A horribly unsafe (generic) doubly-linked list */
/////////////////////////////////////////////////////

// Creates a raw pointer to some Node
//type Link = Option<*mut Node>;
use std::ptr::NonNull;
type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    prev: Link<T>,
    next: Link<T>,
}
impl<T> Node<T> {
    // Creates a new node with a unique, heap-allocated address via Box
    pub fn new(data: T) -> Node<T> {
        Node {
            data,
            prev: None,
            next: None,
        }
    }
}
/** The List's public API contains the following functions:
 - new() -> List<T>
 - insert_head(node)
 - insert_tail(node)
 - insert_ith(node, p) / insert_after() / insert_before()
 - remove_head()
 - remove_tail()
 - remove_ith(p) / remove_after() / remove_before()
 - peek_ith(p) (returns the node at position p)
 - iter(&self) -> Iter
 - print(&self)
 - print_rev(&self)
NOTE: To implement a positional list adding nodes return a reference that can be passed to acessor/mutator methods for O(1) operations.
*/
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}
impl<T> List<T> {
    // Creates a new list
    pub fn new() -> List<T> {
        List {
            head: None,
            tail: None,
            length: 0,
        }
    }
    /** Inserts a node, sorted by its score */
    pub fn insert(&mut self, node: Node<T>, index: usize) {
        unsafe {
            // Creates a NonNull wrapper to the (new) unique heap object
            let new_node_wrapper: NonNull<Node<T>> =
                NonNull::new_unchecked(Box::into_raw(Box::new(node)));

            // Special case for empty list
            if self.head.is_none() {
                // Sets the new node's next pointer to the current head
                (*new_node_wrapper.as_ptr()).next = self.head;
                //(*new_raw_ptr).next = self.head;

                println!("Inserts head");
                // Resets the list's head and increments the list size
                self.head = Some(new_node_wrapper);
                self.length += 1;
                return;
            }
            // Special case for inserting new head node
            if index == 0 {
                // Sets the new node's next pointer to the current head
                (*new_node_wrapper.as_ptr()).next = self.head;
                // Sets the original head's prev pointer to the new node
                (*self.head.unwrap().as_ptr()).prev = Some(new_node_wrapper);

                println!("Inserts new head");
                // Resets the list's head and increments the list size
                self.head = Some(new_node_wrapper);
                self.length += 1;
                return;
            }

            // Traverse the list to find the correct insertion point by peeking at the next node
            let mut current = self.head;
            while let Some(current_ptr) = current {
                // Gets a raw pointer to the current NonNull<Node<T>> reference
                let current_node: *mut Node<T> = current_ptr.as_ptr();
                //let current_node = &mut *current_ptr;
                // If the next node's score is None or if the next node's score is less than
                // the new node's score; insert the new node between current and current.next
                if (*current_node).next.is_none()
                //|| (*current_node).next.unwrap() == index
                {
                    // b.prev -> a
                    (*new_node_wrapper.as_ptr()).prev = Some(current_ptr);
                    // b.next -> c
                    (*new_node_wrapper.as_ptr()).next = (*current_node).next;
                    // If c exists, c.prev -> b
                    if let Some(next_node_ptr) = (*current_node).next {
                        (*next_node_ptr.as_ptr()).prev = Some(new_node_wrapper);
                    }
                    // a.next -> b
                    (*current_node).next = Some(new_node_wrapper);

                    println!("Inserts mid-list or new tail");
                    // Increments the list size
                    self.length += 1;
                    return;
                }
                current = (*current_node).next;
            }
        }
    }
    //    /** Removes a node at a provided index */
    //    pub fn remove(&mut self, index: usize) {
    //        // Traverses the list looking for the Node to remove
    //        let mut current = self.head;
    //        unsafe {
    //            while let Some(current_ptr) = current {
    //                let current_node = &mut *current_ptr.as_ptr();
    //                // Handles edge case in case the removal node is tail
    //                if let Some(next) = current_node.next {
    //                    if self.length == index && (*next.as_ptr()).next.is_none() {
    //                        // Update the current node's next pointer
    //                        current_node.next = None;
    //                        println!("Removed tail");
    //                        self.length -= 1;
    //                        return;
    //                    }
    //                }
    //                // Handles the edge case if the removal node is head
    //                if (*current_node).prev.is_none() {
    //                    if let Some(peek) = current_node.next {
    //                        (*peek.as_ptr()).prev = None;
    //                        self.head = Some(peek);
    //                    } else {
    //                        // In case there is only one list element
    //                        self.head = None;
    //                    }
    //                    println!("Removed head");
    //                    // Decrements the list size
    //                    self.length -= 1;
    //                    return;
    //                }
    //                // Handles removals mid-list
    //                else if (*current_node.next.unwrap()).name == name {
    //                    // a.next = c
    //                    let next: *mut Node<T> = current_node.next.unwrap();
    //                    (*current_node).next = (*next).next;
    //                    // c.prev = a
    //                    (*next).prev = Some(current_node);
    //                    println!("Removed mid-list");
    //                    // Decrements the list size
    //                    self.length -= 1;
    //                    return;
    //                }
    //                current = current_node.next;
    //            }
    //        }
    //    }
    //    pub fn iter(&self) -> Iter<T> {
    //        Iter {
    //            next: self.head.as_ref().map(|&ptr| unsafe { &*ptr }),
    //        }
    //    }
    //    /** Prints the list */
    //    pub fn print(&self) {
    //        let mut current = self.head;
    //        let mut counter = 1;
    //        unsafe {
    //            while let Some(node_ptr) = current {
    //                let node = &*node_ptr;
    //                println!("{:>2}: {:<8} {:>6}", counter, node.name, node.score);
    //                current = node.next;
    //                counter += 1;
    //            }
    //        }
    //        println!("")
    //    }
}
//pub struct Iter<'a, T> {
//    next: Option<&'a Node<T>>,
//}
//impl<'a, T> Iterator for Iter<'a, T> {
//    type Item = &'a Node<T>;
//    /** Returns each Node in the list until there are None */
//    fn next(&mut self) -> Option<Self::Item> {
//        // Update the iterator to point to the next node, return the current one,
//        // and if there aren't any left, its done
//        if let Some(current) = self.next {
//            self.next = current.next.as_ref().map(|&ptr| unsafe { &*ptr });
//            Some(current)
//        } else {
//            None
//        }
//    }
//}
//impl<T> Drop for List<T> {
//    /** List destructor */
//    fn drop(&mut self) {
//        unsafe {
//            let mut current_node_ptr = self.head;
//            while let Some(ptr) = current_node_ptr {
//                // Store a pointer to the next Node before deallocating the current one
//                let next_node_ptr = (*ptr).next;
//
//                // Deallocate the current node
//                let _ = Box::from_raw(ptr);
//
//                // Advance the Node pointer
//                current_node_ptr = next_node_ptr;
//            }
//        }
//    }
//}

//#[test]
//fn test() {
//    // Creates a new doubly-linked list
//    let mut list = List::new();
//
//    // Creates and insert nodes with scores 1000 and 600
//    let a = Node::new("a".to_string(), 1000);
//    let c = Node::new("c".to_string(), 600);
//    list.insert(a);
//    list.insert(c);
//
//    // Creates and insert node b with a score between a and c
//    let b = Node::new("b".to_string(), 800);
//    list.insert(b);
//
//    unsafe {
//        // Gets pointer to head/a
//        let head_ptr: *mut Node = list.head.unwrap();
//        let a = &mut *head_ptr; // Unsafe de-ref
//        assert_eq!(a.name, "a");
//        assert_eq!(a.score, 1000);
//
//        // Follows a.next to b, verifies a.next by checking b's data
//        let b_ptr: *mut Node = a.next.unwrap();
//        let b = &mut *b_ptr; // Unsafe de-ref
//        assert_eq!(b.name, "b");
//        assert_eq!(b.score, 800);
//
//        // Checks that b.prev -> a
//        assert_eq!(b.prev.unwrap(), head_ptr);
//
//        // Follows b.next to c, verifies b.next by checking c's data
//        let c_ptr: *mut Node = b.next.unwrap();
//        let c = &mut *c_ptr; // Unsafe de-ref
//        assert_eq!(c.name, "c");
//        assert_eq!(c.score, 600);
//
//        // Checks that c.prev -> b
//        assert_eq!(c.prev.unwrap(), b_ptr);
//
//        // Verifies that c == tail || c.next -> None
//        assert!(c.next.is_none());
//    }
//}
//
//pub fn example() {
//    println!("The infamous (and unsafe) double!!");
//
//    //use doubly_linked_list::{List, Node};
//
//    let mut list = List::new();
//    let mut node = Node::new("Peter".to_string(), 1223);
//    list.insert(node);
//
//    node = Node::new("Brain".to_string(), 616);
//    list.insert(node);
//
//    node = Node::new("Remus".to_string(), 1225);
//    list.insert(node);
//
//    node = Node::new("Bobson".to_string(), 69);
//    list.insert(node);
//
//    node = Node::new("Dorkus".to_string(), 412);
//    list.insert(node);
//
//    node = Node::new("Dongus".to_string(), 873);
//    list.insert(node);
//
//    // Removes tail
//    list.remove("Bobson".to_string());
//
//    // Removes head
//    list.remove("Remus".to_string());
//
//    // Removes mid-list
//    list.remove("Dongus".to_string());
//
//    // Print this bih
//    println!("The final result:");
//    list.print();
//
//    println!("Iter test:");
//    let mut counter = 1;
//    for e in list.iter() {
//        println!("{:>2}: {:<8} {:>6}", counter, e.name, e.score);
//        counter += 1;
//    }
//}
