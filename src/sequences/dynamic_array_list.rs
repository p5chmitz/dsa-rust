// This module didn't make the library cut
#![allow(dead_code)]

//////////////////////////////////////
/** A safe dynamic array-based list */
//////////////////////////////////////

#[derive(Debug)]
struct Entry<'a> {
    name: &'a str,
    score: Option<i32>,
}
fn build(name: &str, score: Option<i32>) -> Entry {
    Entry { name, score }
}
// Required for an easy, idiomatic re-size operation
impl<'a> Clone for Entry<'a> {
    fn clone(&self) -> Entry<'a> {
        Entry {
            name: self.name,
            score: self.score,
        }
    }
}
/// NOTE: This is mostly just a funsies excuse to illustrate dynamic sizing
#[derive(Debug)] // Required for list visualization in example function
pub struct List<'a> {
    data: Vec<Option<Entry<'a>>>,
    size: usize,
}
impl<'a> Default for List<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a> List<'a> {
    /** Creates a new generic list with capacity of 1 */
    pub fn new() -> List<'a> {
        List {
            data: vec![None; 1],
            size: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    /** Takes a name and optional score, creates a entry, and inserts it into the list;
     * If the addition places the list size at or above capacity, the function re-sizes
     * the list by a factor of two */
    pub fn insert(&mut self, name: &'a str, score: Option<i32>) {
        // Checks the list's size against its capacity and
        // grows geometrically to accommodate new entries
        if self.size + 1 >= self.data.len() {
            self.data.resize(2 * self.data.len(), None);
        }
        // Finds the (first) index that:
        // - Places the new entry at the end of the list if its score is None or
        // - Has data with a score less than or equal to the new entry's score
        let i = if score.is_none() {
            self.size
        } else {
            (0..=self.size)
                //.find(|&j| self.data[j].as_ref().map_or(true, |n| n.score <= score))
                .find(|&j| self.data[j].as_ref().is_none_or(|n| n.score <= score))
                .unwrap_or(self.size)
        };
        // Shift elements to make room for the new entry
        for j in (i..self.size).rev() {
            self.data[j + 1] = self.data[j].clone();
        }

        // Builds the entry, inserts it, and increments the list's size
        let entry: Entry = build(name, score);
        self.data[i] = Some(entry);
        self.size += 1;
    }
    /** Attempts to set an element e to the list at index i;
     * Warning: Overwrites any existing data for the specified name */
    pub fn set_score(&mut self, name: &'a str, score: Option<i32>) -> Result<(), String> {
        // Attempt to remove the existing entry by name, if it exists
        if self.remove(name).is_ok() {
            // Insert a new entry with the updated score
            self.insert(name, score);
            Ok(())
        } else {
            Err(format!("Error: {name} not on list"))
        }
    }
    /** Gets (but does not remove) the score for an input name,
     * if the name matches but there is no score, the function returns 0,
     * if there is no match on the name, function returns Err */
    //pub fn get(&self, name: &str) -> Option<i32> {
    pub fn get(&self, name: &str) -> Result<i32, &str> {
        self.data
            .iter()
            .take(self.size + 1)
            .find_map(|entry_opt| match entry_opt {
                Some(entry) if entry.name == name => match entry.score {
                    Some(score) => Some(Ok(score)),
                    None => Some(Err("No score for entry")),
                },
                _ => None,
            })
            .unwrap_or(Err("No match on name"))
    }
    /** Attempts to remove (and return) the data that matches the input name */
    pub fn remove(&mut self, name: &'a str) -> Result<&'a str, String> {
        // Uses Iterator::find() to identify the index of an entry that matches the name input;
        // No special syntax: this block has an awkwardly long find expression
        if let Some(i) = (0..=self.size).find(|&i| {
            //self.data[i]
            //    .as_ref()
            //    .map_or(false, |entry| entry.name == name) // Finds matching name or returns false
            self.data[i]
                .as_ref()
                .is_some_and(|entry| entry.name == name) // Finds matching name or returns false
        }) {
            // If a match is found shift entries to the left to fill the gap
            for j in i..self.size {
                self.data[j] = self.data[j + 1].clone();
            }
            // Decrement the list's size, call the trim function, and return the name
            self.size -= 1;
            self.trim();
            Ok(name)
        // If no match is found the function surfaces an Err
        } else {
            let err = format!("No match on name {name}");
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

    // Utility functions

    /** Prints the Podium list; If you supply true the function prints the entire list,
    if you supply false the function just prints the top three spots */
    pub fn print_full(&self, print_all: bool) {
        let length = if print_all { self.data.len() } else { 3 };
        for (i, entry) in self.data.iter().enumerate() {
            // Only prints the first three podium entries
            if i >= length {
                break;
            }
            let mut fmtd: (String, String) = ("".to_string(), "".to_string());
            if let Some(e) = entry {
                fmtd = self.format(e);
                println!("{:>2}: {:<9} {:>6}", i + 1, fmtd.0, fmtd.1);
            } else {
                println!("{:>2}: {:<8} ", i + 1, fmtd.0)
            }
        }
        println!()
    }
    /** Formats PodiumEntry instances for output */
    fn format(&self, entry: &Entry) -> (String, String) {
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
fn basic_function_test() {
    // Creates new list
    let mut list: List = List::new();

    // Tests insert and get on a new list
    list.insert("Chester", Some(30));
    let reference: i32 = list.get("Chester").unwrap();
    assert_eq!(reference, 30);
    assert_eq!(list.size, 1);
    assert_eq!(list.data.len(), 2);

    // Tests scoreless inserts and gets -- handling scoreless entries is up to the calling code
    list.insert("Copperpot", None);
    assert!(list.get("Copperpot").is_err());
    let score: (i32, &str) = match list.get("Copperpot") {
        Ok(s) => (s, "Found something that shouldn't be here"),
        Err(e) => (0, e),
    };
    assert_eq!(score, (0, "No score for entry"));

    // Tests set_score on valid and invalid list entries
    assert!(list.set_score("Copperpot", Some(25)).is_ok());
    assert!(list.set_score("Doingus", Some(25)).is_err());
    let msg = list.set_score("Blongus", Some(100));
    assert_eq!(msg, Err("Error: Blongus not on list".to_string()));

    // Tests get on entires not in the list
    assert!(list.get("Peter").is_err());
    let score: (i32, &str) = match list.get("Peter") {
        Ok(s) => (s, "Found something that shouldn't be here"),
        Err(e) => (0, e),
    };
    assert_eq!(score, (0, "No match on name"));

    // Tests automatic list re-sizing
    list.insert("Peter", Some(45));
    assert_eq!(list.size, 3);
    assert_eq!(list.data.len(), 4);

    // Prep fluf
    list.insert("Brain", Some(45));
    list.insert("Dingus", Some(3));
    list.insert("Dangus", Some(87));
    assert_eq!(list.size, 6);
    assert_eq!(list.data.len(), 8);

    // Tests remove on valid and invalid entries
    let name: Result<&str, String> = list.remove("Chester");
    assert_eq!(name, Ok("Chester"));
    let name: Result<&str, String> = list.remove("Remus");
    assert_eq!(name, Err("No match on name Remus".to_string()));

    // Tests that list auto-resizes on removal too
    let _ = list.remove("Copperpot");
    list.remove("Dingus").ok(); // More idiomatic way to neglect error handling
    assert_eq!(list.size, 3);
    assert_eq!(list.data.len(), 8);
}

/** Mostly for print debugging and example usage */
pub fn example() {
    // Creates new list
    let mut list: List = List::new();

    // Inserts some entries
    list.insert("Chester", None);
    list.insert("Copperpot", Some(30));
    list.insert("Peter", Some(40));

    println!("The initial list:");
    list.print_full(true);

    list.set_score("Chester", Some(35)).ok();
    println!("The list after setting Chester's score:");
    list.print_full(true);

    // Removes some entries with calling code options
    // 1) Basic match block
    let mut name: &str = "Peter";
    let result: String = match list.remove(name) {
        Ok(_) => "Success".to_string(),
        Err(e) => e,
    };
    println!("Attempt to remove {name}: {result}");
    // 2) if let syntax
    name = "Remus";
    if let Err(result) = list.remove(name) {
        println!("Attempt to remove {name}: {result}")
    }
    // 3) silent
    name = "Chester";
    println!("(Silent) attempt to remove {name}: 🤫");
    list.remove("Chester").ok();

    // Using unwrap casuses panic on error
    //list.remove("David").unwrap();

    println!("The final list:");
    list.print_full(true);
}
