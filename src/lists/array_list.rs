//#[allow(dead_code)]

// This implementation represents a sorted version of the list ADT and uses only arrays

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

/** This driver illustrates an array implementation for the list ADT... using no vectors */
pub fn example() {
    // Imports the list implementation and creates default list
    use array_list::PodiumEntry;
    let mut podium = PodiumEntry::new();

    // Sample data to build a basic set of PodiumEntry instances
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
