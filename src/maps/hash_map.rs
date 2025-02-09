//////////////////
/** Gone hashin */
//////////////////

// HASHING FUNCTIONS
////////////////////

/** Illustrates the concept of bit shifting;
Takes a string a prints the bit and integer value of each character before
performing a bit shift operation and printing the result as bit and integer values */
pub fn bit_shift(value: &str) {
    for mut v in value.bytes() {
        print!("{:08b} ({}) -> ", v, v);
        v = (v << 5) | (v >> 3);
        println!("{:08b} ({v})", v);
    }
}

/** Calculates a bit-shifted hash code;
The function initializes a 32-bit hash code integer to 0,
then loops over each character in the input string;
Each loop adds the next character in the string to the hash code
as an integer value with wrapping; This ensures consistency
across architectures; The next operation in each loop performs
a cyclic bit shift on the hash code, and the process repeats */
pub fn hash_code(key: &str) -> u32 {
    let mut h: u32 = 0;
    for v in key.bytes() {
        print!("{:08b} -> ", v);
        h = h.wrapping_add(v as u32);
        h = (h << 5) | (h >> 27); 
        println!("{:032b}", h);
    }
    h
}
#[test]
fn hash_code_test() {
    let v = hash_code("Peter");
    assert_eq!(v, 2794168896);

    //let v = hash_code("This block overflows the value");
    //assert_eq!(v, 3862340559);
}

// Explores Rust's default hashing functionality
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fmt::Debug;

/** Takes a string slice and prints a 64-bit hash digest */
pub fn hasher_0<T: Hash + Debug + ?Sized>(key: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher); // Hash::hash
    let digest = hasher.finish(); // Hasher::finish
    digest
}

/** Does the same thing as hasher_0 but feeds individual bytes which produces a slightly less efficient (and different) digest */
pub fn hasher_1(key: &str) -> u64 {
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
pub fn compression_0(key: &u64, len: u64) -> u64 {
    key % len
}

/** Efficient primality test using the 6k ± 1 rule for numbers >3 by trial */
fn is_prime(n: u64) -> bool {
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
//    while !is_prime(candidate) {
//        candidate += 1;
//    }
//    candidate
//}
/** Finds the next prime in O(n/2) time by skipping evens */
fn next_prime(n: u64) -> u64 {
    if n < 2 { return 2; }
    if n == 2 { return 3; }
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
pub fn compression_1(key: &u64, len: u64) -> u64 {
    // Finds the next prime >len
    let p = next_prime(len.pow(3));

    let mut rng = rand::rng(); // Thread-local RNG
    let a: u64 = rng.random_range(1..=p - 1);
    let b: u64 = rng.random_range(0..=p - 1);

    // Raw-dogging the algorithm may cause overflow
    //(((key * a) + b) % p) % len

    // Apply wrapping * and + to prevent overflow
    let wrapped_value = (key.wrapping_mul(a))
        .wrapping_add(b)
        .wrapping_rem(p)
        .wrapping_rem(len);

    wrapped_value
}


