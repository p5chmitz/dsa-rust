#![allow(dead_code)]

/** This implementation is singly owned */
pub mod singly_linked_list {

    pub struct Node {
        name: String,
        score: i32,
        next: Option<Box<Node>>,
    }
    impl Node {
        // Creates a new node
        pub fn new(name: String, score: i32) -> Node {
            Node {
                name,
                score,
                next: None,
            }
        }
    }
    pub struct List {
        head: Option<Box<Node>>, // Adding an extra box just in case things get wild
        length: usize,
    }
    impl List {
        // Creates a new list
        pub fn new() -> List {
            List {
                head: None,
                length: 0,
            }
        }
        /** Inserts a node, sorted by its score */
        pub fn insert(&mut self, node: Node) {
            // Handle the special case of inserting at the head
            if self.head.is_none() || self.head.as_ref().unwrap().score <= node.score {
                let mut new_head = Box::new(node);
                println!("A new star {} emerges", &new_head.name);
                new_head.next = self.head.take();
                self.head = Some(new_head);
                self.length += 1;
                return;
            }

            // Traverse the list to find the insertion point
            let mut iter_node = &mut self.head;
            while let Some(ref mut peek) = iter_node {
                if peek.next.is_none() || peek.next.as_ref().unwrap().score <= node.score {
                    let mut new_node = Box::new(node);
                    new_node.next = peek.next.take();
                    peek.next = Some(new_node);
                    self.length += 1;
                    return;
                }
                iter_node = &mut peek.next;
            }
        }
        /** Removes a node at a provided index */
        pub fn remove(&mut self, index: u32) {
            // Basic logic checks
            if self.head.is_none() {
                println!("Cannot remove elements from an empty list");
                return;
            }
            if index > self.length as u32 {
                println!("Illegal operation; index {} is out of bounds", index);
                return;
            }

            // Handle the special case of removing at the head, also ignoring empty lists
            if index == 0 && self.head.is_some() {
                let next = self.head.as_mut().unwrap().next.take();
                self.head = next;
                self.length -= 1;
                return;
            }

            // Locate and remove the node
            let mut iter_node_ref: &mut Option<Box<Node>> = &mut self.head;
            let mut counter: u32 = 0;
            // Traverses the list, stopping just before the removal node as index - 1;
            // Without a previous pointer this prevents traversing the list twice;
            // peek requres Some(ref mut) to get it to match iter_node's type
            while let Some(ref mut node) = iter_node_ref {
                if counter == index - 1 {
                    // Check if the node intended for removal exists;
                    // If so, assigns the current node's next to the removal node's next
                    // and decreases the list's node counter
                    if let Some(mut node_to_remove) = node.next.take() {
                        node.next = node_to_remove.next.take();
                        self.length -= 1;
                    }
                    return;
                }
                counter += 1;
                // Advances the iterator node
                iter_node_ref = &mut node.next;
            }
        }
        /** Prints the whole list and nothing but the list */
        pub fn print_list(&mut self) {
            println!("Singly inked list contains {} elements:", self.length);
            let mut current = &self.head;
            let mut c = 0;
            while let Some(node) = current {
                println!("{:>2}: {:<8} {:>6}", c + 1, node.name, node.score);
                current = &node.next;
                c += 1;
            }
        }
    }
}
pub fn list_adt_driver_0() {
    use singly_linked_list::{List, Node};

    // Creates a new (empty list)
    let mut podium: List = List::new();
    println!("Created a new list!");

    // Proves that remove is safe on an empty list
    podium.remove(0);

    // Basic insertion
    let node = Node::new("Peter".to_string(), 1223);
    podium.insert(node);

    // Streamlined insertion
    podium.insert(Node::new("Dangus".to_string(), 34));
    podium.insert(Node::new("Remus".to_string(), 8234));
    podium.insert(Node::new("Dingus".to_string(), 602));
    podium.insert(Node::new("Brain".to_string(), 616));

    println!("After all insertions:");
    podium.print_list();

    // Removing items
    let mut i = 0;
    println!("Deleting list index {}", i);
    podium.remove(i);

    // Proves that remove is index safe
    i = 23;
    println!("Deleting list index {}", i);
    podium.remove(i);

    i = 2;
    println!("Deleting list index {}", i);
    podium.remove(i);

    podium.insert(Node::new("Romulus".to_string(), 12837));
    podium.insert(Node::new("Bobson".to_string(), 42069));

    println!("Final list:");
    podium.print_list();
    println!("")
}

