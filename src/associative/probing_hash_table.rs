/*! Safe, open addressing hash table with MAD compression and quadratic probing

# About
This hash map implementation uses open-addressing to take advantage of cache locality, and relies on a "multiply, add, and divide" (MAD) compression algorithm with quadratic probing to handle collisions.

This structure provides the basis for several other structures in this library, including the [PriorityQueue<K, P>](crate::composite::priority_queue) and [HashSet\<T>](crate::associative::hash_set) implementations.

# Design
This structure uses Rust's [default hasher](std::hash::DefaultHasher) to hash keys. The default hasher currently represents a cryptographically strong [SipHash-1-3](https://en.wikipedia.org/wiki/SipHash) design as of the publication date (9/2025). In practice, hashing small key objects <= 128 bytes provide similar performance as ULID generation, but hashing arbitrarily large key objects may incur performance penalties.

The primary [HashMap] struct uses a private byte mask (as a "control" list) to track positions with valid ("live"), empty, and removed ("tombstone") entries. Using a conventional free list and overwriting data for a marked slot would break the quadratic probing logic.

```text
+-------------------------------------------------+
| ctrl | Option<Entry<K, V>>                      |
|------+------------------------------------------|
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    1 | Some(Entry { key: "Dingus", value: 18 }) |
|    1 | Some(Entry { key: "Brain", value: 37 })  |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    1 | Some(Entry { key: "Remus", value: 22 })  |
|  187 | None                                     |
|    0 | None                                     |
|    1 | Some(Entry { key: "Bobson", value: 36 }) |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|  187 | None                                     |
+-------------------------------------------------+
```

## Insertion
The structure contains two primary ways to insert entries into the map. 

The [put()](crate::associative::probing_hash_table::HashMap::put) operation overwrites values for existing keys, but does not mutate the existing key. This keeps with the standard library's implementation where two keys can be `==` without being identical. 

```rust
    use dsa_rust::associative::probing_hash_table::HashMap;

    // Build a map with put()
    let mut map = HashMap::<&str, u8>::new();
    let mut names: Vec<&str> = vec!["Peter", "Brain", "Remus", "Bobson", "Dingus", "Dangus"];
    let values: Vec<u8> = vec![39, 37, 22, 36, 18, 27];
    for (k, v) in names.iter().zip(values.into_iter()) {
        map.put(k, v);
    }
```

The structure also provides a [mut_val_or()](crate::associative::probing_hash_table::HashMap::mut_val_or) operation for mutating the value(s) of the map or inserting a default value. This is useful with the structures iterators to make linear passes over source data during mapping.


```rust
    use dsa_rust::associative::probing_hash_table::HashMap;

    // Build a map with mut_val_or()
    let mut map = HashMap::<char, u8>::new();
    let word: &str = "Hello, sickos!";
    for k in word.chars() {
        map.mut_val_or(k, |x| *x += 1, 1);
    }

```

## Removal & Rehashing
The structure contains a [remove()](crate::associative::probing_hash_table::HashMap::remove) operation that returns an owned entry if it exists in the map. The `remove()` operation marks the index with a tombstone, and uses [std::mem::take] to remove and return the value. Even though `take()` swaps out a `None` variant the index remains unusable until the calling code invokes a rehash operation. This is done in order to retain probing logic. For long-lived maps with many removals/tombstones, the structure provides two re-hashing operations. The [rehash()](crate::associative::probing_hash_table::HashMap::rehash) operation rehashes the entire map (freeing up tombstones) _at the current underlying structure's capacity_. This essentially allows you to permanently remove deleted entries and reduce the map's load factor. Conversely, the [shrink_to_fit()](crate::associative::probing_hash_table::HashMap::shrink_to_fit) operation rehashes the map (freeing up tombstones) down to the minimum backing capacity that still provides a load factor of _<= .5_. Both processes necessarily require _O(n)_ time where _n_ is the number of live entries in the map.

Example of a map that has had many entries removed:

```text
+-------------------------------------------------+
| ctrl | Option<Entry<K, V>>                      |
|------+------------------------------------------|
|    1 | Some(Entry { key: 'o', value: 2 })       |
|  187 | None                                     |
|    0 | None                                     |
|  187 | None                                     |
|    1 | Some(Entry { key: 'l', value: 2 })       |
|  187 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|  187 | None                                     |
|  187 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    1 | Some(Entry { key: 'H', value: 1 })       |
|    0 | None                                     |
|  187 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    1 | Some(Entry { key: 'e', value: 1 })       |
|  187 | None                                     |
|    0 | None                                     |
+-------------------------------------------------+
```

This example illustrates how the map might look after a `rehash()`:

```text
+-------------------------------------------------+
| ctrl | Option<Entry<K, V>>                      |
|------+------------------------------------------|
|    1 | Some(Entry { key: 'H', value: 1 })       |
|    1 | Some(Entry { key: 'o', value: 2 })       |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    1 | Some(Entry { key: 'l', value: 2 })       |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
|    1 | Some(Entry { key: 'e', value: 1 })       |
|    0 | None                                     |
|    0 | None                                     |
+-------------------------------------------------+
```

This example illustrates how the map might look after a `shrink_to_fit()`:

```text
+-------------------------------------------------+
| ctrl | Option<Entry<K, V>>                      |
|------+------------------------------------------|
|    1 | Some(Entry { key: 'H', value: 1 })       |
|    0 | None                                     |
|    0 | None                                     |
|    1 | Some(Entry { key: 'l', value: 2 })       |
|    1 | Some(Entry { key: 'o', value: 2 })       |
|    0 | None                                     |
|    0 | None                                     |
|    1 | Some(Entry { key: 'e', value: 1 })       |
|    0 | None                                     |
|    0 | None                                     |
|    0 | None                                     |
+-------------------------------------------------+
```

# Example

```rust

    use dsa_rust::associative::probing_hash_table::HashMap;

    //Creates a new hash map with str keys and u8 values
    let mut map = HashMap::<&str, u8>::new();

    println!("Map stats: size: {}, capacity: {}, active entries: {}",
        map.occupied(),
        map.capacity(),
        map.len()
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
        map.occupied(),
        map.capacity(),
        map.len()
    );
    map.contents();

    // Illustrates removing entries
    println!("\nThere can be only one!");
    names.remove(0);
    for e in names {
        let removed = map.remove(&e);
        if let Some(entry) = removed {
            println!("Remove: {}", entry.key());
        }
    }

    // The final result
    println!("\nMap stats: size: {}, capacity: {}, active entries: {}",
        map.occupied(),
        map.capacity(),
        map.len()
    );
    map.contents();
```

*/

