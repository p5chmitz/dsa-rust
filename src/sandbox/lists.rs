#![allow(dead_code)]

// The podium implementation illustrates the list ADT.
// - The array_list interface allows you to build an array of ordered objects.
// - Each entry consists of a name and a score and maintains order by highest to lowest score.
// - The array_list has a max size of 10 entries.
// The interface defines the following operations:
//   1) New: Constructor that creates a new list
//   2) Build/Entry: Takes a name and an Option<score> to create an complete individual list entry
//   1) Add: Takes an entry and inserts it into the list while maintaining order by high score.
//   2) Remove: Removes an entry
//   3) Print: Formats and prints the top three array_list entries.
/** This implementation represents a sorted version of the list ADT and uses only arrays */
pub mod array_list {
    #[derive(Default)] // Required for generic array initialization
    pub struct PodiumEntry {
        pub name: String,
        score: Option<usize>,
    }
    impl Clone for PodiumEntry {
        fn clone(&self) -> PodiumEntry {
            PodiumEntry {
                name: self.name.clone(),
                score: self.score,
            }
        }
    }
    impl PodiumEntry {
        // Gross magic number used to initialize an empty list
        // with indexes from 0 to (PODIUM_SIZE - 1)
        const PODIUM_SIZE: usize = 10;
        /** Creates a list that contains `const PODIUM_SIZE` number of elements with indexes from 0 to (PODIUM_SIZE - 1) */
        // Rust requires constants for array size initialization so the following array-based
        // implementation is not possible
        //pub fn new(size: usize) -> Box<[PodiumEntry]> {
        //    let list: [PodiumEntry; size] = Default::default();
        //    Box::new(list)
        //}
        pub fn new() -> [PodiumEntry; Self::PODIUM_SIZE] {
            let list: [PodiumEntry; Self::PODIUM_SIZE] = Default::default();
            list
        }
        /** Constructs a new PodiumEntry object */
        pub fn build(name: String, score: usize) -> PodiumEntry {
            PodiumEntry {
                name,
                score: Some(score),
            }
        }
        /** Formats PodiumEntry objects for output */
        pub fn format(&self) -> (String, String) {
            let name = self.name.to_owned();
            // Required mapping for entries without scores yet
            let score = match self.score {
                Some(s) => s.to_string(),
                None => "".to_string(),
            };
            // More elegant mapping with closure
            //let score = self.score.map_or("".to_string(), |s| s.to_string());
            (name, score)
        }
        /** Der listen printen; Set bool to `true` for whole list or `false` for top three */
        pub fn print(print_all: bool, podium: &[PodiumEntry; Self::PODIUM_SIZE]) {
            let length: usize;
            if print_all == true {
                length = Self::PODIUM_SIZE
            } else {
                // Magic podium number
                length = 3
            }
            for (i, entry) in podium.iter().enumerate() {
                // Only prints the first three podium entries
                if i >= length {
                    break;
                }
                let entry = entry.format();
                println!("{:>2}: {:<8} {:>6}", i + 1, entry.0, entry.1)
            }
            println!("")
        }
        /** Adds entry to list by score to maintain order;
        Evaluates the existing array values to find the first appropriate index;
        Does not overflow with attempts that exceed base structure size,
        but additional entries without sufficient score are not logged */
        pub fn add(
            mut podium: [PodiumEntry; PodiumEntry::PODIUM_SIZE],
            new_entry: PodiumEntry,
        ) -> [PodiumEntry; PodiumEntry::PODIUM_SIZE] {
            // Evaluates the existing array values to find the first appropriate index
            let mut insert_index = None;
            for i in 0..podium.len() {
                if podium[i].score.is_none() || podium[i].score < new_entry.score {
                    insert_index = Some(i);
                    break;
                }
            }
            // Shift elements to the right of the insertion index to make room
            // for the new entry; Requires Clone implementation on PodiumEntry struct
            if let Some(index) = insert_index {
                for j in (index..podium.len() - 1).rev() {
                    podium[j + 1] = podium[j].clone();
                }
                podium[index] = new_entry;
            }
            podium
        }
        // Takes a podium array and an index, removes the entry at the index
        // and shifts all remaining elements up by one index
        /** Removes an entry from the list */
        pub fn remove(
            mut podium: [PodiumEntry; Self::PODIUM_SIZE],
            cheater: usize,
        ) -> [PodiumEntry; Self::PODIUM_SIZE] {
            for i in cheater..podium.len() - 1 {
                podium[i] = podium[i + 1].clone();
            }
            podium[podium.len() - 1] = Default::default();
            podium
        }
    }
}

