/////////////////////////////////////////////////////////
/** Safe chaining hash table with division compression */
/////////////////////////////////////////////////////////

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
pub struct ChainingHashTable<K, V> {
    data: Vec<Option<Vec<Entry<K, V>>>>,
    size: usize,
}
impl<K: Hash + Debug + PartialEq, V: PartialEq + Clone> ChainingHashTable<K, V> {
    /** Creates a new HashTable */
    pub fn new() -> ChainingHashTable<K, V> {
        ChainingHashTable {
            data: Vec::with_capacity(2),
            size: 0,
        }
    }

    /** Returns the number of elements in the HashTable */
    pub fn size(&self) -> usize {
        self.size
    }

    /** Returns a Boolean indicating whether the HashTable is empty */
    pub fn is_empty(&self) -> bool {
        if self.size == 0 {
            true
        } else {
            false
        }
    }

    /** Returns the value `v` associated with key `k` */
    pub fn get(&self, key: K) -> Option<&V> {
        let hashed = hash_lib::hash(&key);
        let location: usize = hash_lib::division_compression(hashed, self.data.len()) as usize;
        if let Some(bucket) = &self.data[location] {
            for e in bucket {
                let chain_key_hash = hash_lib::hash(&e.key);
                if hashed == chain_key_hash {
                    return Some(&e.value);
                }
            }
            // Required because the for loop contains early return statement
            // but does not return a value when no match is found
            None 
        } else {
            // Required because function returns and if let Some is inexhaustive
            None 
        }
    }

    /** Adds entry `(k, v)`, overwriting any value `v` associated with an
    existing key `k`, returns old value. Resizes the map which the
    table encounters a load factor >.75. */
    pub fn put(&mut self, key: K, value: V) {
        // Checks if the addition will bring the load factor above threshold
        if self.size == 0 || (self.size + 1) as f64 / self.data.len() as f64 > 0.5 {
            self.grow()
        }

        // Finds the correct insertion location
        let location: usize =
            hash_lib::division_compression(hash_lib::hash(&key), self.data.len()) as usize;

        // Creates a new Entry
        let entry = Entry::new(key, value);

        // Inserts the Entry
        match &mut self.data[location] {
            Some(v) => v.push(entry),
            None => {
                self.data[location] = Some(vec![entry]);
            }
        }
        self.size += 1;
    }

    /** Internal function that grows the base vector to the next prime larger than
    double the length of the original vector, rehashes and compresses hashes
    for new distribution */
    fn grow(&mut self) {
        // Create a new base vector with_capacity and resize_with to ensure all
        // indexes exist, otherwise you could push to an index that doesn't
        // exist causing a panic
        let new_capacity = hash_lib::next_prime(self.data.len() * 2);
        let mut new_base: Vec<Option<Vec<Entry<K, V>>>> = Vec::with_capacity(new_capacity);
        new_base.resize_with(new_capacity, || None);

        // Move entries from self.data into new_base
        for bucket in self.data.drain(..) {
            if let Some(mut chain) = bucket {
                // Vec::drain transfers ownership with no need to clone
                for entry in chain.drain(..) {
                    let rehash =
                        hash_lib::division_compression(hash_lib::hash(&entry.key), new_capacity);
                    match &mut new_base[rehash as usize] {
                        Some(existing) => existing.push(entry),
                        None => new_base[rehash as usize] = Some(vec![entry]),
                    }
                }
            }
        }

        // Update the struct instance
        self.data = new_base;
    }

    /**  Removes the entry `(k, v)` associated with key `k` */
    pub fn remove(&mut self, key: K) {
        let hashed = hash_lib::hash(&key);
        let location: usize = hash_lib::division_compression(hashed, self.data.len()) as usize;
        if let Some(bucket) = &mut self.data[location] {
            bucket.retain(|e| e.key != key);
            if bucket.is_empty() {
                self.data[location] = None; // Replace Some with None for empty buckets
            }
        }
        self.size -= 1;
    }

    /**  Returns an iterable collection of all keys in the map */
    pub fn key_set(&self) -> Vec<&K> {
        let mut v = Vec::new();
        for k in self.iter().keys() {
            v.push(k)     
        }
        v
    }