use crate::associative::hash_lib;
//use crate::maps::traits;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq)]
// Contains the actual key:value pair
pub struct Entry<K, V> {
    key: K,
    value: V,
}
impl<K, V> Entry<K, V>
where
    K: Debug + Hash + PartialEq,
    V: Debug,
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
impl<K, V> Default for Entry<K, V>
where
    K: Default + Debug + Hash + PartialEq,
    V: Debug + Default,
{
    fn default() -> Self {
        Self {
            key: K::default(),
            value: V::default(),
        }
    }
}

// Byte masks were deemed more performant, but this is valuable historic pattern
//#[derive(Debug, PartialEq)]
//pub enum Entry<K, V> {
//    Live { key: K, value: V },
//    Tombstone,
//}
//impl<K, V> Entry<K, V>
//where
//    K: Debug + Hash + PartialEq,
//    V: PartialEq,
//{
//    /// Constructs a new Entry
//    fn new(key: K, value: V) -> Entry<K, V> {
//        Entry::Live { key, value }
//    }
//
//    /// Returns the key from an Entry
//    pub fn key(&self) -> Option<&K> {
//        match self {
//            Entry::Live { key, .. } => Some(key),
//            Entry::Tombstone => None,
//        }
//    }
//
//    /// Returns the value from an Entry
//    pub fn value(&self) -> Option<&V> {
//        match self {
//            Entry::Live { value, .. } => Some(value),
//            Entry::Tombstone => None,
//        }
//    }
//
//    /// Entry adapter that takes a closure to mutate the value of an entry, if it exists.
//    pub fn mut_val<F>(&mut self, f: F)
//    where
//        F: FnOnce(&mut V),
//    {
//        if let Entry::Live { value, .. } = self {
//            f(value);
//        } else {
//            // No-op: Entry is a Tombstone and that would break the probing logic.
//        }
//    }
//}

