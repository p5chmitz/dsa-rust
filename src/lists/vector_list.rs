/////////////////////////////////
/** A simple vector-based list */
/////////////////////////////////

#[derive(Default)] // Required for generic array initialization
pub struct PodiumEntry {
    name: String,
    score: Option<i32>,
}
// Required for the default initialization and remove functions
impl Clone for PodiumEntry {
    fn clone(&self) -> PodiumEntry {
        PodiumEntry {
            name: self.name.clone(),
            score: self.score,
        }
    }
}
/// NOTE: Seriously, just use `Vec`
pub struct Podium {
    data: Vec<PodiumEntry>,
    size: usize,
}
impl Podium {
    /** Builds new default list containing at least three elements */
    pub fn new() -> Podium {
        Podium {
            data: vec![PodiumEntry::default(); 3],
            size: 0,
        }
    }
    /** Adds entry to list by score to maintain order in O(n) time */
    pub fn add(&mut self, name: String, score: Option<i32>) {
        let entry = Self::build(name, score);
        // Evaluates the existing vector and finds appropriate insertion index
        let mut insert_index = 0;
        if score.is_none() {
            insert_index = self.size;
        } else {
            for i in 0..self.data.len() {
                if self.data[i].score < score {
                    insert_index = i;
                    break;
                }
            }
        }
        // Inserts the entry at the appropriate index
        self.data.insert(insert_index, entry);
        self.size += 1;
    }
    /** Removes the entry at the index and shifts all remaining elements up by one
    place in O(n) time; Vec::remove() also runs in O(n) time but does not clone */
    pub fn remove(&mut self, index: usize) -> Option<String> {
        let rtn = Some(self.data[index].clone().name);
        for e in index..self.data.len() - 1 {
            self.data[e] = self.data[e + 1].clone();
        }
        self.size -= 1;
        rtn
    }
    /** Attempts to set a new score for a given index */
    pub fn set_score(&mut self, index: usize, score: Option<i32>) -> Result<(), String> {
        if let Some(entry) = self.remove(index) {
            self.add(entry, score);
            Ok(())
        } else {
            Err("Error: not found".to_string())
        }
    }

    // Utility functions

    /** Prints the Podium list; If you supply true the function prints the entire list,
    if you supply false the function just prints the top three spots */
    pub fn print_full(&self, print_all: bool) {
        let length: usize = if print_all {
            self.data.len()
        } else {
            3
        };
        for (i, entry) in self.data.iter().enumerate() {
            // Only prints the first three podium entries
            if i >= length {
                break;
            }
            let entry = self.format(entry);
            println!("{:>2}: {:<8} {:>6}", i + 1, entry.0, entry.1)
        }
        println!()
    }
    /** Constructs a new PodiumEntry instance */
    fn build(name: String, score: Option<i32>) -> PodiumEntry {
        PodiumEntry { name, score }
    }
    /** Formats PodiumEntry instances for output */
    fn format(&self, entry: &PodiumEntry) -> (String, String) {
        let name = entry.name.to_owned();
        // Required mapping for entries without scores yet
        let score = match entry.score {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        (name, score)
    }
}

#[test]
fn vector_list_test() {
    use crate::lists::vector_list::Podium;
    let mut podium = Podium::new();

    // Builds and adds a basic set of PodiumEntry instances
    let names_vec = vec!["Peter", "Dingus", "Brain", "Bobson"];
    let scores_vec = vec![Some(1223), None, Some(616), Some(42069)];
    for i in 0..names_vec.len() {
        podium.add(names_vec[i].to_string(), scores_vec[i as usize]);
    }

    // Tests that the lead score is the cheater
    assert_eq!(Some("Bobson".to_string()), podium.remove(0));

    // Adds the new bronze medalist... but it turns out its not legit
    podium.add("Dangus".to_string(), Some(187));
    assert_eq!(Some("Dangus".to_string()), podium.remove(2));
}

/** This example illustrates an array implementation for the list ADT... using no vectors */
pub fn example() {
    use crate::lists::vector_list::Podium;
    let mut podium = Podium::new();

    // Sample data to build a basic set of PodiumEntry instances
    let names_vec = ["Peter", "Dingus", "Brain", "Bobson"];
    let scores_vec = [Some(1223), None, Some(616), Some(42069)];
    for i in 0..names_vec.len() {
        podium.add(names_vec[i].to_string(), scores_vec[i]);
    }
    println!("Initial results:");
    podium.print_full(true);

    // Adds an entry to the middle
    let mut name = "Dave";
    println!("Add {name}");
    podium.add({ name }.to_string(), Some(334));

    // Adds an entry with no score to the end of the list
    name = "Blorbson";
    println!("Add {name} with score None");
    podium.add(name.to_string(), None);
    podium.print_full(true);

    // Sets the None score entry to Some
    println!("Set {name}'s score to 345555");
    podium.set_score(5, Some(345555)).ok();
    //podium.print_full(true);

    // Removes the first score in the list
    let i = 1;
    println!("Remove the entry at index {i}");
    podium.remove(i);

    // Adds an entry to the middle
    name = "Dangus";
    println!("Add {name}");
    podium.add({ name }.to_string(), Some(420));

    // Prints the final list and podium results
    println!("Final List:");
    podium.print_full(true);
    println!("Final podium:");
    podium.print_full(false);
}
