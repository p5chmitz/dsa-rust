///////////////////////////////////////////
/** A horribly unsafe doubly-linked list */
///////////////////////////////////////////

/** A raw pointer to some Node */
type Link<'a> = Option<*mut Node<'a>>;

#[derive(Debug)]
pub struct Node<'a> {
    pub name: &'a str,
    pub score: Option<i32>,
    prev: Link<'a>,
    next: Link<'a>,
}
impl<'a> Node<'a> {
    // Creates a new node with a unique, heap-allocated address via Box
    pub fn new(name: &'a str, score: Option<i32>) -> Box<Node<'a>> {
        Box::new(Node {
            name,
            score,
            prev: None,
            next: None,
        })
    }
}
/** The List API includes the following functions:
 - new() -> List
 - insert(&mut self, node: Box<Node>)
 - remove(&mut self, name: String)
 - iter(&self) -> Iter
 - print(&self)
 - print_rev(&self)
*/
pub struct List<'a> {
    head: Link<'a>,
    tail: Link<'a>,
    length: usize,
}
impl<'a> List<'a> {
    // Creates a new list
    pub fn new() -> List<'a> {
        List {
            head: None,
            tail: None,
            length: 0,
        }
    }
    /** Inserts a node, sorted by its score */
    pub fn insert(&mut self, node: Box<Node<'a>>) {
        // Gets a raw, mutable pointer to the (new) unique heap object
        let new_node_ptr: *mut Node = Box::into_raw(node);

        unsafe {
            // Special case for empty list
            if self.head.is_none() {
                // Sets the new node's pointers to None
                (*new_node_ptr).next = None;
                (*new_node_ptr).prev = None;

                println!("Inserts first node");

                // Sets the list's initial head and tail pointers,
                // increments the list size
                self.head = Some(new_node_ptr);
                self.tail = Some(new_node_ptr);
                self.length += 1;
                return;
            }
            // Special case for inserting new head node
            if (*new_node_ptr).score > (*self.head.unwrap()).score {
                // Sets the new node's next pointer to the current head
                (*new_node_ptr).next = self.head;
                // Sets the original head's prev pointer to the new node
                (*self.head.unwrap()).prev = Some(new_node_ptr);

                println!("Inserts new head");

                // Resets the list's head and increments the list size
                self.head = Some(new_node_ptr);
                self.length += 1;
                return;
            }

            // Traverse the list to find the correct insertion point by peeking
            // at the next node
            let mut current = self.head;
            while let Some(current_ptr) = current {
                let current_node = &mut *current_ptr;
                // Special case for inserting new tail
                if current_node.next.is_none() {
                    (*new_node_ptr).prev = Some(current_ptr);
                    (*new_node_ptr).next = None;
                    current_node.next = Some(new_node_ptr);

                    println!("Inserts new tail");

                    // Resets the list's tail pointer and increments the list size
                    self.tail = Some(new_node_ptr);
                    self.length += 1;
                    return;
                }
                // Inserts mid-list;
                // If the next node's score is less than the new node's score
                // insert the new node between current and current.next
                else if (*current_node.next.unwrap()).score <= (*new_node_ptr).score {
                    // b.prev -> a
                    (*new_node_ptr).prev = Some(current_ptr);
                    // b.next -> c
                    (*new_node_ptr).next = current_node.next;
                    // c.prev -> b
                    (*current_node.next.unwrap()).prev = Some(new_node_ptr);
                    // a.next -> b
                    current_node.next = Some(new_node_ptr);

                    println!("Inserts mid-list");

                    // Increments the list size
                    self.length += 1;
                    return;
                }
                current = current_node.next;
            }
        }
    }
    /** Attempts to set a node's score by first matching the name via remove(),
     * and if successful, inserting a new node */
    pub fn set_score(&mut self, name: &'a str, score: Option<i32>) -> Result<(), String> {
        println!("Attempts to set score for {}", name);
        //match self.remove(name) {
        //    Ok(_) => {
        //        let node = Node::new(name, score);
        //        self.insert(node);
        //        Ok(())
        //    },
        //    Err(e) => Err(e),
        //}
        //
        // Or, if you're good at Rust
        self.remove(name).and_then(|_| {
            let node = Node::new(name, score);
            self.insert(node);
            Ok(())
        })
    }
    /** Removes a node at a provided index */
    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        let mut current = self.head;
        unsafe {
            while let Some(current_ptr) = current {
                let current_node = &mut *current_ptr;

                if (*current_node).name == name {
                    // Removes head Node
                    if current_node.prev.is_none() {
                        self.head = current_node.next;
                        if let Some(next) = current_node.next {
                            (*next).prev = None;
                        }
                    // Removes tail Node
                    } else if current_node.next.is_none() {
                        self.tail = current_node.prev;
                        if let Some(prev) = current_node.prev {
                            (*prev).next = None;
                        }
                    // Removes mid-list Node
                    } else {
                        let prev_ptr = current_node.prev.unwrap();
                        let next_ptr = current_node.next.unwrap();
                        (*prev_ptr).next = Some(next_ptr);
                        (*next_ptr).prev = Some(prev_ptr);
                    }

                    // Boxes the removal Node for automatic collection
                    let _ = Box::from_raw(current_ptr);

                    println!("Removed node: {}", name);
                    self.length -= 1;
                    //return;
                    return Ok(());
                }
                current = current_node.next;
            }
            Err(format!("Node not found: {}", name))
            //println!("Node not found: {}", name);
        }
    }
    pub fn iter(&self) -> Iter<'a> {
        Iter {
            next: self.head.as_ref().map(|&ptr| unsafe { &*ptr }),
            prev: self.tail.as_ref().map(|&ptr| unsafe { &*ptr }),
        }
    }
    pub fn print_fwd(&self, rev: bool) {
        let none = "";
        if rev {
            let mut counter = 1;
            for e in self.iter() {
                if let Some(v) = e.score {
                    println!("{:>2}: {:<8} {:>6}", counter, e.name, v);
                } else {
                    println!("{:>2}: {:<8}   {none}", counter, e.name);
                }
                counter += 1;
            }
        } else {
            let mut counter = self.length;
            for e in self.iter().rev() {
                if let Some(v) = e.score {
                    println!("{:>2}: {:<8} {:>6}", counter, e.name, v);
                } else {
                    println!("{:>2}: {:<8}   {none}", counter, e.name);
                }
                counter -= 1;
            }
        }
        println!()
    }
}
pub struct Iter<'a> {
    next: Option<&'a Node<'a>>,
    prev: Option<&'a Node<'a>>,
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a Node<'a>;
    /** Returns each Node in the list until there are None */
    //fn next(&mut self) -> Option<Self::Item> {
    //    // Update the iterator to point to the next node, return the current one,
    //    // and if there aren't any left, its done
    //    if let Some(current) = self.next {
    //        self.next = current.next.as_ref().map(|&ptr| unsafe { &*ptr });
    //        Some(current)
    //    } else {
    //        None
    //    }
    //}
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|current| {
            self.next = current.next.as_ref().map(|&ptr| unsafe { &*ptr });
            current
        })
    }
}
// Enables the use of rev() on Iterator
impl<'a> DoubleEndedIterator for Iter<'a> {
    //fn next_back(&mut self) -> Option<Self::Item> {
    //    if let Some(current) = self.prev {
    //        self.prev = current.prev.as_ref().map(|&ptr| unsafe { &*ptr });
    //        Some(current)
    //    } else {
    //        None
    //    }
    //}
    fn next_back(&mut self) -> Option<Self::Item> {
        self.prev.take().map(|current| {
            self.prev = current.prev.as_ref().map(|&ptr| unsafe { &*ptr });
            current
        })
    }
}
impl<'a> Drop for List<'a> {
    /** List destructor */
    fn drop(&mut self) {
        unsafe {
            let mut current_node_ptr = self.head;
            while let Some(ptr) = current_node_ptr {
                // Store a pointer to the next Node before dropping the current one
                let next_node_ptr = (*ptr).next;

                // Boxes the current Node for automatic collection
                let _ = Box::from_raw(ptr);

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

    // Creates initial head and tail nodes
    let a = Node::new("a", Some(101));
    let b = Node::new("b", Some(91));
    let c = Node::new("c", None);
    let d = Node::new("d", Some(47));
    let e = Node::new("e", Some(35));

    unsafe {
        // Test case: Inserts first Node
        list.insert(b); // head is now b...ecause its the only item so far
        let b_ptr: *mut Node = list.head.unwrap();
        let b_ref: &mut Node = &mut *b_ptr;
        assert_eq!(b_ref.name, "b");
        assert_eq!(b_ref.score, Some(91));
        assert_eq!(b_ref.next, None);
        assert_eq!(b_ref.prev, None);
        // b...ut its also the tail
        let b_t_ptr: *mut Node = list.tail.unwrap();
        let b_t_ref: &mut Node = &mut *b_t_ptr;
        assert_eq!(b_t_ref.name, "b");
        assert_eq!(b_t_ref.score, Some(91));
        assert_eq!(b_t_ref.next, None);
        assert_eq!(b_t_ref.prev, None);

        // Test case: Inserts new tail
        list.insert(d); // tail is now d
        let d_ptr: *mut Node = list.tail.unwrap();
        let d_ref: &mut Node = &mut *d_ptr;
        assert_eq!(d_ref.name, "d");
        assert_eq!(d_ref.score, Some(47));
        assert_eq!(d_ref.next, None);
        assert_eq!(d_ref.prev, Some(list.head.unwrap()));

        // Test case: Inserts None score at the tail
        list.insert(c); // tail is now c (None)
        let c_ptr: *mut Node = list.tail.unwrap();
        let c_ref: &mut Node = &mut *c_ptr;
        assert_eq!(c_ref.name, "c");
        assert_eq!(c_ref.score, None);
        assert_eq!(c_ref.next, None);
        assert_eq!(c_ref.prev, Some(d_ptr));

        // Test case: Sets None score to Some, placing it mid-list
        assert!(list.set_score("c", Some(69)).is_ok()); // Asserts that the operation took
        let c_ptr: *mut Node = (*b_ptr).next.unwrap();
        let c_ref: &mut Node = &mut *c_ptr;
        assert_eq!(c_ref.name, "c");
        assert_eq!(c_ref.score, Some(69));
        assert_eq!(c_ref.next, Some(d_ptr));
        assert_eq!(c_ref.prev, Some(b_ptr));

        // Test case: Replace head
        list.insert(a); // head is now a
        let a_ptr: *mut Node = list.head.unwrap();
        let a_ref: &mut Node = &mut *a_ptr;
        assert_eq!(a_ref.name, "a");
        assert_eq!(a_ref.score, Some(101));
        assert_eq!(a_ref.next, Some(b_ptr));
        assert_eq!(a_ref.prev, None);

        // Test case: Replace tail
        list.insert(e); // tail is now e
        let e_ptr: *mut Node = list.tail.unwrap();
        let e_ref: &mut Node = &mut *e_ptr;
        assert_eq!(e_ref.name, "e");
        assert_eq!(e_ref.score, Some(35));
        assert_eq!(e_ref.next, None);
        assert_eq!(e_ref.prev, Some(d_ptr));

        // Test case: Remove head (pop)
        list.remove("a").ok(); // head is still b
        let b_ptr: *mut Node = list.head.unwrap();
        let b_ref: &mut Node = &mut *b_ptr;
        assert_eq!(b_ref.name, "b");
        assert_eq!(b_ref.score, Some(91));
        assert_eq!(b_ref.next, Some(c_ptr));
        assert_eq!(b_ref.prev, None);

        // Test case: Remove tail
        list.remove("e").ok(); // tail is now d
        let tail_ptr: *mut Node = list.tail.unwrap();
        let d_ref: &mut Node = &mut *tail_ptr;
        assert_eq!(d_ref.name, "d");
        assert_eq!(d_ref.score, Some(47));
        assert_eq!(d_ref.next, None);
        assert_eq!(d_ref.prev, Some(c_ptr));

        // Test case: Remove mid-list
        list.remove("c").ok();
        let head_ptr: *mut Node = list.head.unwrap(); // head is still b
        let head_nex: *mut Node = (*head_ptr).next.unwrap(); // head.next should be d
        let d_ref: &mut Node = &mut *head_nex; // Type coercion for assertions
        assert_eq!(d_ref.name, "d");
        assert_eq!(d_ref.score, Some(47));
        assert_eq!(d_ref.next, None);
        assert_eq!(d_ref.prev, Some(b_ptr));

        // Test case: Removes a non-existant Node safely
        list.remove("x").ok();
    }
}

/** Runs example operations to demonstrate functionality */
pub fn example() {
    use crate::lists::doubly_linked_list_2::{List, Node};

    let mut list = List::new();

    let mut node = Node::new("Peter", Some(1223));
    list.insert(node);

    node = Node::new("Brain", None);
    list.insert(node);

    node = Node::new("Remus", Some(1225));
    list.insert(node);

    node = Node::new("Bobson", Some(69));
    list.insert(node);

    node = Node::new("Dorkus", Some(412));
    list.insert(node);

    node = Node::new("Dongus", Some(873));
    list.insert(node);

    println!("The initial list contains {} results:", list.length);
    list.print_fwd(true);

    // Sets Brain's score and prints the list again
    list.set_score("Brain", Some(616)).ok();
    println!("The revised list contains {} results:", list.length);
    list.print_fwd(true);

    // Attempts to set a non-existent score
    if let Err(msg) = list.set_score("Blorbson", None) {
        println!("Attempt to set a non-existent score:\n\t{}", msg);
    }

    // Removes tail
    list.remove("Bobson").ok();

    // Removes head
    list.remove("Remus").ok();

    // Ensures safety for non-existent entries
    list.remove("Bjorn").ok();

    // Removes mid-list
    list.remove("Dongus").ok();

    // Print this bih
    println!("The final list contains {} results:", list.length);
    list.print_fwd(true);
    list.print_fwd(false);
}