#[derive(Debug)]
pub struct HashMap<K, V> {
    pub data: Vec<Option<Entry<K, V>>>, // The primary memory backing
    pub ctrl: Vec<u8>,                  // A byte mask to identify available positions
    size: usize,                     // The total number of entries in the map (live + deleted)
    live: usize,                    // The number of "live" entries in the map
                 
    // NOTE: Prime, scale, and shift are used by the MAD compression algorithm.
    // These randomly-generated values must remain static for the
    // lifetime of the backing structure; The grow() operation changes these values
    prime: usize,
    scale: usize,
    shift: usize,
}
impl<K, V> Default for HashMap<K, V>
where
    K: Debug + Default + Eq + Hash + PartialEq,
    V: Default + PartialEq + std::fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<K, V> HashMap<K, V>
where
    K: Debug + Eq + Hash + PartialEq,
    V: std::fmt::Debug,
{
    /// Constructor for an empty map with a default capacity of 2.
    pub fn new() -> Self {
        let new_capacity = 2;
        let mut table = Self {
            data: Vec::with_capacity(new_capacity),
            ctrl: Vec::with_capacity(new_capacity),
            prime: 13,
            scale: 5,
            shift: 7,
            size: 0,
            live: 0,
        };
        // Initialize storage to ensure access
        table.data.resize_with(new_capacity, || None);
        table.ctrl.resize_with(new_capacity, || 0x00);
        table
    }

    /// Constructor for an empty map with a given capacity.
    pub fn new_with_capacity(size: usize) -> Self {
        let mut table = Self {
            data: Vec::with_capacity(size),
            ctrl: Vec::with_capacity(size),
            prime: 13,
            scale: 5,
            shift: 7,
            size: 0,
            live: 0,
        };
        // Initialize storage to ensure access
        table.data.resize_with(size, || None);
        table.ctrl.resize_with(size, || 0x00);
        table
    }

    /// Returns the number of active entries in the map in _O(1)_ time.
    pub fn len(&self) -> usize {
        self.live
    }

    /// Returns the total number of entries in the map (active + deleted entries) in _O(n)_ time.
    pub fn occupied(&self) -> usize {
        let mut occupied = 0;
        for e in self.ctrl.iter() {
            if *e == 1 || *e == 187 { occupied += 1 }
        }
        occupied
    }

    /// Returns the total number of _deleted_ entries in the map in _O(n)_ time.
    pub fn deleted(&self) -> usize {
        let mut occupied = 0;
        for e in self.ctrl.iter() {
            if *e == 187 { occupied += 1 }
        }
        occupied
    }

    /// Returns the total number of available slots in the map in _O(1)_ time. 
    ///
    /// NOTE: The load factor is the quotient of `len() / capacity()`.
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the map is either empty or contains only empty slots and deleted entries.
    pub fn is_empty(&self) -> bool {
        self.live == 0 || self.data.is_empty()
    }

    /// Pretty-prints the map's contents.
    pub fn contents(&self) {
        for (e, m) in self.data.iter().zip(self.ctrl.iter()) {
            println!("  {m:>3}: {e:?}")
        }
    }

    /// Takes a key as a reference and returns a Boolean indicating whether its in
    /// the map. The expected temporal complexity is _O(1)_, as the map
    /// maintains a [laod factor](https://www.headyimage.com/cs/dsa/maps/#collision-handling-schemes) of <=.5.
    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: Debug + Hash + Eq + ?Sized,
    {
        self.find_index(key) >= 0
    }

    /// Returns a reference to the value associated with a key, if the key exists.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    //pub fn get(&self, key: &K) -> Option<&V> {
    where
        K: std::borrow::Borrow<Q>,
        Q: Debug + Hash + Eq + ?Sized,
    {
        // find_index() uses quadratic probing
        let location = self.find_index(key);
        if location >= 0 {
            let value = &self.data[location as usize].as_ref().unwrap().value;
            Some(value)
        } else {
            None
        }
    }

    /// Adds entry `(k, v)`, overwriting any value `v` associated with an
    /// existing key `k`, returns old value. If a new addition increases
    /// the map's load factor above the designated threshhold of 0.5
    /// the map resizes.
    pub fn put(&mut self, key: K, value: V) -> Option<Entry<K, V>> 
    //where 
    //    K: std::default::Default,
    {
        // Checks if the addition will bring the load factor above threshold
        if ((self.size) as f64 + 1.0) / self.data.len() as f64 >= 0.5 {
            self.grow();
        }

        let location = self.find_index(&key);

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
            self.live += 1;
        };
        old_entry
    }

    // Attempts to add entry `(k, v)` to the map, if key `k` does not
    // already exist. If a new addition increases
    // the map's load factor above the designated threshhold of 0.5
    // the map resizes.
    pub fn try_put(&mut self, key: K, value: V) {
        // Checks if the addition will bring the load factor above threshold
        if ((self.size) as f64 + 1.0) / self.data.len() as f64 >= 0.5 {
            self.grow();
        }
    
        let location = self.find_index(&key);
    
        // Avoids duplicates all together
        if location < 0 {
            let entry = Entry::new(key, value);
            self.data[-(location + 1) as usize] = Some(entry);
            self.ctrl[-(location + 1) as usize] = 0x01;
            self.size += 1;
            self.live += 1;
        }
    }

    /// Takes a key, a closure, and a default value. If the key is in the map, applies the closure
    /// to the corresponding entry's value. If the key is not in the map, creates a new entry with
    /// the provided value.
    ///
    /// Example:
    /// ```rust
    /// use dsa_rust::associative::probing_hash_table::HashMap;
    /// let mut count: HashMap<char, u32> = HashMap::new();
    /// let word: &str = "Hello";
    /// for k in word.chars() {
    ///     count.mut_val_or(k, |x| *x += 1, 1);
    /// }
    /// ```
    pub fn mut_val_or<F>(&mut self, key: K, f: F, default: V)
    where
        F: FnOnce(&mut V),
        K: std::default::Default,
    {
        // Find the appropriate index
        let index = self.find_index(&key);

        if index >= 0 {
            // Found existing key, mutate value
            if let Some(Entry { value, .. }) = self.data[index as usize].as_mut() {
                f(value);
            }
        } else {
            // Create new entry
            self.put(key, default);
        }
    }

    /// Removes and returns an entry from the map for a given key, if it exists in the map.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<Entry<K, V>>
    where
        K: std::borrow::Borrow<Q>,
        Q: Debug + Hash + Eq + ?Sized,
    {
    //pub fn remove(&mut self, key: K) -> Option<Entry<K, V>> {
        let location = self.find_index(key);

        let mut entry = None;
        if location >= 0 {
            //entry = self.data[location as usize].replace(Entry::default());
            entry = self.data[location as usize].take();
            self.ctrl[location as usize] = 0xBB; // Mark with tombstone
            self.live -= 1;
        }
        entry
    }

    /// Consumes self and returns a new map of the same size, but without any tombstones.
    /// Works like a cleanup operation in _O(capacity)_ time because `into_iter` checks all positions.
    pub fn rehash(self) -> Self {
        let mut new = Self::new_with_capacity(self.data.len());
        for (k, v) in self.into_iter() {
            new.put(k, v);
        }
        new
    }

    /// Rebuilds the map to eliminate accumulated tombstones thereby reducing
    /// spatial bloat. This operation runs in _O(n)_ time and is intended for
    /// long-lived maps that have undergone many deletions.
    ///
    /// The operation consumes the existing map and returns a new `HashMap`
    /// with the same live entries.
    pub fn shrink_to_fit(self) -> Self {
        let mut new = Self::new();
        for (k, v) in self.into_iter() {
            new.put(k, v);
        }
        new
    }

    /// Returns an iterator over borrowed values. The resulting values
    /// appear in random order.
    ///
    /// Example use:
    /// ```rust
    /// use dsa_rust::associative::probing_hash_table::HashMap;
    /// let mut count: HashMap<char, u8> = HashMap::new();
    /// let mut v = Vec::new();
    /// for e in count.iter() {
    ///     v.push(*e.0);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            iter: self.data.iter(),
        }
    }

    /// Returns an iterator over borrowed keys only. The keys
    /// appear in random order.
    ///
    /// Example use:
    /// ```text
    /// for e in map.keys() {
    ///     println!("{e}");
    /// }
    /// ```
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys {
            iter: self.data.iter(),
        }
    }

    /// Returns an iterator over borrowed values only. The values
    /// appear in random order.
    ///
    /// Example use:
    /// ```text
    /// for e in map.values() {
    ///     println!("{e}");
    /// }
    /// ```
    pub fn values(&self) -> Values<'_, K, V> {
        Values {
            iter: self.data.iter(),
        }
    }

    // UTILITY FUNCTIONS
    ////////////////////

    /// Finds the correct insertion location using probing
    /// Searches the map for key:
    /// if `>= 0`, overwrite the location and return the old Entry,
    /// if `< 0`, insert new entry at that location, return None
    fn find_index<Q>(&self, key: &Q) -> isize
    where
        K: std::borrow::Borrow<Q>,
        Q: Debug + Hash + Eq + ?Sized,
    {
        let mut i: usize = 1;
        let hash = Self::hash(&key);
        let mut current_index = self.compress(hash);
    
        // Quadratic probing logic
        loop {
            match &self.data[current_index] {
                Some(val) => {
                    if val.key.borrow() == key {
                        return current_index as isize;
                    }
                },
                None => {
                    if self.ctrl[current_index] != 0xBB {
                        return -(current_index as isize + 1)
                    } 
                }
            }
            current_index = (current_index + i.pow(2)) % self.data.len();
            i += 1;
        }
        //while let Some(entry) = &self.data[current_index] {
        //    if entry.key.borrow() == key && self.ctrl[current_index] != 0xBB {
        //        return current_index as isize;
        //    } else {
        //        current_index = (current_index + i.pow(2)) % self.data.len();
        //        i += 1;
        //    }
        //}
        //-(current_index as isize + 1)
    }

    /// Takes a reference to a type `T` and uses Rust's default hasher to return a
    /// 64-bit digest.
    fn hash<T: Hash + Debug + ?Sized>(key: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher); // Hash::hash()
        hasher.finish() as usize // Hasher::finish()
    }

    // Compresses the hash using the MAD algorithm
    fn compress(&self, hash: usize) -> usize {
        (hash.wrapping_mul(self.scale)).wrapping_add(self.shift) % (self.prime) % (self.data.len())
    }

    /// Grows the base (storage) vector to the next prime larger than
    /// double the length of the original vector, rehashes and compresses
    /// hashes for new distribution.
    fn grow(&mut self) {
        // Create a new base vector with_capacity() and resize_with()
        // to ensure all indexes exist, otherwise you could push to an
        // index that doesn't exist causing a panic.
        // NOTE: Vec::resize_with() may result in "hidden allocation"
        // despite description that indicates that the function resizes
        // "in place", initializes new_base with all None values.
        let new_capacity = hash_lib::next_prime(self.data.len() * 2);
        let mut new_base: Vec<Option<Entry<K, V>>> = Vec::with_capacity(new_capacity);
        new_base.resize_with(new_capacity, || None);
        let mut new_ctrl: Vec<u8> = Vec::with_capacity(new_capacity);
        new_ctrl.resize_with(new_capacity, || 0x00);

        //println!("Growing from {} to {}", self.data.len(), new_capacity);

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
                let mut location: usize = ((hash_lib::hash(&v.key)).wrapping_mul(self.scale))
                    //let mut location: usize = ((hash_lib::hash(&v.key().unwrap())).wrapping_mul(self.scale))
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

// Returns a tuple of borrowed key:value pairs (&K, &V)
//pub struct Iter<'a, K, V> {
//    iter: std::slice::Iter<'a, Option<Entry<K, V>>>,
//}
//impl<'a, K, V> Iterator for Iter<'a, K, V>
//    where
//        K: Debug + Hash + PartialEq,
//        V: PartialEq,
//{
//
//    type Item = (&'a K, &'a V);
//
//    fn next(&mut self) -> Option<Self::Item> {
//        for opt_entry in self.iter.by_ref() {
//            if let Some(entry) = opt_entry {
//                return Some((&entry.key, &entry.value));
//            }
//        }
//        None
//    }
//}
pub struct Iter<'a, K, V> {
    iter: std::slice::Iter<'a, Option<Entry<K, V>>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: Debug + Hash + PartialEq,
    V: PartialEq,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find_map(|opt_entry| opt_entry.as_ref().map(|entry| (&entry.key, &entry.value)))
    }
}

