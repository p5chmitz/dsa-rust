/*! Safe open addressing hash table with MAD compression and quadratic probing 

```rust

    use dsa_rust::maps::probing_hash_table::ProbingHashTable;

    //Creates a new hash map with str keys and u8 values
    let mut map = ProbingHashTable::<&str, u8>::new();

    println!("Map stats: size: {}, capacity: {}, active entries: {}",
        map.size(),
        map.len(),
        map.entries()
    );

    // Puts some entries into the map
    println!("Building the map...");
    let mut names: Vec<&str> = vec!["Peter", "Brain", "Remus", "Bobson", "Dingus", "Dangus"];
    let values: Vec<u8> = vec![39, 37, 22, 36, 18, 27];
    for (k, v) in names.iter().zip(values.into_iter()) {
        map.put(k, v);
    }

    // Checks that the map contains what we'd expect
    if map.contains("Peter") == false {
        panic!()
    };
    let val = map.get("Peter").unwrap();
    println!("Peter is {val}");

    // Replaces a value for a given key and
    // checks that the new value took
    let new = 41;

    //let old = map.put("Peter", new).unwrap().value;
    let old = map.get("Peter").unwrap().clone();
    map.put("Peter", new);

    println!("[Peter's age increased from {old} to {new}]");
    let val = map.get("Peter").unwrap();
    println!("Uhhhhh, I meant Peter is {val}");

    // Shows the map and its data
    println!(
        "\nMap stats: size: {}, capacity: {}, active entries: {}",
        map.size(),
        map.len(),
        map.entries()
    );
    map.contents();

    // Illustrates removing entries
    println!("\nThere can be only one!");
    names.remove(0);
    for e in names {
        let removed = map.remove(e);
        if let Some(entry) = removed {
            println!("Remove: {}", entry.key());
        }
    }

    // The final result
    println!("\nMap stats: size: {}, capacity: {}, active entries: {}",
        map.size(),
        map.len(),
        map.entries()
    );
    map.contents();
```

*/

