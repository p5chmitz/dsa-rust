/////////////////////////////////////////////////////////////////////////////////
/** Safe open addressing hash table with MAD compression and quadratic probing */
/////////////////////////////////////////////////////////////////////////////////

use crate::maps::hash_lib;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct Entry<K, V> {
    key: K,
    value: V,
}
impl<K, V> Entry<K, V> {
    fn new(key: K, value: V) -> Entry<K, V> {
        Entry { key, value }
    }
}
#[derive(Debug)]
pub struct ProbingHashTable<K, V> {
    data: Vec<Option<Entry<K, V>>>,
    size: usize,
}
impl<K: Debug + Hash + PartialEq, V: PartialEq> ProbingHashTable<K, V> {
    /** Constructor for an empty table with a default capacity of 2 */
    pub fn new() -> ProbingHashTable<K, V> {
        ProbingHashTable {
            data: Vec::with_capacity(2),
            size: 0,
        }
    }

    /** Returns the value associated with a key, if the key exists */
    pub fn get(&self, key: K) -> Option<&V> {
        let hashed = hash_lib::hash(&key);
        let mut location: usize = hash_lib::mad_compression(hashed, self.data.len()) as usize;
        let capacity: usize = self.data.len();
        
        // 1) Check if initial hashed/compressed location has a value
        // 2) if Some, check if the two hashed keys match, otherwise proceed to
        // probe locations until theres a match,
        // 3) return a reference to the value associated with the Entry,
        // if it exists in the table
        // 4) Return None if the key does not exist in the table
        let mut i = 1;
        while let Some(v) = &self.data[location] { // 1
            let target = hash_lib::hash(&v.key);
            let query = hash_lib::hash(&key);
            if target != query { // 2
                location = (location + i^2) % capacity; // Quadratic probe
                //location = (location + i) % capacity; // Linear probe
                i += 1;
            } else {
                return Some(&v.value); // 3
            }
        } 
        None // 4
    }

    /** Adds entry `(k, v)`, overwriting any value `v` associated with an
    existing key `k`, returns old value. Resizes the map which the
    table encounters a load factor >.75. */
    pub fn put(&mut self, key: K, value: V) {
        // Checks if the addition will bring the load factor above threshold
        if self.size == 0 || (self.size + 1) as f64 / self.data.len() as f64 > 0.5 {
            self.grow()
        }

        // Finds the correct insertion location using probing
        //let location: usize = (&self).linear_probe(&key);
        let location: usize = (&self).quadratic_probe(&key);

        // Creates a new Entry
        let entry = Entry::new(key, value);

        // Inserts the Entry
        self.data[location] = Some(entry);

        // Increase the size of the table
        self.size += 1;
    }

    pub fn remove() {}

    // Doesn't handle probing yet
    pub fn contains(&self, key: K) -> bool {
        let hashed = hash_lib::hash(&key);
        let location: usize = hash_lib::division_compression(hashed, self.data.len()) as usize;
        if let Some(bucket) = &self.data[location] {
            if bucket.key == key {
                return true;
            }
            // Required because for loop contains an early return statement
            // but does not return a value if no match is found
            false 
        } else {
            // Required because function returns and if let Some is inexhaustive
            false 
        }
    }
    
    pub fn iter() {}

    /** Linear probe calculates A[(h(k) + f(i)) mod N] where f(i) == 1 
    Assumes there is always going to be a valid index */
    fn linear_probe(&self, key: &K) -> usize {
        let capacity: usize = self.data.len() as usize;
        let mut location: usize = 
            hash_lib::mad_compression(hash_lib::hash(&key), capacity) as usize;
        while self.data[location].is_some() {
            location = (location + 1) % capacity
        }
        location
    }
    /** Linear probe calculates A[location] = A[(h(k) + f(i)) mod N], where i > 0 and f(i) == i^2 
    Assumes there is always going to be a valid index */
    pub fn quadratic_probe(&self, key: &K) -> usize {
        let capacity: usize = self.data.len() as usize;
        let mut location: usize = 
            hash_lib::mad_compression(hash_lib::hash(&key), capacity) as usize;
        let mut i = 1;
        while self.data[location].is_some() {
            location = (location + i^2) % capacity;
            i += 1;
        }
        location
    }


