///////////////////////////////////////
/** A simple, owned array-based list */
///////////////////////////////////////

#[derive(Default)] // Required for generic array initialization
pub struct Podium {
    pub name: String,
    score: Option<usize>,
}
impl Clone for Podium {
    fn clone(&self) -> Podium {
        Podium {
            name: self.name.clone(),
            score: self.score,
        }
    }
}
/** The Podium's public API contains the following functions: 
 * - new() -> [Podium; Self::PODIUM_SIZE]
 * - build(name: String, score: usize) -> Podium
 * - print(print_all: bool, podium: &[Podium; Self::PODIUM_SIZE])
 * - remove(mut podium: [Podium; Self::PODIUM_SIZE], cheater: usize,) -> [Podium; Self::PODIUM_SIZE]
 * - add(mut podium: [Podium; Podium::PODIUM_SIZE], new_entry: Podium,) -> [Podium; Podium::PODIUM_SIZE]
 *
* NOTE: Rust requires array initializations to happen at compile time; For implementations where the 
* same value is used across several functions in a module you need to use a constant */
impl Podium {
    // Sets list size with indexes from 0 to (PODIUM_SIZE - 1)
    const PODIUM_SIZE: usize = 10;

    /** Creates a list that contains `const PODIUM_SIZE` number of elements with indexes from 0 to (PODIUM_SIZE - 1) */
    pub fn new() -> [Podium; Self::PODIUM_SIZE] {
        let list: [Podium; Self::PODIUM_SIZE] = Default::default();
        list
    }

    /** Constructs a new Podium entry */
    pub fn entry(name: String, score: usize) -> Podium {
        Podium {
            name,
            score: Some(score),
        }
    }

    /** Adds entry to list by score to maintain a sorted list;
    Does not overflow with attempts that exceed the initialized structure size,
    but does not log additional entries without sufficiently high score values */
    pub fn add(
        mut podium: [Podium; Podium::PODIUM_SIZE],
        new_entry: Podium,
    ) -> [Podium; Podium::PODIUM_SIZE] {
        // Evaluates the existing array values to find the first appropriate index
        let mut insert_index = None;
        for i in 0..podium.len() {
            if podium[i].score.is_none() || podium[i].score < new_entry.score {
                insert_index = Some(i);
                break;
            }
        }
        // Shift elements to the right of the insertion index to make room
        // for the new entry; Requires Clone implementation on Podium struct
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
        mut podium: [Podium; Self::PODIUM_SIZE],
        cheater: usize,
    ) -> [Podium; Self::PODIUM_SIZE] {
        for i in cheater..podium.len() - 1 {
            podium[i] = podium[i + 1].clone();
        }
        podium[podium.len() - 1] = Default::default();
        podium
    }

    /** Formats Podium instances for output */
    fn format(&self) -> (String, String) {
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
    pub fn print(print_all: bool, podium: &[Podium; Self::PODIUM_SIZE]) {
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
}

/** This driver illustrates an array implementation for the list ADT... using no vectors */
pub fn example() {
    // Imports the list implementation and creates default list
    //use crate::lists::array_list::Podium;
    use super::array_list::Podium;
    let mut podium = Podium::new();

    // Sample data to build a basic set of Podium instances
    let names_array: [String; 4] = [
        "Peter".to_string(),
        "Dingus".to_string(),
        "Brain".to_string(),
        "Bobson".to_string(),
    ];
    let scores_array: [usize; 4] = [1223, 34, 616, 42069];

    // Adds all known sample values to the list and prints the result
    for i in 0..names_array.len() {
        podium = Podium::add(
            podium,
            Podium::entry(names_array[i].to_string(), scores_array[i as usize]),
        );
    }
    println!("Initial results:");
    Podium::print(true, &podium);

    // Adds an entry to the middle
    println!("Add Dave\n");
    podium = Podium::add(podium, Podium::entry("Dave".to_string(), 334));

    // Removes the score at the ith index (Bobson)
    println!("Remove Bobson\n");
    podium = Podium::remove(podium, 0);

    // 6) Adds an entry to the middle
    println!("Add Dangus\n");
    podium = Podium::add(podium, Podium::entry("Dangus".to_string(), 420));

    // 7) Prints the final list and podium results
    println!("Final List:");
    Podium::print(true, &podium);
    println!("Final podium:");
    Podium::print(false, &podium);
}
