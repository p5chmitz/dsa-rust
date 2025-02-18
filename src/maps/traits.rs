use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use rand::Rng;

pub trait AbstractMap<K, V> 
where 
    K: Hash + Debug
{

    // Core functions
    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where 
        K: Borrow<Q>,
        Q: Hash + Debug;

    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove<Q: ?Sized>(&mut self, key: Q) -> Option<V>
    where 
        K: Borrow<Q>,
        Q: Hash + Debug;

    fn contains<Q: ?Sized>(&self, key: &Q) -> bool
    where 
        K: Borrow<Q>,
        Q: Hash + Debug;

    // Utility functions
    //fn iter(&self) -> Iter<K, V>;
    fn grow(&mut self);

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

    fn mad_compression(hash: usize, prime: usize, scale: usize, shift: usize, capacity: usize) -> usize {
        (hash.wrapping_mul(scale as usize))
            .wrapping_add(shift)
            % (prime)
            % (capacity)
    }

}
