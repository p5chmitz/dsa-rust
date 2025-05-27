////////////////////////
// FUNSIES & EXPLORATION
////////////////////////
#![allow(dead_code)]
#![allow(unused)]

// BIT SHIFTING
///////////////

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
    let mut hash: u32 = 0;
    for word in key.bytes() {
        print!("{:08b} -> ", word);
        hash = hash.wrapping_add(word as u32);
        hash = (hash << 5) | (hash >> 27);
        println!("{:032b}", hash);
    }
    return hash;
}
#[test]
fn hash_code_test() {
    let v = hash_code("Peter");
    assert_eq!(v, 2794168896);

    //let v = hash_code("This block overflows the value");
    //assert_eq!(v, 3862340559);
}

use crate::maps::hash_lib;

/** Linear probe calculates A[(h(k) + f(i)) mod N] where f(i) == 1
Assumes there is always going to be a valid index */
fn linear_probe(v: &Vec<Option<&str>>, key: &str) -> usize {
    // Example instance values
    let prime = 13;
    let scale = 4;
    let shift = 11;
    let capacity = 7;
    let hash = hash_lib::hash(&key);

    // MAD compression logic
    let mut location =
        (hash.wrapping_mul(scale as usize)).wrapping_add(shift) % (prime) % (capacity);

    assert_eq!(location, 3);

    // The actual probing logic
    let mut i = 1;
    while v[location].is_some() {
        println!("Linear probe: {i}: {location}");
        location = (location + 1) % capacity;
        i += 1;
    }
    location
}
/// Linear probe calculates `A[location] = A[(h(k) + f(i)) mod N]`, where i > 0 and f(i) == i^2
/// Assumes there is always going to be a valid index
pub fn quadratic_probe(v: &Vec<Option<&str>>, key: &str) -> usize {
    // Example instance values
    let prime = 13;
    let scale = 4;
    let shift = 11;
    let capacity = 7;
    let hash = hash_lib::hash(&key);

    // MAD compression logic
    let mut location =
        (hash.wrapping_mul(scale as usize)).wrapping_add(shift) % (prime) % (capacity);

    assert_eq!(location, 3);

    // The actual probing logic
    let mut i: usize = 1;
    while v[location].is_some() {
        location = (location + i.pow(2)) % capacity;
        i += 1;
    }
    location
}

#[test]
fn probe_test() {
    // Peter hashes/compresses to 3
    let vec = vec![
        Some("Steve"),
        Some("Bert"),
        None,
        Some("Peter"),
        Some("Brain"),
        None,
        None,
    ];

    // Skips Brain at 4, proceeds to 5
    assert_eq!(linear_probe(&vec, "Peter"), 5);

    // Skips 3 + 1 % 7 = 4 because vec[4] is Some("Brain")
    // Skips 4 + 4 % 7 = 1 because vec[1] is Some("Bert")
    // Skips 1 + 9 % 7 = 3 (back where we started)
    // Should be 3 + 16 % 7 = 5 because vec[5] is None
    assert_eq!(quadratic_probe(&vec, "Peter"), 5);
}
