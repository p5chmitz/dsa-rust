////////////////////////////////
/** A simple array-based list */
////////////////////////////////

// Sets list size with indexes from 0 to (PODIUM_SIZE - 1)
const PODIUM_SIZE: usize = 10;

#[derive(Debug)]
struct Entry {
    name: String,
    score: Option<usize>,
}
// Necessary for shifting entries upon insert
impl Clone for Entry {
    fn clone(&self) -> Entry {
        Entry {
            name: self.name.clone(),
            score: self.score,
        }
    }
}

/** The Podium's public API contains the following functions:
 - new() -> Podium
 - add<'a>(&mut self, name: &'a str, new_score: Option<usize>)
 - set_score(&mut self, index: usize, score: Option<usize>) -> Result<(), String>
 - remove(&mut self, cheater: usize)
 - print_full(&self, print_all: bool)

 The Podium also as the following private funcitons:
 - entry(name: String, score: Option<usize>) -> Entry
 - format(e: &Entry) -> (String, String)

NOTE: Rust requires array initializations to happen at compile time; For implementations where the
same value is used across several functions in a module you need to use a constant */
#[derive(Default)] // Required for generic array initialization
pub struct Podium {
    data: [Option<Entry>; PODIUM_SIZE],
    size: usize,
}
impl Podium {
    /** Creates a list that contains `const PODIUM_SIZE` number of elements with indexes
    from 0 to (PODIUM_SIZE - 1) */
    pub fn new() -> Podium {
        Podium {
            data: [const { None }; PODIUM_SIZE],
            size: 0,
        }
    }

    /** Adds entry to list by score to maintain a sorted list in O(n) time;
    Does not overflow with attempts that exceed the initialized structure size,
    but does not log additional entries without sufficiently high score values */
    pub fn add<'a>(&mut self, name: &'a str, new_score: Option<usize>) {
        // Evaluates the existing array values to find the first appropriate index
        let mut insert_index = None;
        for i in 0..self.data.len() {
            //if new_score.is_none() ||
            if self.data[i].is_none() || self.data[i].as_ref().unwrap().score < new_score {
                insert_index = Some(i);
                break;
            }
        }
        // Shift elements to the right of the insertion index to make room
        // for the new entry;
        if let Some(index) = insert_index {
            for j in (index..self.data.len() - 1).rev() {
                self.data[j + 1] = self.data[j].clone();
            }
            let new_entry = Self::entry(name.to_string(), new_score);
            self.data[index] = Some(new_entry);
        }
    }

    /** Attempts to write new score data to an index; Returns an error if there is no
     * data at the specified index, or if the index is out of bounds by 
     * using recycled logic and propagated errors from remove()
     *
     * NOTE: There is probably a better way to write directly to the underlying 
     * node instead of overwriting it, but then you'd have to write another set
     * of logical assertions */
    pub fn set_score(&mut self, index: usize, score: Option<usize>) -> Result<(), String> {
        // Or, if you're good at Rust
        //
        // Remove and rewrite data to the entry at the index, 
        // propagate the error if it fails
        let name = self.remove(index)?;
        self.add(&name, score);
        Ok(())
    }

    /** Removes the ith entry in O(n) time and returns the entry's name,
    shifts all remaining elements up by one index */
    pub fn remove<'a>(&mut self, index: usize) -> Result<String, String> {
        if index >= PODIUM_SIZE - 1 {
            let msg: String = format!(
                "Index out of bounds: {} is out of the range 0..={}",
                index,
                PODIUM_SIZE - 1
            );
            return Err(msg);
        }
        let entry: Entry = match self.data[index].clone() {
            Some(e) => e,
            None => return Err("No data at index".to_string()),
        };
        for i in index..self.data.len() - 1 {
            self.data[i] = self.data[i + 1].clone();
        }
        self.data[self.data.len() - 1] = None;
        Ok(entry.name)
    }

    // Private utility funcitons

    /** Constructs a new Podium entry */
    fn entry(name: String, score: Option<usize>) -> Entry {
        let score = match score {
            Some(s) => s,
            None => 0,
        };
        Entry {
            name,
            score: Some(score),
        }
    }

    /** Formats Podium instances for output */
    fn format(e: &Entry) -> (String, String) {
        let name = e.name.to_owned();
        // Required mapping for entries without scores yet
        let score = match e.score {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        // More elegant mapping with closure
        //let score = self.score.map_or("".to_string(), |s| s.to_string());
        (name, score)
    }

    /** Der listen printen; Set bool to `true` for whole list or `false` for top three */
    pub fn print_full(&self, print_all: bool) {
        let length: usize;
        if print_all == true {
            length = PODIUM_SIZE
        } else {
            // Magic listen printen number
            length = 3
        }
        for (i, entry) in self.data.iter().enumerate() {
            // Only prints the first three podium entries
            if i >= length {
                break;
            }

            let name: String = match entry.to_owned() {
                Some(s) => s.name,
                None => "".to_string(),
            };
            // Required mapping for entries without scores yet
            let score = match entry.to_owned() {
                Some(s) => s.score.expect("Bullshit").to_string(),
                None => "".to_string(),
            };

            println!("{:>2}: {:<8} {:>6}", i + 1, name, score)
        }
        println!("")
    }
}

#[test]
pub fn array_list_test() {
    use crate::array_list::Podium;

    // Creates a new list and adds some entries
    let mut pod = Podium::new();
    pod.add("Bobson", None);
    pod.add("Peter", Some(1223));
    pod.add("Brain", None);

    // Tests that the None score is below the only Some score in the list 
    assert_eq!("Bobson", &pod.remove(1).unwrap());

    // Tests the set_score function, then shifts its position and checks it
    pod.set_score(1, Some(616)).ok();
    pod.add("Remus", Some(899));
    assert_eq!("Brain", &pod.remove(2).unwrap());

    // Tests removal on an empty index
    assert_eq!("No data at index", &pod.remove(7).unwrap_err());

    // Tests OOB logic with some random usize > (PODIUM_SIZE - 1)
    let oob = 10;
    assert_eq!(
        format!("Index out of bounds: {} is out of the range 0..=9", oob),
        pod.remove(oob).unwrap_err()
    );
}

/** Silly little visual test to see all the silly little operations */
pub fn example() {
    use crate::array_list::Podium;

    let mut pod = Podium::new();
    pod.add("Bobson", None);
    pod.add("Dingus", None);
    pod.add("Dorkus", None);
    pod.print_full(false);

    let _ = pod.remove(2);
    pod.print_full(false);

    pod.add("Brain", Some(616));
    pod.print_full(false);

    pod.add("Peter", Some(1223));
    pod.print_full(false);

    pod.add("Dangus", Some(420));
    pod.print_full(false);

    let success = pod.remove(4).unwrap();
    println!("Removing an entry: \n\t{success}");

    if let Err(msg) = pod.remove(5) {
        println!("Attempting to remove an empty index: {msg}");
    }

    if let Err(msg) = pod.remove(10) {
        println!("Attempting to remove an OOB index: {msg}");
    }
}