/** This driver illustrates an array implementation for the list ADT... using no vectors */
pub fn list_adt_driver_0() {
    // Imports the list implementation and creates default list
    use array_list::PodiumEntry;
    let mut podium = PodiumEntry::new();

    // Sample data to build a basic set of PodiumEntry objects
    let names_array: [String; 4] = [
        "Peter".to_string(),
        "Dingus".to_string(),
        "Brain".to_string(),
        "Bobson".to_string(),
    ];
    let scores_array: [usize; 4] = [1223, 34, 616, 42069];

    // Adds all known sample values to the list and prints the result
    for i in 0..names_array.len() {
        podium = PodiumEntry::add(
            podium,
            PodiumEntry::build(names_array[i].to_string(), scores_array[i as usize]),
        );
    }
    println!("Initial results:");
    PodiumEntry::print(true, &podium);

    // Adds an entry to the middle
    println!("Add Dave\n");
    podium = PodiumEntry::add(podium, PodiumEntry::build("Dave".to_string(), 334));

    // Removes the score at the ith index (Bobson)
    println!("Remove Bobson\n");
    podium = PodiumEntry::remove(podium, 0);

    // 6) Adds an entry to the middle
    println!("Add Dangus\n");
    podium = PodiumEntry::add(podium, PodiumEntry::build("Dangus".to_string(), 420));

    // 7) Prints the final list and podium results
    println!("Final List:");
    PodiumEntry::print(true, &podium);
    println!("Final podium:");
    PodiumEntry::print(false, &podium);
}

// A reimplementation of the podium structure using vectors.
/** This implementation represents a sorted version of the list ADT using vectors for sanity */
pub mod vec_list {
    #[derive(Default)] // Required for generic array initialization
    pub struct PodiumEntry {
        pub name: String,
        score: Option<usize>,
    }
    impl Clone for PodiumEntry {
        fn clone(&self) -> PodiumEntry {
            PodiumEntry {
                name: self.name.clone(),
                score: self.score,
            }
        }
    }
    impl PodiumEntry {
        /** Builds new default list containing at least three elements */
        pub fn new() -> Vec<PodiumEntry> {
            vec![PodiumEntry::default(); 1]
        }
        /** Constructs a new PodiumEntry object */
        pub fn build(name: String, score: usize) -> PodiumEntry {
            PodiumEntry {
                name,
                score: Some(score),
            }
        }
        /** Formats PodiumEntry objects for output */
        pub fn format(&self) -> (String, String) {
            let name = self.name.to_owned();
            // Required mapping for entries without scores yet
            let score = self.score.map_or("".to_string(), |s| s.to_string());
            (name, score)
        }
        /** Prints the first three entires of the list; set print_type to 0 for whole list or 1 for
         * just the top three entries */
        pub fn print(print_all: bool, podium: &Vec<PodiumEntry>) {
            let length: usize;
            if print_all == true {
                length = podium.len()
            } else {
                length = 3
            }
            for (i, entry) in podium.iter().enumerate() {
                // Only prints the first three podium entries
                if i >= length {
                    break;
                }
                let entry = entry.format();
                println!("{:>2}: {:<8} {:>6}", i + 1, entry.0, entry.1)
            }
            println!("")
        }
        /** Adds entry to list by score to maintain order */
        pub fn add(mut podium: Vec<PodiumEntry>, new_entry: PodiumEntry) -> Vec<PodiumEntry> {
            // Evaluates the existing vector and finds appropriate insertion index
            let mut insert_index: i32 = -1;
            for i in 0..podium.len() {
                if podium[i].score.is_none() || podium[i].score < new_entry.score {
                    insert_index = i as i32;
                    break;
                }
            }
            // Inserts the entry at the appropriate index
            podium.insert(insert_index as usize, new_entry);
            podium
        }
        // Takes a podium vec and an index, removes the entry at the index
        // and shifts all remaining elements up by one index
        /** Removes an entry from the list */
        pub fn remove(mut podium: Vec<PodiumEntry>, index: usize) -> Vec<PodiumEntry> {
            for e in index..podium.len() - 1 {
                podium[e] = podium[e + 1].clone();
            }
            //podium[podium.len() - 1] = Default::default();
            podium
        }
    }
}