// Returns a tuple of owned key:value pairs (K, V)
pub struct IntoIter<K, V> {
    inner: std::iter::Zip<
        std::vec::IntoIter<Option<Entry<K, V>>>,
        std::vec::IntoIter<u8>
    >,
}
impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        for (slot, tag) in self.inner.by_ref() {
            if tag == 1 {
                if let Some(entry) = slot {
                    return Some((entry.key, entry.value));
                }
            }

        }
        None
    }
}

// Allows caller to consume HashMap as an iterator
impl<K, V> IntoIterator for HashMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.data.into_iter().zip(self.ctrl),
        }
    }
}

pub struct Keys<'a, K, V> {
    iter: std::slice::Iter<'a, Option<Entry<K, V>>>,
}
impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.iter.by_ref().flatten().next() {
            return Some(&entry.key);
        }
        None
    }
}

pub struct Values<'a, K, V> {
    iter: std::slice::Iter<'a, Option<Entry<K, V>>>,
}
impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.iter.by_ref().flatten().next() {
            return Some(&entry.value);
        }
        None
    }
}

// Unit tests
/////////////

#[test]
// Generic type test
fn probing_hash_table_test() {
    //Creates a new hash map
    let mut map = HashMap::<&str, u8>::new();

    assert_eq!(map.data.len(), 2);
    assert_eq!(map.ctrl.len(), 2);

    // Illustrates that put() and get() work
    map.put("Peter", 40);
    assert_eq!(map.size, 1);
    assert_eq!(map.data.len(), 5);
    assert_eq!(map.ctrl.len(), 5);

    let fetch = map.get("Peter").unwrap();
    //assert_eq!(*fetch, 40 as u8);
    assert_eq!(*fetch, 40);

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
    assert!(map.contains("Dingus"));

    // Illustrates that put() returns old values and
    // overwrites existing values upon collision...
    let collision = map.put("Peter", 41).unwrap();
    assert_eq!(collision.value, 40_u8);
    //assert_eq!(*collision.value().unwrap(), 40_u8);
    let new_val = map.get("Peter").unwrap();
    //assert_eq!(*new_val, 41 as u8);
    assert_eq!(*new_val, 41);
    // Without increasing the list size
    assert_eq!(map.size, 6);

    // Illustrates that removes entries by key and returns the value
    assert!(map.contains("Dangus"));
    let removed = map.remove(&"Dangus").unwrap();
    assert_eq!(
        removed,
        Entry {
            key: "Dangus",
            value: 27
        }
    );
    assert!(!map.contains("Dangus"));
}