use crate::maps::hash_lib;
//use crate::maps::traits;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq)]
/// Contains the actual (generic) key:value pair
pub struct Entry<K, V> {
    key: K,
    value: V,
}
impl<K, V> Entry<K, V>
where
    K: Clone + Debug + Hash + PartialEq,
    V: Clone + PartialEq,
{
    /// Constructs a new Entry
    fn new(key: K, value: V) -> Entry<K, V> {
        Entry { key, value }
    }

    /// Returns the key from an Entry
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Returns the value from an Entry
    pub fn value(&self) -> &V {
        &self.value
    }

}
#[derive(Debug)]
/** Prime, scale, and shift are used by the MAD compression algorithm and
because they are randomly generated they must remain static for the
lifetime of the backing structure; The grow() operation changes these values */
pub struct ProbingHashTable<K, V> {
    data: Vec<Option<Entry<K, V>>>,
    ctrl: Vec<u8>,
    prime: usize,
    scale: usize,
    shift: usize,
    size: usize,
    entries: usize,
}
impl<K, V> ProbingHashTable<K, V>
where
    K: Clone + Debug + Hash + PartialEq,
    V: Clone + PartialEq  + std::fmt::Debug,
{
    /// Constructor for an empty table with a default capacity of 2
    pub fn new() -> ProbingHashTable<K, V> {
        let new_capacity = 2;
        let mut table = ProbingHashTable {
            data: Vec::with_capacity(new_capacity),
            ctrl: Vec::with_capacity(new_capacity),
            prime: 13,
            scale: 5,
            shift: 7,
            size: 0,
            entries: 0,
        };
        // Initialize storage to ensure access
        table.data.resize_with(new_capacity, || None);
        table.ctrl.resize_with(new_capacity, || 0x00);
        table
    }

    /// Returns the total number of entries in the map minus the "deleted" entries
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the capacity of the map
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the current number of active entires in the map
    pub fn entries(&self) -> usize {
        self.entries
    }

    /// Pretty-prints the map's contents
    pub fn contents(&self) {
        for (e, m) in self.data.iter().zip(self.ctrl.iter()) {
            println!("\t{:>3}: {:?}", m, e)
        }
    }
    
    /// Takes a key and returns a Boolean indicating whether its in the map
    pub fn contains(&self, key: K) -> bool {
        let hash = Self::hash(&key);
        let mut location = self.compress(hash);

        // Quadratic probing logic
        let mut i: usize = 1;
        while let Some(bucket) = &self.data[location] {
            if bucket.key == key && self.ctrl[location] == 0x01 {
                return true;
            } else {
                location = (location + i.pow(2)) % self.data.len();
                i += 1;
            }
        }
        false
    }

    /// Returns the value associated with a key, if the key exists
    pub fn get(&self, key: K) -> Option<&V> {
        // Hashes and compresses key to get the initial bucket
        let hash = Self::hash(&key);
        let bucket = self.compress(hash);

        // find_index() uses quadratic probing
        let location = self.find_index(bucket, &key);
        if location >= 0 {
            let value = &self.data[location as usize].as_ref().unwrap().value;
            return Some(value);
        } else {
            None
        }
    }

    /** Adds entry `(k, v)`, overwriting any value `v` associated with an
    existing key `k`, returns old value. If a new addition increases the map's
    load factor above the designated threshhold of 0.5 the map resizes */
    pub fn put(&mut self, key: K, value: V) -> Option<Entry<K, V>> {
        // Checks if the addition will bring the load factor above threshold
        if ((self.size) as f64 + 1.0) / self.data.len() as f64 >= 0.5 {
            self.grow();
        }

        // Hashes and compresses key to get the initial bucket
        let hash = Self::hash(&key);
        let bucket = self.compress(hash);

        // Finds the correct insertion location using probing
        // Searches the map for key:
        // if >= 0, overwrite the location and return the old Entry,
        // if < 0, insert new entry at that location, return None
        let location = self.find_index(bucket, &key);

        // Creates a new Entry and inserts it
        let entry = Entry::new(key, value);
        let mut old_entry: Option<Entry<K, V>> = None;
        if location >= 0 {
            // Replace an entry
            //println!("COLLISION!!!! {:?}", &entry.key);
            old_entry = self.data[location as usize].take();
            self.data[location as usize] = Some(entry);
        } else {
            // Add a new entry
            self.data[-(location + 1) as usize] = Some(entry);
            self.ctrl[-(location + 1) as usize] = 0x01;
            self.size += 1;
            self.entries += 1;
        };
        return old_entry;
    }

    /** Removes an entry from the map by key; The secret, though, is that
    the data "leaks", meaning its no longer available but it doesn't
    technically get removed either ¯\_(ツ)_/¯ */
    pub fn remove(&mut self, key: K) -> Option<Entry<K, V>> {
        let hash = Self::hash(&key);
        let bucket = self.compress(hash);
        let location = self.find_index(bucket, &key);

        let mut entry = None;
        if location >= 0 {
            //TODO Figure out how to remove clone()
            // Cannot use take() because that leaves the index
            // as None which breaks probing logic
            entry = self.data[location as usize].clone();
            //entry = self.data[location as usize].take();
            self.ctrl[location as usize] = 0xBB; // Mark with tombstone
        }
        self.entries -= 1;
        entry
    }

    /// Warning: Unimplemented
    pub fn key() {}

    /// Warning: Unimplemented
    /// Returns the value for a given key
    pub fn value(&self) {}

    // UTILITY FUNCTIONS

    // Hashes/compresses the key and checks the associated index;
    // if Some, checks the &Entry for equivalent key,
    //     if true, returns positive_isize representing the index,
    //     if false, loops through the probing logic looking for equivalency,
    //     fails when the probing lands on None
    // if None, returns negative_isize representing the first available index
    fn find_index(&self, bucket: usize, key: &K) -> isize {
        let mut i: usize = 1;
        let mut current_bucket = bucket;

        // Quadratic probing logic
        while let Some(entry) = &self.data[current_bucket] {
            if entry.key == *key {
                return current_bucket as isize;
            } else {
                current_bucket = (current_bucket + i.pow(2)) % self.data.len();
                i += 1;
            }
        }
        return -(current_bucket as isize + 1);
    }

    /** Hashes the key using Rust's DefaultHasher */
    fn hash(key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher); // Hash::hash()
        hasher.finish() as usize // Hasher::finish()
    }

    // Compresses the hash using the MAD algorithm
    fn compress(&self, hash: usize) -> usize {
        (hash.wrapping_mul(self.scale as usize)).wrapping_add(self.shift)
            % (self.prime)
            % (self.data.len()) as usize
    }

    /** Internal function that grows the base (storage) vector to the next prime
    larger than double the length of the original vector, rehashes and compresses
    hashes for new distribution */
    fn grow(&mut self) {
        // Create a new base vector with_capacity() and resize_with() to ensure
        // all indexes exist, otherwise you could push to an index that doesn't
        // exist causing a panic
        // NOTE: Vec::resize_with() may result in "hidden allocation" despite description
        // that indicates that the function resizes "in place", initializes
        // new_base with all None values
        let new_capacity = hash_lib::next_prime(self.data.len() * 2);
        let mut new_base: Vec<Option<Entry<K, V>>> = Vec::with_capacity(new_capacity);
        new_base.resize_with(new_capacity, || None);
        let mut new_ctrl: Vec<u8> = Vec::with_capacity(new_capacity);
        new_ctrl.resize_with(new_capacity, || 0x00);

        println!("Growing from {} to {}", self.data.len(), new_capacity);

        // Reset the instance values for MAD compression
        // Finds a prime > len, starting much larger to ensure even spread
        self.prime = hash_lib::next_prime(new_capacity * 2);
        let mut rng = rand::rng(); // Thread-local RNG
        self.scale = rng.random_range(1..=self.prime - 1);
        self.shift = rng.random_range(0..=self.prime - 1);

        // Move entries from self.data into new_base
        // For each Some entry in old table, calculate new location
        // via hash/compression and insert the entry into new_base[location]
        for e in &mut self.data {
            if let Some(v) = e.take() {
                // MAD compression algorithm
                let mut location: usize = ((hash_lib::hash(&v.key))
                    .wrapping_mul(self.scale as usize))
                .wrapping_add(self.shift)
                    % (self.prime)
                    % (new_capacity);

                // Handle potential collisions for new vec insert
                let mut i: usize = 0;
                while new_base[location].is_some() {
                    location = (location + i.pow(2)) % new_capacity;
                    i += 1;
                }
                new_base[location] = Some(v);
                new_ctrl[location] = 0x01;
            }
        }

        //Update the struct instance to the new collections
        self.data = new_base;
        self.ctrl = new_ctrl;
    }
}

