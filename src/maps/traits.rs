use rand::Rng;
use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

pub trait AbstractMap<K, V>
where
    K: Hash + Debug,
{
    // Core functions

    /** Returns the value associated with the specified key, if the entry exists */
    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Debug;

    /** Inserts a key-value pair into the map, if the key already exists
    the function overwrites and returns the original value */
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    /** Removes the entry associated with the specified key;
    Only actually removed/cleaned up for chaining solutions,
    open addressing solutions use "defunct" markers that "leak" data */
    fn remove<Q: ?Sized>(&mut self, key: Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Debug;

    /** Returns true if the map contains the specified key */
    fn contains<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Debug;

    /** Returns a vector of entries, can be used to implement
    key_set() and value_set() */
    fn entry_set();

    // Utility functions
    //fn iter(&self) -> Iter<K, V>;

    /** Grows the backing storage based on a specified load factor */
    fn grow(&mut self);

    /** Used by open addressing schemes;
    Takes an initially hashed/compressed value as bucket and a key
    and uses some probing method to find:
    a) Whether the key exists in the map, returns that
    bucket as a positive value
    b) If the key does not exist, returns a negative value
    indicating the next open bucket */
    fn find_index(&self, bucket: usize, key: &K) -> isize;
}

pub trait HashTable {
    fn hash<K: Hash + Debug + ?Sized>(key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher); // Hash::hash()
        let digest = hasher.finish(); // Hasher::finish()
        digest as usize
    }

    fn division_compression(key: usize, len: usize) -> usize {
        key % len as usize
    }

    fn is_prime(n: usize) -> bool {
        // Only positive numbers >1 need apply
        if n < 2 {
            return false;
        }
        // The lil guys get some love
        if n == 2 || n == 3 {
            return true;
        }
        // Skips evens and numbers divisible by 3
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        // 6k ± 1 rule check
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    fn next_prime(n: usize) -> usize {
        if n < 2 {
            return 2;
        }
        if n == 2 {
            return 3;
        }
        let mut candidate = if n % 2 == 0 { n + 1 } else { n + 2 }; // Ensure candidate is odd
        while !Self::is_prime(candidate) {
            candidate += 2; // Skip even numbers
        }
        candidate
    }

    fn set_compression_values(&self, prime: usize, scale: usize, shift: usize);

    fn mad_compression(
        hash: usize,
        prime: usize,
        scale: usize,
        shift: usize,
        capacity: usize,
    ) -> usize {
        (hash.wrapping_mul(scale as usize)).wrapping_add(shift) % (prime) % (capacity)
    }
}
