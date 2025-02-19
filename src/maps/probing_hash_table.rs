/////////////////////////////////////////////////////////////////////////////////
/** Safe open addressing hash table with MAD compression and quadratic probing */
/////////////////////////////////////////////////////////////////////////////////

use crate::maps::hash_lib;
use std::fmt::Debug;
use std::hash::Hash;
use rand::Rng;
use crate::maps::traits;

#[derive(Debug)]
pub struct Entry<K, V> {
    key: K,
    value: V,
}
impl<K, V> Entry<K, V> 
where 
    K: Debug + Hash + PartialEq, 
    V: PartialEq
{
    fn new(key: K, value: V) -> Entry<K, V> {
        Entry { key, value }
    }
}
#[derive(Debug)]
/** Prime, scale, and shift are used by the MAD compression algorithm and 
because they are randomly generated they must remain static for the 
lifetime of the backing structure; The grow() operation changes these values */
pub struct ProbingHashTable<K, V> {
    data: Vec<Option<Entry<K, V>>>,
    prime: usize,
    scale: usize,
    shift: usize,
    size: usize,
}
impl<K, V> ProbingHashTable<K, V> 
where 
    K: Debug + Hash + PartialEq, 
    V: PartialEq
{
    /** Constructor for an empty table with a default capacity of 2 */
    pub fn new() -> ProbingHashTable<K, V> {
        ProbingHashTable {
            data: Vec::with_capacity(2),
            prime: 13,
            scale: 5,
            shift: 7,
            size: 0,
        }
    }

    /** Returns the value associated with a key, if the key exists */
    pub fn get(&self, key: K) -> Option<&V> {
        let hashed = hash_lib::hash(&key);
        let capacity: usize = self.data.len();

        // MAD compression algorithm
        let mut location: usize = (hashed.wrapping_mul(self.scale as usize))
            .wrapping_add(self.shift)
            % (self.prime)
            % (capacity);
        
        // 1) Check if initial hashed/compressed location has a value
        // 2) if Some, check if the two hashed keys match, otherwise proceed to
        // probe locations until theres a match,
        // 3) return a reference to the value associated with the Entry,
        // if it exists in the table
        // 4) Return None if the key does not exist in the table
        let mut i: usize = 1;
        while let Some(v) = &self.data[location] { // 1
            let target = hash_lib::hash(&v.key);
            let query = hash_lib::hash(&key);
            if target != query { // 2
                location = (location + i.pow(2)) % capacity; // Quadratic probe
                //location = (location + i) % capacity; // Linear probe
                i += 1;
            } else {
                return Some(&v.value); // 3
            }
        } 
        None // 4
    }

    /** Adds entry `(k, v)`, overwriting any value `v` associated with an
    existing key `k`, returns old value. If a new addition increases the map's
    load factor above the designated threshhold of 0.5 the map resizes */
    pub fn put(&mut self, key: K, value: V) -> Option<Entry<K, V>> {
        // Checks if the addition will bring the load factor above threshold
        if ((self.size) as f64 + 1.0) / self.data.len() as f64 >= 0.5 {
            self.grow();
        }

        // Hashes the key using Rust's DefaultHasher
        let hashed = hash_lib::hash(&key);

        // Compresses the hash using the MAD algorithm
        let bucket: usize = (hashed.wrapping_mul(self.scale as usize))
            .wrapping_add(self.shift)
            % (self.prime)
            % (self.data.len());


        // Finds the correct insertion location using probing
        // Searches the map for key:
        // if >= 0, overwrite the location and return the old Entry,
        // if < 0, insert new entry at that location, return None
        let location = self.find_index(bucket, &key);

        // Creates a new Entry and inserts it
        let entry = Entry::new(key, value);
        let mut old_entry: Option<Entry<K, V>> = None;
        if location >= 0 { // Replace an entry
            //println!("COLLISION!!!! {:?}", &entry.key);
            old_entry = self.data[location as usize].take();
            self.data[location as usize] = Some(entry);
        } else { // Add a new entry
            self.data[-(location + 1) as usize] = Some(entry);
            self.size += 1;
        };
        return old_entry;
    }

    // Hashes/compresses the key and checks the associated index;
    // if Some, checks the &Entry for equivalent key, 
    //     if true, returns positive_isize representing the index, 
    //     if false, loops through the probing logic looking for equivalency,
    //     fails when the probing lands on None
    // if None, returns negative_isize representing the first available index
    fn find_index(&self, mut bucket: usize, key: &K) -> isize {
        // Quadratic probing logic
        let mut i: usize = 1;
        while let Some(entry) = &self.data[bucket] {
            if entry.key == *key {
                //println!("Match key!!!");
                return bucket as isize;
            } else {
                bucket = (bucket + i.pow(2)) % self.data.len();
                i += 1;
            }
        }
        //println!("None: {}", -(bucket as isize));
        return -(bucket as isize + 1);
    }

    // Doesn't handle probing yet
    pub fn contains(&self, key: K) -> bool {
        let hashed = hash_lib::hash(&key);

        // MAD compression algorithm
        let mut location: usize = (hashed.wrapping_mul(self.scale as usize))
            .wrapping_add(self.shift)
            % (self.prime)
            % (self.data.len());

        // Quadratic probing logic
        let mut i: usize = 1;
        while let Some(bucket) = &self.data[location] {
            if bucket.key == key {
                return true;
            } else {
                location = (location + i.pow(2)) % self.data.len();
                i += 1;
            }
        } false
    }

    // TODO
    pub fn remove() {}
    pub fn iter() {}

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
    map.put("Peter", 40);
    assert_eq!(map.size, 1);
    assert_eq!(map.data.len(), 2);
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

}

pub fn example() {
    //Creates a new hash map
    let mut map = ProbingHashTable::<&str, u8>::new();
    //Creates several entries
    map.put("Peter", 39);
    map.put("Brain", 37);
    map.put("Remus", 22);
    map.put("Bobson", 36);
    map.put("Dingus", 18);
    map.put("Dangus", 27);

    if map.contains("Peter") == false { panic!() };
    //let val = map.get("Peter").unwrap();
    //println!("Peter is: {val}");

    let new = 41;
    let old = map.put("Peter", new).unwrap().value;
    println!("Peter's age increased from {old} to {new}");

    println!("List size: {}, capacity {}", map.size, map.data.len());
    println!("Iterating over all entries:");
    for e in map.data.iter() {
        println!("\t{:?}", e)
    }

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