#[test]
fn probing_hash_table_test() {
    //Creates a new hash map
    let mut map = ProbingHashTable::<&str, u8>::new();

    assert_eq!(map.data.len(), 2);
    assert_eq!(map.ctrl.len(), 2);

    // Illustrates that put() and get() work
    map.put("Peter", 40);
    assert_eq!(map.size, 1);
    assert_eq!(map.data.len(), 5);
    assert_eq!(map.ctrl.len(), 5);

    let fetch = map.get("Peter").unwrap();
    assert_eq!(*fetch, 40 as u8);

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

    // Illustrates that contains() works as intended
    assert_eq!(map.contains("Dingus"), true);

    // Illustrates that put() returns old values and
    // overwrites existing values upon collision...
    let collision = map.put("Peter", 41).unwrap();
    assert_eq!(collision.value, 40 as u8);
    let new_val = map.get("Peter").unwrap();
    assert_eq!(*new_val, 41 as u8);
    // Without increasing the list size
    assert_eq!(map.size, 6);

    // Illustrates that removes entries by key and returns the value
    assert!(map.contains("Dangus"));
    let removed = map.remove("Dangus").unwrap();
    assert_eq!(
        removed,
        Entry {
            key: "Dangus",
            value: 27
        }
    );
    assert!(!map.contains("Dangus"));
}

pub fn example() {
    //Creates a new hash map
    let mut map = ProbingHashTable::<&str, u8>::new();

    let s = format!(
        "Map stats: size: {}, capacity: {}, active entries: {}",
        map.size,
        map.data.len(),
        map.entries
    );
    let l = "=".repeat(s.len());
    println!("{l}\n{s}");

    // Puts some entries into the map
    println!("Building the map...");
    let mut names: Vec<&str> = vec!["Peter", "Brain", "Remus", "Bobson", "Dingus", "Dangus"];
    let values: Vec<u8> = vec![39, 37, 22, 36, 18, 27];
    for (k, v) in names.iter().zip(values.into_iter()) {
        map.put(k, v);
    }

    // Checks that the map contains what we'd expect
    if map.contains("Peter") == false {
        panic!()
    };
    let val = map.get("Peter").unwrap();
    println!("Peter is {val}");

    // Replaces a value for a given key and
    // checks that the new value took
    let new = 41;
    let old = map.put("Peter", new).unwrap().value;
    println!("[Peter's age increased from {old} to {new}]");
    let val = map.get("Peter").unwrap();
    println!("Uhhhhh, I meant Peter is {val}");

    // Shows the map and its data
    println!(
        "\nMap stats: size: {}, capacity: {}, active entries: {}",
        map.size,
        map.data.len(),
        map.entries
    );
    for (e, m) in map.data.iter().zip(map.ctrl.iter()) {
        println!("\t{:>3}: {:?}", m, e)
    }

    // Illustrates removing entries
    println!("\nThere can be only one!");
    names.remove(0);
    for e in names {
        println!("Remove: {}", map.remove(e).unwrap().key);
    }

    // The final result
    let s = format!(
        "\nMap stats: size: {}, capacity: {}, active entries: {}",
        map.size,
        map.data.len(),
        map.entries
    );
    println!("{s}");
    for (e, m) in map.data.iter().zip(map.ctrl.iter()) {
        println!("\t{:>3}: {:?}", m, e)
    }

    let l = "=".repeat(s.len());
    println!("\n{l}");
}
