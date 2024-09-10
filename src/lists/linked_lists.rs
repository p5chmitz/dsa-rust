#![allow(dead_code)]

pub mod singly_linked_list {

    pub struct Node {
        name: String,
        score: i32,
        next: Option<Box<Node>>
    }
    impl Node {
        // Creates a new node
        pub fn new(name: String, score: i32) -> Node {
            Node {
                name,
                score,
                next: None
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
                length: 0
            }
        }
        // Inserts a node, sorted by its score
        pub fn insert(&mut self, node: Node) {
            // Handle the special case of inserting at the head
            if self.head.is_none() || self.head.as_ref().unwrap().score <= node.score {
                let mut new_head = Box::new(node);
                println!("A new star {} emerges", &new_head.name);
                new_head.next = self.head.take();
                self.head = Some(new_head);
                self.length += 1;
                return
            }

            // Traverse the list to find the insertion point
            let mut iter_node = &mut self.head;
            while let Some(ref mut peek) = iter_node {
                if peek.next.is_none() || peek.next.as_ref().unwrap().score <= node.score {
                    let mut new_node = Box::new(node);
                    new_node.next = peek.next.take();
                    peek.next = Some(new_node);
                    self.length += 1;
                    return
                }
                iter_node = &mut peek.next;
            }
        }
        // Removes a node
        pub fn remove(&mut self, index: u32) {
            // Basic logic checks
            if index == 0 && self.head.is_none() {
                println!("Cannot remove elements from an empty list");
                return
            }
            if index > self.length as u32 {
                println!("Illegal operation; index {} is out of bounds index", index);
                return
            }

            // Handle the special case of removing at the head
            // Ignores empty lists
            if index == 0 && self.head.is_some() {
                let next = self.head.as_mut().unwrap().next.take();
                self.head = next;
                self.length -= 1;
                return
            }

            // Traverse the list to find the node to remove
            let mut iter_node = &mut self.head;
            let mut counter = 0;
            while let Some(ref mut peek) = iter_node {
                if counter == index as usize - 1 {
                    // Check if the next node exists (the one to be removed)
                    if let Some(mut node_to_remove) = peek.next.take() {
                        // Reassign the next pointer of the current node to skip the node to be removed
                        peek.next = node_to_remove.next.take();
                        self.length -= 1;
                    }
                    return
                }
                counter += 1;
                iter_node = &mut peek.next;
            }

        }
        // Prints the list
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

}
