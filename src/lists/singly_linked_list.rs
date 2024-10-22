//#![allow(dead_code)]

/** This implementation is singly owned */

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

pub fn example() {
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
