/////////////////////////////////
/** A dynamic array-based list */
/////////////////////////////////

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    score: Option<i32>,
}
fn build(name: &str, score: Option<i32>) -> Node {
    Node {
        name,
        score,
    }
}
// Required for the Vec::resize operation
impl<'a> Clone for Node<'a> {
    fn clone(&self) -> Node<'a> {
        Node {
            name: self.name,
            score: self.score,
        }
    }
}
/** The List's API contains the following functions:
 - new() -> List<'a>
 - is_empty(self) -> bool
 - insert(&mut self, name: &'a str, score: Option<i32>)
 - set(&mut self, e: T, i: usize) -> Result<(), &'static str>
 - get(&self, i: usize) -> Option<T>
 - remove(&mut self, i: usize) -> Option<T>
 - clear(&mut self)
 - trim(&mut self) - private, called by remove()
NOTE: This is just a funsies excuse to use RefCell; This list probably doesn't need it,
but notice that the get method only takes a reference, which should violate Rust's rules about
having mutable and immutable references in scope, but RefCell takes care of us, that is 
until you try to get and set simultaneously! */
#[derive(Debug)] // Required for list visualization in example function
pub struct List<'a> {
    data: Vec<Option<Node<'a>>>,
    size: usize,
}
impl<'a> List<'a> {
    /** Creates a new generic list with capacity of 1 */
    pub fn new() -> List<'a> {
        List {
            data: vec![None; 3],
            size: 0,
        }
    }
    pub fn is_empty(self) -> bool {
        self.size == 0
    }
    /** Takes a name and optional score, creates a node, and inserts it into the list;
     * If the addition places the list size at or above capacity, the function re-sizes
     * the list by a factor of two */
    pub fn insert(&mut self, name: &'a str, score: Option<i32>) {
        // Checks the list's size against its capacity and 
        // grows geometrically to accommodate new entries
        if self.size + 1 >= self.data.len() {
            self.data.resize(2 * self.data.len(), None);
        }
        //// Initializes the insert index
        //let mut i: usize = 0;
        //// If the list is empty, skips the rest of the checks
        //if self.size == 0 {}
        //// If the new entry has no score its added to the end of the list
        //else if score.is_none() { i = self.size }
        //// Walks all entires with scores to find the insertion point, shifts all lower scores
        //// to make room for new entry
        //else { 
        //    while i <= self.size {
        //        if self.data[i].as_ref().is_none() || self.data[i].as_ref().unwrap().score <= score {
        //            break;
        //        }
        //        i += 1;
        //    }
        //    // Shifts lower scores to the right of the new entry
        //    for j in (i..self.size).rev() {
        //        self.data[j + 1] = self.data[j].clone();
        //    }
        //};
        // Builds and inserts node at correct index, increments list size

        // Or, if you're good at Rust
        //
        // Finds the (first) index that:
        // - Places the new entry at the end of the list if its score is None or
        // - Has data with a score less than or equal to the new entry's score
        let i = if score.is_none() {
            self.size
        } else {
            (0..=self.size)
                .find(|&j| self.data[j].as_ref().map_or(true, |n| n.score <= score))
                .unwrap_or(self.size)
        };
        // Shift elements to make room for the new entry
        for j in (i..self.size).rev() {
            self.data[j + 1] = self.data[j].clone();
        }

        // Builds the entry, inserts it, and increments the list's size
        let node: Node = build(name, score);
        self.data[i] = Some(node);
        self.size += 1;
    }
    /** Attempts to set an element e to the list at index i;
     * Warning: Overwrites any existing data for the specified name */
    pub fn set_score(&mut self, name: &'a str, score: i32) -> Result<(), &'static str> {
        // Matches name with current element values
        let mut i: usize = 0;
        while i <= self.size {
            if self.data[i].as_ref().unwrap().name == name {
                break;
            }
            i += 1;
        }
        if i > self.size {
            return Err("Error: name on in list")
        }
        // Add element to the list and increment the list size
        self.data[i].as_mut().unwrap().score = Some(score);
        self.size += 1;
        Ok(())
    }
    /** Gets (but does not remove) the score for an input name, 
     * if the name matches but there is no score, the function returns 0,
     * if there is no match on the name, function returns Err */
    //pub fn get(&self, name: &str) -> Option<i32> {
    pub fn get(&self, name: &str) -> Result<i32, &str> {
        let mut i: usize = 0;
        while i <= self.size {
            // Matches occupied indexes for the input name
            if self.data[i].is_some() && self.data[i].as_ref().unwrap().name == name {
                break;
            }
            i += 1;
        }
        // Tests if name exists
        if i > self.size {
            return Err("No match on name")
        // Tests if the matched name has a score
        } else if self.data[i].as_ref().unwrap().score.is_none() {
            return Err("No score for entry")
        // Returns the matched name's score
        } else {
            let score: i32 = self.data[i].as_ref().unwrap().score.unwrap();
            Ok(score)
        }
        // Or, if you're good at Rust
        //self.data
        //    .iter()
        //    .take(self.size + 1)
        //    .find_map(|node_opt| match node_opt {
        //        Some(node) if node.name == name => match node.score {
        //            Some(score) => Some(Ok(score)),
        //            None => Some(Err("No score for entry")),
        //        },
        //        _ => None,
        //    })
        //    .unwrap_or(Err("No match on name"))
    }
    /** Attempts to remove (and return) the data that matches the input name */
    pub fn remove(&mut self, n: &str) -> Result<&str, String> {
        //let mut i: usize = 0;
        //while i <= self.size {
        //    // Matches occupied indexes for the input name
        //    if self.data[i].is_some() && self.data[i].as_ref().unwrap().name == n {
        //        break;
        //    }
        //    i += 1;
        //}
        //// Tests if name exists
        //if i > self.size {
        //    return Err("No match on name")
        //// Returns the matched name's score
        //} else {
        //    // Captures name for return
        //    let name: &str = self.data[i].as_ref().unwrap().name;

        //    // Shifts lower scores to the left to fill in the gap of removed entry
        //    for j in i..self.size {
        //        self.data[j] = self.data[j + 1].clone();
        //    }

        //    self.size -= 1;
        //    self.trim();
        //    Ok(name)
        //}
            // Find the index of the entry to remove
        if let Some(i) = (0..=self.size).find(|&i| {
            self.data[i].as_ref().map_or(false, |node| node.name == n)
        }) {
            let name = self.data[i].as_ref().unwrap().name;

            // Shift entries to the left to fill the gap
            for j in i..self.size {
                self.data[j] = self.data[j + 1].clone();
            }

            self.size -= 1;
            self.trim();
            Ok(name)
        } else {
            let err = format!("No match on name {n}");
            Err(err)
        }
    }
    /** Halves the list's capacity (down to a min size of 1) if the size is <= 25% of capacity */
    fn trim(&mut self) {
        let capacity = self.data.len();
        if self.size <= capacity / 4 && capacity > 1 {
            self.data.resize(capacity.max(1) / 2, None);
        }
    }
    /** Clears all elements from the list and resizes to 1 */
    pub fn clear(&mut self) {
        for i in (0..self.data.len()).rev() {
            self.data.remove(i);
        }
        self.data.resize(1, None);
        self.size = 0;
    }
}