pub mod doubly_linked_list {

    // Creates a raw pointer to some Node
    type Link = Option<*mut Node>;

    pub struct Node {
        name: String,
        score: i32,
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
        //TODO: Make prev pointers work
        pub fn insert(&mut self, node: Box<Node>) {
            // Gets a raw, mutable pointer to the unique heap object
            let new_node_ptr = Box::into_raw(node);
        
            unsafe {
                // Special case for empty list or inserting new head node
                if self.head.is_none() || (*new_node_ptr).score > (*self.head.unwrap()).score {
                    // Sets the new node's next pointer to the current head
                    (*new_node_ptr).next = self.head;
                    // Resets the list's head
                    self.head = Some(new_node_ptr);
                    self.length += 1;
                    return;
                }
        
                // Traverse the list to find the correct insertion point
                let mut current = self.head;
                while let Some(current_ptr) = current {
                    let current_node = &mut *current_ptr;
                    // Insert the new node if the next node's score is None 
                    // or if the next node's score is less than the new node's score
                    if current_node.next.is_none()
                        || (*current_node.next.unwrap()).score <= (*new_node_ptr).score
                    {
                        // Sets the new node's next pointer
                        (*new_node_ptr).next = current_node.next;
                        // Resets the current node's next pointer to the new node
                        current_node.next = Some(new_node_ptr);
                        self.length += 1;
                        return;
                    }
                    current = current_node.next;
                }
            }
        }
        /** Removes a node at a provided index */
        //TODO: Implement remove
        pub fn remove(&mut self) {}
        /** Prints the list */
        pub fn print(&self) {
            let mut current = self.head;
            let mut counter = 1;
            println!("Teh double list");
            unsafe {
                while let Some(node_ptr) = current {
                    let node = &*node_ptr;
                    println!("{:>2}: {:<8} {:>6}", counter, node.name, node.score);
                    current = node.next;
                    counter += 1;
                }
            }
            println!("")
        }
    }
}
pub fn doubly_linked_list_driver() {
    use doubly_linked_list::{List, Node};

    let mut list = List::new();
    let mut node = Node::new("Peter".to_string(), 1223);
    list.insert(node);
    list.print();

    node = Node::new("Brain".to_string(), 616);
    list.insert(node);
    list.print();

    node = Node::new("Remus".to_string(), 1225);
    list.insert(node);
    list.print();

    node = Node::new("Bobson".to_string(), 69);
    list.insert(node);
    list.print();
}

// The mem::take() method takes the passed value
// and returns a dummy value or a default value if the type
// implements Default
#[test]
pub fn taking() {
    let mut v: Vec<i32> = vec![1, 2];
    let old_v = std::mem::take(&mut v[0]);
    assert_eq!(1, old_v);

    let mut v: Vec<i32> = vec![1, 2];
    let old_v = std::mem::take(&mut v);
    assert_eq!(vec![1, 2], old_v);
    assert!(v.is_empty());
}
// The mem::replace method replaces a passed value
// and returns the old value
#[test]
pub fn replacing() {
    let mut v: Vec<i32> = vec![1, 2];
    let replaced = std::mem::replace(&mut v[0], 23);
    // The 0th index is now 23
    assert_eq!(v[0], 23);
    assert_eq!(v, vec![23, 2]);
    // The returned value is the old 0th value
    assert_eq!(replaced, 1);
}
