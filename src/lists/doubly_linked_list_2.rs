///////////////////////////////////////////
/** A horribly unsafe doubly-linked list */
///////////////////////////////////////////

// Creates a raw pointer to some Node
type Link = Option<*mut Node>;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub score: i32,
    prev: Link,
    next: Link,
}
impl Node {
    // Creates a new node with a unique, heap-allocated address via Box
    pub fn new(name: String, score: i32) -> Box<Node> {
        Box::new(Node {
            name,
            score,
            prev: None,
            next: None,
        })
    }
}
pub struct List {
    head: Link,
    tail: Link,
    length: usize,
}
impl List {
    // Creates a new list
    pub fn new() -> List {
        List {
            head: None,
            tail: None,
            length: 0,
        }
    }
    /** Inserts a node, sorted by its score */
    pub fn insert(&mut self, node: Box<Node>) {
        // Gets a raw, mutable pointer to the (new) unique heap object
        let new_node_ptr: *mut Node = Box::into_raw(node);

        unsafe {
            // Special case for empty list
            if self.head.is_none() {
                // Sets the new node's next pointer to the current tail (None)
                (*new_node_ptr).next = self.tail;

                println!("Inserts first node");
                // Sets initial head and tail pointers, increments the list size
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

            // Traverse the list to find the correct insertion point by peeking at the next node
            let mut current = self.head;
            while let Some(current_ptr) = current {
                let current_node = &mut *current_ptr;
                // Special case for inserting new tail
                if current_node.next.is_none() {
                    // b.prev -> a
                    (*new_node_ptr).prev = Some(current_ptr);
                    // b.next -> c
                    (*new_node_ptr).next = current_node.next;
                    // If c exists, c.prev -> b
                    if let Some(next_node_ptr) = current_node.next {
                        (*next_node_ptr).prev = Some(new_node_ptr);
                    }
                    // a.next -> b 
                    current_node.next = Some(new_node_ptr);

                    // Resets new tail pointer
                    self.tail = Some(new_node_ptr);

                    println!("Inserts new tail");
                    // Increments the list size
                    self.length += 1;
                    return;
                }
                // If the next node's score is None or if the next node's score is less than
                // the new node's score; insert the new node between current and current.next
                else if (*current_node.next.unwrap()).score <= (*new_node_ptr).score {
                    // b.prev -> a
                    (*new_node_ptr).prev = Some(current_ptr);
                    // b.next -> c
                    (*new_node_ptr).next = current_node.next;
                    // If c exists, c.prev -> b
                    if let Some(next_node_ptr) = current_node.next {
                        (*next_node_ptr).prev = Some(new_node_ptr);
                    }
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
    /** Removes a node at a provided index */
    pub fn remove(&mut self, name: String) {
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
                    return;
                }
                current = current_node.next;
            }
            println!("Node not found: {}", name);
        }
    }
    pub fn iter(&self) -> Iter {
        Iter {
            next: self.head.as_ref().map(|&ptr| unsafe { &*ptr }),
            prev: self.tail.as_ref().map(|&ptr| unsafe { &*ptr }),
        }
    }
    pub fn print(&self) {
        let mut counter = 1;
        for e in self.iter() {
            println!("{:>2}: {:<8} {:>6}", counter, e.name, e.score);
            counter += 1;
        }
        println!("")
    }
    pub fn print_rev(&self) {
        let mut counter = self.length;
        for e in self.iter().rev() {
            println!("{:>2}: {:<8} {:>6}", counter, e.name, e.score);
            counter -= 1;
        }
        println!("")
    }
}
pub struct Iter<'a> {
    next: Option<&'a Node>,
    prev: Option<&'a Node>
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a Node;
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
impl Drop for List {
    /** List destructor */
    fn drop(&mut self) {
        unsafe {
            let mut current_node_ptr = self.head;
            while let Some(ptr) = current_node_ptr {
                // Store a pointer to the next Node before deallocating the current one
                let next_node_ptr = (*ptr).next;

                // Deallocate the current node
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
    let a = Node::new("a".to_string(), 101);
    let b = Node::new("b".to_string(), 91);
    let c = Node::new("c".to_string(), 69);
    let d = Node::new("d".to_string(), 47);
    let e = Node::new("e".to_string(), 35);

    unsafe {
        // Test case: Inserts first Node (push)
        list.insert(b); // head is now b...ecause its the only item so far
        let b_ptr: *mut Node = list.head.unwrap(); 
        let b_ref: &mut Node = &mut *b_ptr;
        assert_eq!(b_ref.name, "b");
        assert_eq!(b_ref.score, 91);
        assert_eq!(b_ref.next, None);
        assert_eq!(b_ref.prev, None);
        // b...ut its also the tail
        let b_t_ptr: *mut Node = list.tail.unwrap(); 
        let b_t_ref: &mut Node = &mut *b_t_ptr;
        assert_eq!(b_t_ref.name, "b");
        assert_eq!(b_t_ref.score, 91);
        assert_eq!(b_t_ref.next, None);
        assert_eq!(b_t_ref.prev, None);

        // Test case: Inserts new tail (push back)
        list.insert(d); // tail is now d
        let tail_ptr: *mut Node = list.tail.unwrap(); 
        let d_ref: &mut Node = &mut *tail_ptr; 
        assert_eq!(d_ref.name, "d");
        assert_eq!(d_ref.score, 47);
        assert_eq!(d_ref.next, None);
        assert_eq!(d_ref.prev, Some(b_ptr));

        // Test case: Inserts mid-list
        list.insert(c); // b.next is c
        let c_ptr: *mut Node = (*b_ptr).next.unwrap(); 
        let c_ref: &mut Node = &mut *c_ptr;
        assert_eq!(c_ref.name, "c");
        assert_eq!(c_ref.score, 69);
        assert_eq!(c_ref.next, Some(tail_ptr));
        assert_eq!(c_ref.prev, Some(b_ptr));

        // Test case: Replace head (push)
        list.insert(a); // head is now a
        let a_ptr: *mut Node = list.head.unwrap();
        let a_ref: &mut Node = &mut *a_ptr;
        assert_eq!(a_ref.name, "a");
        assert_eq!(a_ref.score, 101);
        assert_eq!(a_ref.next, Some(b_ptr));
        assert_eq!(a_ref.prev, None);

        // Test case: Replace tail (push back)
        list.insert(e); // tail is now e
        let e_ptr: *mut Node = list.tail.unwrap(); 
        let e_ref: &mut Node = &mut *e_ptr;
        assert_eq!(e_ref.name, "e");
        assert_eq!(e_ref.score, 35);
        assert_eq!(e_ref.next, None);
        assert_eq!(e_ref.prev, Some(tail_ptr));

        // Test case: Remove head (pop)
        list.remove("a".to_string()); // head is still b
        let b_ptr: *mut Node = list.head.unwrap(); 
        let b_ref: &mut Node = &mut *b_ptr;
        assert_eq!(b_ref.name, "b");
        assert_eq!(b_ref.score, 91);
        assert_eq!(b_ref.next, Some(c_ptr));
        assert_eq!(b_ref.prev, None);

        // Test case: Remove tail (pop back)
        list.remove("e".to_string()); // tail is now d
        let tail_ptr: *mut Node = list.tail.unwrap(); 
        let d_ref: &mut Node = &mut *tail_ptr;
        assert_eq!(d_ref.name, "d");
        assert_eq!(d_ref.score, 47);
        assert_eq!(d_ref.next, None);
        assert_eq!(d_ref.prev, Some(c_ptr));
        
        // Test case: Remove mid-list
        list.remove("c".to_string());
        let head_ptr: *mut Node = list.head.unwrap(); // head is still b
        let head_nex: *mut Node = (*head_ptr).next.unwrap(); // head.next should be d
        let d_ref: &mut Node = &mut *head_nex; // Type coercion for assertions
        assert_eq!(d_ref.name, "d");
        assert_eq!(d_ref.score, 47);
        assert_eq!(d_ref.next, None);
        assert_eq!(d_ref.prev, Some(b_ptr));

        // Test case: Removes a non-existant Node safely
        list.remove("x".to_string());
    }
}

/** Runs example operations to demonstrate functionality */
pub fn example() {
    let mut list = List::new();

    let mut node = Node::new("Peter".to_string(), 1223);
    list.insert(node);

    node = Node::new("Brain".to_string(), 616);
    list.insert(node);

    node = Node::new("Remus".to_string(), 1225);
    list.insert(node);

    node = Node::new("Bobson".to_string(), 69);
    list.insert(node);

    node = Node::new("Dorkus".to_string(), 412);
    list.insert(node);

    node = Node::new("Dongus".to_string(), 873);
    list.insert(node);

    // Removes tail
    list.remove("Bobson".to_string());

    // Removes head
    list.remove("Remus".to_string());

    // Ensures safety for non-existent entries
    list.remove("Bjorn".to_string());

    // Removes mid-list
    list.remove("Dongus".to_string());

    // Print this bih
    println!("The final list contains {} results:", list.length);
    list.print();
    list.print_rev();
}