/** This driver illustrates an array implementation for the list ADT... using no vectors */
#[test]
fn example() {
    // Creates new list
    let mut list: List = List::new();

    // Sets index 0 to a first name, tests the get method
    list.insert("Chester", Some(30));
    let reference: i32 = list.get("Chester").unwrap();
    assert_eq!(reference, 30);

    // Tests that the size and capacity is appropriate after the first.set
    assert_eq!(list.size, 1);
    assert_eq!(list.data.len(), 3);

    // Ensures that you can insert scoreless entries
    // Handling scoreless entries is up to the user
    list.insert("Copperpot", None);
    assert!(list.get("Copperpot").is_err());
    let score: (i32, &str) = match list.get("Copperpot") {
        Ok(s) => (s, "Found something that shouldn't be here"),
        Err(e) => (0, e),
    };
    assert_eq!(score, (0, "No score for entry"));

    // Tests matches on entires not in the list
    assert!(list.get("Peter").is_err());
    let score: (i32, &str) = match list.get("Peter") {
        Ok(s) => (s, "Found something that shouldn't be here"),
        Err(e) => (0, e),
    };
    assert_eq!(score, (0, "No match on name"));

    // Tests the size of the list
    assert_eq!(list.size, 2);

    // Doubles the list's capacity
    list.insert("Peter", Some(45));
    assert_eq!(list.size, 3);
    assert_eq!(list.data.len(), 6);

    // Preps the list size for further testing
    list.insert("Brain", Some(45));
    list.insert("Dingus", Some(3));
    list.insert("Dangus", Some(87));
    assert_eq!(list.size, 6);
    assert_eq!(list.data.len(), 12);

    // Tests removal of known name
    let name: Result<&str, String> = list.remove("Chester");
    assert_eq!(name, Ok("Chester"));
    assert_eq!(list.size, 5);

    // Tests removal of non-existnat list entry
    let name: Result<&str, String> = list.remove("Remus");
    assert_eq!(name, Err("No match on name Remus".to_string()));

    // Tests that list auto-resizes on removal too
    let _ = list.remove("Copperpot");
    let _ = list.remove("Dingus");
    assert_eq!(list.size, 3);
    assert_eq!(list.data.len(), 6);
}

/** Just used to debug print the list to visualize how the list works */
pub fn usage() {
    // Creates new list
    let mut list: List = List::new();

    // Inserts some entries
    list.insert("Chester", Some(30));
    list.insert("Copperpot", None);
    list.insert("Peter", Some(45));

    println!("The initial list:");
    for e in list.data.iter() {
        println!("{:?}", e);
    }

    // Removes some entries with code for visualization
    let name: &str = "Peter";
    let result: String = match list.remove(name) {
        Ok(_) => "Success".to_string(),
        Err(e) => e,
    };
    println!("Attempt to remove {}: {}", name, result); 

    let name: &str = "Remus";
    let result: String = match list.remove(name) {
        Ok(_) => "Success".to_string(),
        Err(e) => e,
    };
    println!("Attempt to remove {}: {}", name, result); 

    let name: &str = "Chester";
    let result: String = match list.remove(name) {
        Ok(_) => "Success".to_string(),
        Err(e) => e,
    };
    println!("Attempt to remove {}: {}", name, result); 

    // Some sneaky add/removal options
    list.insert("Romulus", Some(100));
    // Removes a legit entry
    let _ = list.remove("Romulus");
    // Attempts to remove a non-existent entry
    let _ = list.remove("David"); // No panic because the return value is ignored
    // list.remove("David").unwrap(); // Using unwrap casuses panic on error

    println!("The final list:");
    for e in list.data.iter() {
        println!("{:?}", e);
    }
    println!("");

}