#[test]
// Tests the custom update value function
fn mut_val_test() {
    //Creates a new hash map
    let mut count = HashMap::<char, u8>::new();
    let phrase: &str = "Hello, sickos";

    // Seeds the map with just the characters and a default value
    for char in phrase.chars() {
        count.put(char, 0);
    }

    eprintln!("\nInitial state:");
    count.contents();

    // Iterates through the map again, updating each value based on its occurence
    // NOTE: Can also be used as initial mapping operation with the same code, 
    // but was split here for illustrative purposes
    for char in phrase.chars() {
        count.mut_val_or(char, |x| *x += 1, 1);
    }

    // Pretty-prints the contents of the map
    eprintln!("\nModified:");
    count.contents();

    // Uncomment to trigger debug print
    //assert!(count.is_empty());
}

#[test]
// Tests that the structure is iterable
fn iter_test() {
    //Creates a new hash map of the frequence letters in the given phrase
    let mut map = HashMap::<char, u8>::new();
    let phrase: &str = "Hello, sickos";
    for char in phrase.chars() {
        map.mut_val_or(char, |x| *x += 1, 1);
    }

    eprintln!("\nInitial map state:");
    map.contents();

    // Iterates through the map, pushing entries 
    // to a Vec as tuples returned from the iterator
    eprintln!("\nCompact map: ");
    let mut vec = Vec::new();
    for e in map.iter() {
        vec.push(*e.0);
        eprintln!("{e:?}")
    }

    eprintln!("\nFinal map state:");
    map.contents();

    eprintln!("\nFinal vec state:");
    for e in vec.iter() {
        eprintln!("{:?}", *e)
    }

    // Proves that all entries are still valid
    // Vec contains only keys
    assert!(vec.contains(&'s'));
    assert!(vec.contains(&'i'));
    assert!(vec.contains(&'c'));
    assert!(vec.contains(&'k'));
    assert!(vec.contains(&'o'));

    // Map contains whole entries
    assert!(map.contains(&'s'));
    assert!(map.contains(&'i'));
    assert!(map.contains(&'c'));
    assert!(map.contains(&'k'));
    assert!(map.contains(&'o'));

    eprintln!("\nPrinting just the keys:");
    for key in map.keys() {
        eprintln!("{key}");
    }

    eprintln!("\nPrinting just the values:");
    for value in map.values() {
        eprintln!("{value}");
    }

    // Consume the map and transfer it to a new map
    let mut new_map = HashMap::<char, u8>::new();
    for e in map.into_iter() {
        new_map.put(e.0, e.1);
    }
    // Illegal, map is moved/consumed!
    //assert!(map.is_empty());
    
    // But new_map contains data now
    assert!(new_map.contains(&'s'));
    assert!(new_map.contains(&'i'));
    assert!(new_map.contains(&'c'));
    assert!(new_map.contains(&'k'));
    assert!(new_map.contains(&'o'));

    // Uncomment to trigger debug print
    //assert!(map.is_empty());
}