/** This driver illustrates an array implementation for the list ADT... using no vectors */
pub fn list_adt_driver_1() {
    // Imports the list implementation and creates default list
    use vec_list::PodiumEntry;
    let mut podium = PodiumEntry::new();

    // Sample data to build a basic set of PodiumEntry objects
    let names_vec = vec![
        "Peter".to_string(),
        "Dingus".to_string(),
        "Brain".to_string(),
        "Bobson".to_string(),
    ];
    let scores_vec = vec![1223, 34, 616, 42069];

    // Adds all known sample values to the list and prints the result
    for i in 0..names_vec.len() {
        podium = PodiumEntry::add(
            podium,
            PodiumEntry::build(names_vec[i].to_string(), scores_vec[i as usize]),
        );
    }
    println!("Initial results:");
    PodiumEntry::print(true, &podium);

    // Adds an entry to the middle
    println!("Add Dave\n");
    podium = PodiumEntry::add(podium, PodiumEntry::build("Dave".to_string(), 334));

    // Removes the score at the ith index (Bobson)
    println!("Remove Bobson\n");
    podium = PodiumEntry::remove(podium, 0);

    // 6) Adds an entry to the middle
    println!("Add Dangus\n");
    podium = PodiumEntry::add(podium, PodiumEntry::build("Dangus".to_string(), 420));

    // 7) Prints the final list and podium results
    println!("Final List:");
    PodiumEntry::print(true, &podium);
    println!("Final podium:");
    PodiumEntry::print(false, &podium);
}

// Singly linked list with single ownership as a stack implementation
// This version is adapted from the book Entirely Too Many Linked Lists
pub mod owned_singly_linked_list {

    type Link<T> = Option<Box<Node<T>>>; // Brevity is the soul of wit

    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    // Basic list structure
    pub struct List<T> {
        head: Link<T>,
    }
    impl<T> List<T> {
        /** Creates a new list */
        pub fn new() -> Self {
            List { head: None }
        }
        /** Adds a new list node */
        pub fn push(&mut self, elem: T) {
            let new_node = Box::new(Node {
                elem,
                next: self.head.take(),
            });

            self.head = Some(new_node);
        }
        /** Returns (and removes) the element from the top of the list */
        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.elem
            })
        }
        /** Returns (without removing) the element from the top of the list */
        pub fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.elem)
        }

        // Iterator functions convert instances into iterators
        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                next: self.head.as_deref(),
            }
        }
        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }
        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            IterMut {
                next: self.head.as_deref_mut(),
            }
        }
    }

    // Customizes how elements are dropped, providing structural integrity
    // when deleting a node of the list to avoid overflowing the stack
    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut cur_link = self.head.take();
            while let Some(mut boxed_node) = cur_link {
                cur_link = boxed_node.next.take();
            }
        }
    }

    // Implements the Iterator trait, provides required types/trait definitions for
    // associated function implementations which yield immutable references (iter),
    // owned values (into_iter), and mutable references (iter_mut)
    pub struct Iter<'a, T> {
        next: Option<&'a Node<T>>,
    }
    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                self.next = node.next.as_deref();
                &node.elem
            })
        }
    }

    pub struct IntoIter<T>(List<T>);
    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            // access fields of a tuple struct numerically
            self.0.pop()
        }
    }

    pub struct IterMut<'a, T> {
        next: Option<&'a mut Node<T>>,
    }
    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
            self.next.take().map(|node| {
                self.next = node.next.as_deref_mut();
                &mut node.elem
            })
        }
    }
}

