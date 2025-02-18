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
    pub fn insert(&mut self, key: K, value: V) {
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

    // TODO
    pub fn remove() {}

    // Doesn't handle probing yet
    pub fn contains(&self, key: K) -> bool {
        let hashed = hash_lib::hash(&key);

        // MAD compression algorithm
        let mut location: usize = (hashed.wrapping_mul(self.scale as usize))
            .wrapping_add(self.shift)
            % (self.prime)
            % (self.data.len());

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
    
    pub fn iter() {}

    /** Linear probe calculates A[(h(k) + f(i)) mod N] where f(i) == 1 
    Assumes there is always going to be a valid index */
    // NOTE: Just for illustration, this module uses quadratic probing
    fn linear_probe(&self, key: &K) -> usize {
        let capacity: usize = self.data.len() as usize;
        let hashed = hash_lib::hash(&key);

        // MAD compression algorithm
        let mut location: usize = (hashed.wrapping_mul(self.scale as usize))
            .wrapping_add(self.shift)
            % (self.prime)
            % (capacity);

        while self.data[location].is_some() {
            location = (location + 1) % capacity
        }
        location
    }
    /** Linear probe calculates A[location] = A[(h(k) + f(i)) mod N], where i > 0 and f(i) == i^2 
    Assumes there is always going to be a valid index */
    pub fn quadratic_probe(&self, key: &K) -> usize {
        let capacity: usize = self.data.len() as usize;
        let hashed = hash_lib::hash(&key);

        // MAD compression algorithm
        let mut location: usize = (hashed.wrapping_mul(self.scale as usize))
            .wrapping_add(self.shift)
            % (self.prime)
            % (capacity);

        let mut i: usize = 1;
        while self.data[location].is_some() {
            location = (location + i.pow(2)) % capacity;
            i += 1;
        }
        location
    }

    /** Returns the  */
    // TODO
    fn find_slot(&self,) {}

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
                let mut location: usize = hash_lib::mad_compression_1(
                    hash_lib::hash(&v.key), 
                    self.prime, 
                    self.scale, 
                    self.shift, 
                    new_capacity) as usize;
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

    // Illustrates that insert() and get() work
    map.insert("Peter", 41);
    assert_eq!(map.size, 1);
    assert_eq!(map.data.len(), 2);
    let fetch = map.get("Peter").unwrap();
    assert_eq!(*fetch, 41 as u8);

    // Illustrates that the map grows correctly
    map.insert("Brain", 39); // Grows the map
    assert_eq!(map.data.len(), 5);
    map.insert("Remus", 22);
    map.insert("Bobson", 36); // Grows the map
    assert_eq!(map.data.len(), 11);
    map.insert("Dingus", 18);
    map.insert("Dangus", 27); // Grows the map
    assert_eq!(map.size, 6);
    assert_eq!(map.data.len(), 23);
    
    // Illustrates that remove() works as intended
    assert_eq!(map.contains("Dingus"), true);
    let fetch = map.get("Peter").unwrap();
    assert_eq!(*fetch, 41 as u8);

}

pub fn example() {
    //Creates a new hash map
    let mut map = ProbingHashTable::<&str, u8>::new();
    //Creates several entries
    map.insert("Peter", 41);
    map.insert("Brain", 39);
    map.insert("Remus", 22);
    map.insert("Bobson", 36);
    map.insert("Dingus", 18);
    map.insert("Dangus", 27);

    // Prints a debug version of the map
    //println!("{:#?}", map);
    println!("prime: {}\nscale: {}\nshift: {}\ncapacity: {}\nsize: {}", 
        map.prime, 
        map.scale, 
        map.shift, 
        map.data.len(), 
        map.size);

    if map.contains("Peter") == false { panic!() };

    if let Some(value) = map.get("1Peter") {
        println!("map.get(\"1Peter\"): {}", value)
    };

    // Its now iterable!!!
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