#[test]
// Tests that into_iter consumes the map, 
// and that rehash and shrink_to_fit delete removed entries
fn rehash_test() {
    //Creates a new hash map
    let mut map = HashMap::<char, u8>::new();
    let word: &str = "Hello, sickos!";
    for k in word.chars() {
        map.mut_val_or(k, |x| *x += 1, 1);
    }

    // Prints initial state of the map
    // and illustrates that values exist
    eprintln!("\nInitial state: ");
    map.contents();
    assert!(map.contains(&','));
    assert!(map.contains(&' '));
    assert!(map.contains(&'!'));
    assert!(map.contains(&'s'));
    assert_eq!(map.len(), 11);
    assert_eq!(map.data.len(), 23);

    // Removes the values
    map.remove(&',');
    map.remove(&' ');
    map.remove(&'!');
    map.remove(&'s');
    map.remove(&'i');
    map.remove(&'c');
    map.remove(&'k');

    // Checks that the values are removed
    eprintln!("\nModified: ");
    map.contents();
    assert!(!map.contains(&','));
    assert!(!map.contains(&' '));
    assert!(!map.contains(&'!'));
    assert!(!map.contains(&'s'));
    assert_eq!(map.len(), 4);
    assert_eq!(map.data.len(), 23);

    // Rehashes to get rid of deleted items
    // map goes out of scope here
    let new_map = map.rehash();

    // Illegal operation! into_iter has consumed map!
    // assert!(map.contains(&'H'));

    // Checks that the values are removed
    // but the capacity remains the same as before
    eprintln!("\nRehash: ");
    new_map.contents();
    assert!(!new_map.contains(&','));
    assert!(!new_map.contains(&' '));
    assert!(!new_map.contains(&'!'));
    assert!(!new_map.contains(&'s'));
    assert_eq!(new_map.len(), 4);
    assert_eq!(new_map.data.len(), 23);

    // Repeats the process for shrink_to_fit
    //
    //Creates a new hash map
    let mut map = HashMap::<char, u8>::new();
    let word: &str = "Hello, sickos!";
    for k in word.chars() {
        map.mut_val_or(k, |x| *x += 1, 1);
    }

    // Prints initial state of the map
    // and illustrates that values exist
    eprintln!("\nInitial state: ");
    map.contents();
    assert!(map.contains(&','));
    assert!(map.contains(&' '));
    assert!(map.contains(&'!'));
    assert!(map.contains(&'s'));
    assert_eq!(map.len(), 11);
    assert_eq!(map.data.len(), 23);

    // Removes the values
    map.remove(&',');
    map.remove(&' ');
    map.remove(&'!');
    map.remove(&'s');
    map.remove(&'i');
    map.remove(&'c');
    map.remove(&'k');

    // Checks that the values are removed
    eprintln!("\nModified: ");
    map.contents();
    assert!(!map.contains(&','));
    assert!(!map.contains(&' '));
    assert!(!map.contains(&'!'));
    assert!(!map.contains(&'s'));
    assert_eq!(map.len(), 4);
    assert_eq!(map.data.len(), 23);

    // Rehashes to get rid of deleted items
    // map goes out of scope here
    let new_map = map.shrink_to_fit();

    // Checks that the values are removed
    // but the capacity remains the same as before
    eprintln!("\nRehash (shrink to fit): ");
    new_map.contents();
    assert!(!new_map.contains(&','));
    assert!(!new_map.contains(&' '));
    assert!(!new_map.contains(&'!'));
    assert!(!new_map.contains(&'s'));
    assert_eq!(new_map.len(), 4);
    assert_eq!(new_map.data.len(), 11);

    // Uncomment to trigger debug print
    //panic!("MANUAL TEST FAILURE");
}


