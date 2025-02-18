//////////////////////////////////
// HASHING & COMPRESSION FUNCTIONS
//////////////////////////////////

// STANDARD LIBRARY HASHING
///////////////////////////

// Explores Rust's default hashing functionality
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

/** Takes a reference to a type `T` and uses Rust's default hasher to return a 64-bit digest */
pub fn hash<T: Hash + Debug + ?Sized>(key: &T) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher); // Hash::hash()
    let digest = hasher.finish(); // Hasher::finish()
    digest as usize
}

/** Does the same thing as hash() but feeds individual bytes which produces a
slightly less efficient (and different) digest */
pub fn hash_1(key: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    for e in key.bytes() {
        hasher.write_u8(e)
    }
    let digest = hasher.finish();
    digest
}

// COMPRESSION
//////////////

/** Super simple division compression as `i mod N`;
Produces a deterministic output, works best when `n` (len) is prime */
pub fn division_compression(key: usize, len: usize) -> usize {
    key % len as usize
}

/** Efficient primality test using the 6k ± 1 rule for numbers >3 by trial */
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
#[test]
fn prime_test() {
    // Prime
    assert!(is_prime(7));
    assert!(is_prime(23));
    assert!(is_prime(1103));
    assert!(is_prime(50411));
    assert!(is_prime(120569));
    // Not prime
    assert!(!is_prime(21));
    assert!(!is_prime(39));
    assert!(!is_prime(98));
    assert!(!is_prime(1208));
    assert!(!is_prime(445));
}

/** Finds the next prime by brute force in O(n) time */
//fn next_prime(n: u64) -> u64 {
//    let mut candidate = n + 1;
//    while !is_prime(candidate) {e
//        candidate += 1;
//    }
//    candidate
//}
/** Finds the next prime in O(n/2) time by skipping evens */
pub fn next_prime(n: usize) -> usize {
    if n < 2 {
        return 2;
    }
    if n == 2 {
        return 3;
    }
    let mut candidate = if n % 2 == 0 { n + 1 } else { n + 2 }; // Ensure candidate is odd
    while !is_prime(candidate) {
        candidate += 2; // Skip even numbers
    }
    candidate
}
#[test]
fn next_prime_test() {
    assert_eq!(next_prime(3), 5);
    assert_eq!(next_prime(19), 23);
    assert_eq!(next_prime(8868), 8887);
    assert_eq!(next_prime(117760), 117763);
    assert_eq!(next_prime(1899498), 1899503);
}

use rand::Rng;

/** Implements MAD compression as `[(ai + b) mod p] mod N`
Relies on `is_prime` and `next_prime` functions */
// `c(h(k)) = ((a * h(k) + b) mod p) mod N`
pub fn mad_compression(key: u64, len: usize) -> u64 {
    // Finds a prime >len, starting much larger to ensure even spread
    let p = next_prime(len.pow(3)) as u64;

    let mut rng = rand::rng(); // Thread-local RNG
    let a = rng.random_range(1..=p - 1);
    let b = rng.random_range(0..=p - 1);

    // Raw-dogging the algorithm may cause overflow
    //(((key * a) + b) % p) % len

    // Apply wrapping * and + to prevent overflow
    let wrapped_value = (key.wrapping_mul(a))
        .wrapping_add(b)
        .wrapping_rem(p)
        .wrapping_rem(len as u64);

    wrapped_value
}
pub fn mad_compression_1(hash: usize, prime: usize, scale: usize, shift: usize, capacity: usize) -> usize {
    (hash.wrapping_mul(scale as usize))
        .wrapping_add(shift)
        % (prime)
        % (capacity)
}