pub fn list_adt_driver_2() {
    use owned_singly_linked_list::List;
    let mut list: List<String> = List::new();
    list.push("Peter".to_string());
    list.push("Brain".to_string());
    list.push("Bobson".to_string());
    list.push("Dingus".to_string());
    for e in list.iter() {
        let node = e.to_string();
        println!("{}", node)
    }
    println!("")
}

/** Adapted list ADT for the podium example */
pub mod podium_singly_linked_list {

    type Link = Option<Box<Node>>;

    pub struct Node {
        pub name: String,
        pub score: i32,
        next: Link,
    }

    // Basic list structure
    pub struct List {
        head: Link,
    }
    impl List {
        /** Creates a new list */
        pub fn new() -> Self {
            List { head: None }
        }
        /** Inserts node into middle of sorted list */
        pub fn insert(&mut self, name: String, score: i32) {
            // Creates a new node to insert
            let mut new_node = Box::new(Node {
                name,
                score,
                next: None,
            });
            // Special case for inserting at the head
            if self.head.is_none() || self.head.as_ref().unwrap().score < score {
                new_node.next = self.head.take();
                self.head = Some(new_node);
                return;
            }
            // Find the correct position for the new node
            let mut current = &mut self.head;
            while let Some(ref mut node) = *current {
                if node.next.is_none() || node.next.as_ref().unwrap().score < score {
                    // Insert before node.next
                    new_node.next = node.next.take();
                    node.next = Some(new_node);
                    return;
                }
                current = &mut node.next;
            }
            // If the score is low enough insert it at the end
            *current = Some(new_node);
        }

        // Iterator functions convert instances into iterators
        pub fn into_iter(self) -> IntoIter {
            IntoIter { next: self.head }
        }
    }

    pub struct IntoIter {
        next: Link,
    }
    impl Iterator for IntoIter {
        type Item = Box<Node>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next.take() {
                Some(mut node) => {
                    self.next = node.next.take();
                    Some(node)
                }
                _ => None,
            }
        }
    }
}

#[test]
pub fn linked_list_test() {
    use podium_singly_linked_list::List;
    let mut podium = List::new();
    podium.insert("Peter".to_string(), 1223);
    podium.insert("Brain".to_string(), 616);
    podium.insert("Dingus".to_string(), 420);
    podium.insert("Dangus".to_string(), 1423);
    podium.insert("Bobson".to_string(), 3434);
    podium.insert("Remus".to_string(), 9838);
    podium.insert("Romulus".to_string(), 18423);
    let mut v = vec![];
    for (i, e) in podium.into_iter().enumerate() {
        v.push(e.name);
        if i == 2 {
            break;
        }
    }
    assert_eq!(
        v,
        vec![
            "Romulus".to_string(),
            "Remus".to_string(),
            "Bobson".to_string()
        ]
    )
}

pub fn list_adt_driver_3() {
    use podium_singly_linked_list::List;
    let mut podium = List::new();
    podium.insert("Peter".to_string(), 1223);
    podium.insert("Brain".to_string(), 616);
    podium.insert("Dingus".to_string(), 420);
    podium.insert("Dangus".to_string(), 1423);
    podium.insert("Bobson".to_string(), 3434);
    podium.insert("Remus".to_string(), 9838);
    podium.insert("Romulus".to_string(), 18423);

    println!("Final (linked list) podium:");
    for (i, e) in podium.into_iter().enumerate() {
        println!("{:>2}: {:<8} {:>6}", i + 1, e.name, e.score);
        if i == 2 {
            break;
        }
    }
}