// TODO: This belongs in either an external runner or in an integration test module
pub fn example() {
    //Creates a new hash map
    let mut map = HashMap::<&str, u8>::new();

    let s = format!(
        "Map stats: size: {}, capacity: {}, active entries: {}",
        map.size,
        map.data.len(),
        map.live
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
    if !map.contains("Peter") {
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
        map.live
    );
    for (e, m) in map.data.iter().zip(map.ctrl.iter()) {
        println!("\t{m:>3}: {e:?}")
    }

    // Illustrates removing entries
    println!("\nThere can be only one!");
    names.remove(0);
    for e in names {
        println!("Remove: {}", map.remove(&e).unwrap().key);
    }

    // The final result
    let s = format!(
        "\nMap stats: size: {}, capacity: {}, active entries: {}",
        map.size,
        map.data.len(),
        map.live
    );
    println!("{s}");
    for (e, m) in map.data.iter().zip(map.ctrl.iter()) {
        println!("\t{m:>3}: {e:?}")
    }

    let mut count: HashMap<char, u8> = HashMap::new();
    let word: &str = "The quick brown fox jumps over the lazy dog";
    for k in word.chars() {
        count.mut_val_or(k, |x| *x += 1, 1);
    }
    println!(
        "\n\nString: {word:?}
\nNOTE: There are 28 entries because T != t, and the space literal ' ' is included.
\nMap stats: size: {}, capacity: {}, active entries: {}",
        count.occupied(), // Total number of elements
        count.capacity(),
        count.len() // Active entries

    );
    for (e, m) in count.data.iter().zip(count.ctrl.iter()) {
        println!("\t{m:>3}: {e:?}")
    }

    let l = "=".repeat(s.len());
    println!("\n{l}");

}
