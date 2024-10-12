//#[allow(dead_code)]

// A reimplementation of the podium structure in array_list.rs with vectors instead
// This implementation represents a sorted version of the list ADT using vectors for sanity

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
    /** Constructs a new PodiumEntry instance */
    pub fn build(name: String, score: usize) -> PodiumEntry {
        PodiumEntry {
            name,
            score: Some(score),
        }
    }
    /** Formats PodiumEntry instances for output */
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

/** This driver illustrates an array implementation for the list ADT... using no vectors */
pub fn example() {
    // Imports the list implementation and creates default list
    use vec_list::PodiumEntry;
    let mut podium = PodiumEntry::new();

    // Sample data to build a basic set of PodiumEntry instances
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