    /** Internal function that grows the base vector to the next prime larger than
    double the length of the original vector, rehashes and compresses hashes
    for new distribution */
    fn grow(&mut self) {
        // Create a new base vector with_capacity and resize_with to ensure all
        // indexes exist, otherwise you could push to an index that doesn't
        // exist causing a panic
        let new_capacity = hash_lib::next_prime(self.data.len() * 2);
        let mut new_base: Vec<Option<Entry<K, V>>> = Vec::with_capacity(new_capacity);

        // Vec::resize_with may result in "hidden allocation" despite description that indicates
        // that the function resizes "in place"
        new_base.resize_with(new_capacity, || None);

        // Move entries from self.data into new_base
        // For each entry in old table, calculate new location via hash/compression
        // Insert e into new_base[location] 
        for e in self.data.drain(..) {
            if e.is_some() {
                let k = &e.as_ref().unwrap().key;
                let mut location: usize = 
                    hash_lib::mad_compression(hash_lib::hash(&k), new_capacity) as usize;
                if new_base[location].is_none() {
                    new_base[location] = e;
                } else {
                    let mut i = 1;
                    while new_base[location].is_some() {
                        location = (location + i^2) % new_capacity;
                        i += 1;
                    }
                }
            }
        }
        // Update the struct instance to the new collection
        self.data = new_base;
    }

}

#[test]
fn probing_hash_table_test() {
    //Creates a new hash map
    let mut map = ProbingHashTable::<&str, u8>::new();

    // Illustrates that put() and get() work
    map.put("Peter", 41);
    assert_eq!(map.size, 1);
    assert_eq!(map.data.len(), 2);
    let fetch = map.get("Peter").unwrap();
    assert_eq!(*fetch, 41 as u8);

    // Illustrates that the map grows correctly
    map.put("Brain", 39); // Grows the map
    assert_eq!(map.data.len(), 5);
    map.put("Remus", 22);
    map.put("Bobson", 36); // Grows the map
    assert_eq!(map.data.len(), 11);
    map.put("Dingus", 18);
    map.put("Dangus", 27); // Grows the map
    assert_eq!(map.size, 6);
    assert_eq!(map.data.len(), 23);

    // Illustrates that remove() works as intended
    //assert_eq!(map.contains("Dingus"), true);
}

pub fn example() {
    //Creates a new hash map
    let mut map = ProbingHashTable::<&str, u8>::new();
    //Creates several entries
    map.put("1Peter", 41);
    map.put("2Brain", 39);
    map.put("3Remus", 22);
    map.put("4Bobson", 36);
    map.put("5Dingus", 18);
    map.put("6Dangus", 27);

    // Prints a debug version of the map
    println!("{:#?}", map);
    //let value = map.get("Peter").unwrap();
    //println!("map.get(\"Peter\"): {}", value);

    // Its now iterable!!!
    //println!("Iterating over all entries:");
    //for e in map.iter() {
    //    println!("\t{:?}", e)
    //}
    //println!("\nNow just the keys:");
    //for k in map.iter().keys() {
    //    println!("\t{}", k)
    //}
    //println!("\nNow just the values:");
    //for v in map.iter().values() {
    //    println!("\t{}", v)
    //}

    //// Does the same thing with public methods
    //let _entries: Vec<&Entry<&str, u8>> = map.entry_set();
    //let _keys: Vec<&&str> = map.key_set();
    //let _values: Vec<&u8> = map.values();

    //map.remove("Brain");
    //map.remove("Remus");
    //map.remove("Bobson");
    //map.remove("Dingus");
    //map.remove("Dangus");

    //println!("\nIts all over now:\n{:#?}", map);
}

