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

/** A horribly unsafe doubly-linked list */
pub mod doubly_linked_list {

    // Creates a raw pointer to some Node
    type Link = Option<*mut Node>;

    #[derive(Debug)]
    pub struct Node {
        name: String,
        score: i32,
        prev: Link,
        next: Link,
    }
    impl Node {
        // Creates a new node with a unique, heap-allocated address via Box
        pub fn new(name: String, score: i32) -> Node {
            Node {
                name,
                score,
                prev: None,
                next: None,
            }
        }
    }
    //TODO: Implement a way to store a tail reference
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
        pub fn insert(&mut self, node: Node) {
            // Gets a raw, mutable pointer to the (new) unique heap object
            let new_node_ptr: *mut Node = Box::into_raw(Box::new(node));

            unsafe {
                // Special case for empty list
                if self.head.is_none() {
                    // Sets the new node's next pointer to the current head
                    (*new_node_ptr).next = self.head;

                    // Checks that the first Node was inserted properly
                    println!(
                        "The list begins with\n\t{:?}",
                        &(*new_node_ptr),
                    );

                    // Resets the list's head and increments the list size
                    self.head = Some(new_node_ptr);
                    self.length += 1;
                    return;
                }
                // Special case for inserting new head node
                if (*new_node_ptr).score > (*self.head.unwrap()).score {
                    // Sets the new node's next pointer to the current head
                    (*new_node_ptr).next = self.head;
                    // Sets the original head's prev pointer to the new node
                    (*self.head.unwrap()).prev = Some(new_node_ptr);

                    // Checks that the new node was inserted properly
                    println!(
                        "Inserted new head\n\t{:?}\nbetween\n\t{:?}\nand\n\t{:?}",
                        &(*new_node_ptr),
                        &(*new_node_ptr).prev,
                        &(*new_node_ptr).next,
                    );

                    // Resets the list's head and increments the list size
                    self.head = Some(new_node_ptr);
                    self.length += 1;
                    return;
                }

                // Traverse the list to find the correct insertion point by peeking at the next node
                let mut current = self.head;
                while let Some(current_ptr) = current {
                    let current_node = &mut *current_ptr;
                    // If the next node's score is None or if the next node's score is less than
                    // the new node's score; insert the new node between current and current.next
                    if current_node.next.is_none()
                        || (*current_node.next.unwrap()).score <= (*new_node_ptr).score
                    {
                        (*new_node_ptr).next = current_node.next;
                        current_node.next = Some(new_node_ptr);
                        (*new_node_ptr).prev = Some(current_node);

                        // The simple (naive) way;
                        // potentially problematic if you try to unwrap() a None value
                        //let next_node: *mut Node = (*current_node).next.unwrap();
                        //(*next_node).prev = Some(new_node_ptr);

                        // Avoids the potential for a panic if current_node.next is None
                        // This is only really relevant if you want to swap operational precedence
                        if let Some(next_node_ptr) = current_node.next {
                            (*next_node_ptr).prev = Some(new_node_ptr);
                        }

                        // Checks that the new node was inserted properly
                        println!(
                            "Inserted\n\t{:?}\nbetween\n\t{:?}\nand\n\t{:?}",
                            &(*new_node_ptr),
                            &current_node,
                            &(*new_node_ptr).next
                        );

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
            // Traverses the list looking for the Node to remove
            let mut current = self.head;
            unsafe {
                while let Some(current_ptr) = current {
                    let current_node = &mut *current_ptr;
                    // Handles edge case in case the removal node is tail
                    if let Some(next) = current_node.next {
                        if (*next).name == name && (*next).next.is_none() {
                            current_node.next = None;  // Update current node's next pointer
                            println!("Removed tail");
                            self.length -= 1;
                            return;
                        }
                    }
                    // Handles the edge case if the removal node is head
                    if (*current_node).name == name && (*current_node).prev.is_none(){
                        if let Some(peek) = current_node.next {
                            (*peek).prev = None;
                            self.head = Some(peek);
                        } else {
                            self.head = None // In case there is only one list element
                        }
                        println!("Removed head");
                        // Decrements the list size
                        self.length -= 1;
                        return;
                    }
                    // Handles removals mid-list
                    else if (*current_node.next.unwrap()).name == name {
                        // a.next = c
                        let next: *mut Node = current_node.next.unwrap();
                        (*current_node).next = (*next).next; 
                        // c.prev = a
                        (*next).prev = Some(current_node);
                        println!("Removed mid-list");
                        // Decrements the list size
                        self.length -= 1;
                        return;
                    }
                    current = current_node.next;
                }
            }
        }
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

    node = Node::new("Dorkus".to_string(), 412);
    list.insert(node);
    list.print();

    node = Node::new("Dongus".to_string(), 873);
    list.insert(node);
    list.print();

    // Removes tail
    list.remove("Bobson".to_string());
    list.print();

    // Removes head
    list.remove("Remus".to_string());
    list.print();

    // Removes mid-list
    list.remove("Dongus".to_string());
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
#[test]
pub fn mapping() {
    // Illustrates Iterator map
    let numbers = vec![1, 2, 3];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6]);

    // Illustrates Option map
    let greeting_option = Some("Hello, World!".to_string());
    assert_eq!(Some("Hello, World!".to_string()), greeting_option);
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let len = greeting_option.map(|s| s.len());
    assert_eq!(len, Some(13));

    // Illustrates Result map
    let line = "1\n2\n3\n4\n";
    for (i, val) in line.lines().enumerate() {
        // Attempts to parse the String into a usize to match the
        // default type of the enumerate tuple
        match val.parse::<usize>().map(|i| i * 3) {
            Ok(n) => {
                // i is 0-indexed so add one to ensure accuracy
                assert_eq!(n, (i + 1) * 3);
                // Might as well set match arms to return a unit type
                println!("Ok({n})")
            }
            Err(_) => {}
        }
    }
}