    /**  Returns an iterable collection of all values in the map, including
    repeats for multiple key-value associations */
    pub fn values(&self) -> Vec<&V> {
        let mut vec = Vec::new();
        for v in self.iter().values() {
            vec.push(v)
        }
        vec
    }

    /**  Returns an iterable collection of all `(k, v)` entries in the map */
    pub fn entry_set(&self) -> Vec<&Entry<K, V>> {
        let mut vec = Vec::new();
        for e in self.iter() {
            vec.push(e)
        }
        vec
    }

    /**  Returns a Boolean if the map contains _key_ `k`; Used to disambiguate
    the presence of a key with a null/None value */
    pub fn contains(&self, key: K) -> bool {
        let hashed = hash_lib::hash(&key);
        let location: usize = hash_lib::division_compression(hashed, self.data.len()) as usize;
        if let Some(bucket) = &self.data[location] {
            for e in bucket.iter() {
                if e.key == key {
                    return true;
                }
            }
            // Required because for loop contains an early return statement
            // but does not return a value if no match is found
            false 
        } else {
            // Required because function returns and if let Some is inexhaustive
            false 
        }
    }

    /** A function that follows the naming convention for iterable collections;
    Returns struct Iter that tracks iteration state */
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            outer: self.data.iter(),
            inner: None,
        }
    }
}

// Create an iterator as a struct to track iteration state
pub struct Iter<'a, K, V> {
    outer: std::slice::Iter<'a, Option<Vec<Entry<K, V>>>>,
    inner: Option<std::slice::Iter<'a, Entry<K, V>>>,
}
// Direct implementation for adapter methods
impl<'a, K, V> Iter<'a, K, V> {
    /// Adapter method for iterating over the keys of a map;
    /// Example:
    /// ``` for k in map.iter().keys() { ... }
    /// ```
    pub fn keys(self) -> Keys<'a, K, V> {
        Keys { iter: self }
    }
    /// Adapter method for iterating over the values of a map;
    /// Example:
    /// ``` for v in map.iter().values() { ... }
    /// ```
    pub fn values(self) -> Values<'a, K, V> {
        Values { iter: self }
    }
}
// Implement Iterator for the struct
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a Entry<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.inner {
                if let Some(entry) = inner.next() {
                    return Some(entry);
                }
            }
            // Move to the next bucket if available
            self.inner = self.outer.next()?.as_ref().map(|vec| vec.iter());
        }
    }
}
pub struct Keys<'a, K, V> {
    iter: Iter<'a, K, V>,
}
impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|entry| &entry.key)
    }
}
pub struct Values<'a, K, V> {
    iter: Iter<'a, K, V>,
}
impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|entry| &entry.value)
    }
}

#[test]
fn hash_map_test() {
    //Creates a new hash map
    let mut map = ChainingHashTable::<&str, u8>::new();

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
    assert_eq!(map.contains("Dingus"), true);
    map.remove("Dingus");
    assert_eq!(map.contains("Dingus"), false);
}

pub fn example() {
    //Creates a new hash map
    let mut map = ChainingHashTable::<&str, u8>::new();
    //Creates several entries
    map.put("Peter", 41);
    map.put("Brain", 39);
    map.put("Remus", 22);
    map.put("Bobson", 36);
    map.put("Dingus", 18);
    map.put("Dangus", 27);

    // Prints a debug version of the map
    println!("Debug print of the whole shebang:\n{:#?}", map);
    let value = map.get("Peter").unwrap();
    println!("map.get(\"Peter\"): {}", value);

    // Its now iterable!!!
    println!("Iterating over all entries:");
    for e in map.iter() {
        println!("\t{:?}", e)
    }
    println!("\nNow just the keys:");
    for k in map.iter().keys() {
        println!("\t{}", k)
    }
    println!("\nNow just the values:");
    for v in map.iter().values() {
        println!("\t{}", v)
    }

    // Does the same thing with public methods
    let _entries: Vec<&Entry<&str, u8>> = map.entry_set();
    let _keys: Vec<&&str> = map.key_set();
    let _values: Vec<&u8> = map.values();

    map.remove("Brain");
    map.remove("Remus");
    map.remove("Bobson");
    map.remove("Dingus");
    map.remove("Dangus");

    println!("\nIts all over now:\n{:#?}", map);
}
