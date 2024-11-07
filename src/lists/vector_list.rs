/////////////////////////////////
/** A simple vector-based list */
/////////////////////////////////

#[derive(Default)] // Required for generic array initialization
pub struct PodiumEntry<'a> {
    pub name: &'a str,
    score: Option<i32>,
}
impl<'a> Clone for PodiumEntry<'a> {
    fn clone(&self) -> PodiumEntry<'a> {
        PodiumEntry {
            name: self.name,
            score: self.score,
        }
    }
}
/** The Podium's public API contains the following functions:
 - new() -> Podium<'a>
 - add(&mut self, name: &'a str, score: Option<i32>)
 - remove(&mut self, index: usize) -> Option<&str>
 - peek???
 - print(&self, print_all: bool) {

Podium also contains the following utility functions:
 - build(name: &str, score: Option<i32>) -> PodiumEntry {
 - format(&self, entry: &PodiumEntry) -> (String, String) {

NOTE: Seriously, just use Vec */
pub struct Podium<'a> {
    data: Vec<PodiumEntry<'a>>,
    size: usize,
}
impl<'a> Podium<'a> {
    /** Builds new default list containing at least three elements */
    pub fn new() -> Podium<'a> {
        Podium {
            data: vec![PodiumEntry::default(); 3],
            size: 0
        }
    }
    /** Returns a reference to the ith node */
    pub fn peek(&self, index: usize) -> Option<&PodiumEntry> { 
        let data = &self.data[index];
        Some(data)
    }
    /** Adds entry to list by score to maintain order in O(n) time */
    pub fn add(&mut self, name: &'a str, score: Option<i32>) { 
        let entry = Self::build(name, score);
        // Evaluates the existing vector and finds appropriate insertion index
        let mut insert_index = 0;
        for i in 0..self.data.len() {
            if self.data[i].score.is_none() || self.data[i].score < score {
                insert_index = i;
                break;
            }
        }
        // Inserts the entry at the appropriate index
        self.data.insert(insert_index as usize, entry);
        self.size += 1;
    }
    /** Removes the entry at the index and shifts all remaining elements up by one 
    place in O(n) time; Vec::remove() also runs in O(n) time but does not clone */
    pub fn remove(&mut self, index: usize) -> Option<&str> {
        let rtn = Some(self.data[index].clone().name);
        for e in index..self.data.len() - 1 {
            self.data[e] = self.data[e + 1].clone();
        }
        self.size -= 1;
        rtn
    }

    // Utility functions

    /** Prints the Podium list; If you supply true the function prints the entire list,
     if you supply false the function just prints the top three spots */
    pub fn print_full(&self, print_all: bool) {
        let length: usize;
        if print_all == true {
            length = self.data.len()
        } else {
            length = 3
        }
        for (i, entry) in self.data.iter().enumerate() {
            // Only prints the first three podium entries
            if i >= length {
                break;
            }
            let entry = self.format(&entry);
            println!("{:>2}: {:<8} {:>6}", i + 1, entry.0, entry.1)
        }
        println!("")
    }
    /** Constructs a new PodiumEntry instance */
    fn build(name: &str, score: Option<i32>) -> PodiumEntry {
        PodiumEntry {
            name,
            score,
        }
    }
    /** Formats PodiumEntry instances for output */
    fn format(&self, entry: &PodiumEntry) -> (String, String) {
        let name = entry.name.to_owned();
        // Required mapping for entries without scores yet
        let score = entry.score.map_or("".to_string(), |s| s.to_string());
        (name, score)
    }

}

#[test]
fn vector_list_test() {
    use crate::lists::vector_list::Podium;
    let mut podium = Podium::new();

    // Sample data to build a basic set of PodiumEntry instances
    let names_vec = vec![
        "Peter",
        "Dingus",
        "Brain",
        "Bobson",
    ];
    let scores_vec = vec![1223, 34, 616, 42069];

    // Adds all known sample values to the list
    for i in 0..names_vec.len() {
        podium.add(names_vec[i], Some(scores_vec[i as usize]));
    }

    // Tests that the lead score is the cheater
    assert_eq!(Some("Bobson"), podium.remove(0));

    // Adds the new bronze medalist... but it turns out its not legit
    podium.add("Dangus", Some(187));
    assert_eq!(Some("Dangus"), podium.remove(2));

}

/** This example illustrates an array implementation for the list ADT... using no vectors */
pub fn example() {
    use crate::lists::vector_list::Podium;
    let mut podium = Podium::new();

    // Sample data to build a basic set of PodiumEntry instances
    let names_vec = vec![
        "Peter",
        "Dingus",
        "Brain",
        "Bobson",
    ];
    let scores_vec = vec![1223, 34, 616, 42069];

    // Adds all known sample values to the list and prints the result
    for i in 0..names_vec.len() {
        podium.add(names_vec[i], Some(scores_vec[i as usize]));
    }
    println!("Initial results:");
    podium.print_full(true);

    // Adds an entry to the middle
    println!("Add Dave\n");
    podium.add("Dave", Some(334));

    // Removes the score at the ith index (Bobson)
    println!("Remove Bobson\n");
    podium.remove(0);

    // Adds an entry to the middle
    println!("Add Dangus\n");
    podium.add("Dangus", Some(420));

    // Prints the final list and podium results
    println!("Final List:");
    podium.print_full(true);
    println!("Final podium:");
    podium.print_full(false);
}
