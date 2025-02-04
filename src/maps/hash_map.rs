///////////////////////////
/** Babys first hash map */
///////////////////////////

/** Calculates a bit-shifted hash code;
 * The function initializes a 32-bit hash code to 0,
 * Then it loops over each character in the input string;
 * Each loop adds the bit value of the character to the hash code,
 * then the hash code is bit-shifted left, and the process repeats;
 *
 * Uses wrapping_add() to explicitly handle overflow,
 * this ensures consistent behavior across architectures */
pub fn hash_code(key: &str) -> u32 {
    let mut h: u32 = 0;
    for v in key.bytes() {
        print!("{:08b} -> ", v);
        h = (h << 5) | (h >> 27); 
        h = h.wrapping_add(v as u32);
        println!("{:032b}", h);
    }
    h
}

/** Simple bit shift demonstration;
 * Returns an 8-bit hash value;
 * Loops over a string,
 * prints the bit value of each character,
 * shifts the bit value left,
 * prints the shifted value,
 * adds shifted value to a running total with wrapping,
 * returns the final sum */
pub fn hash_code_2(key: &str) -> u8 {
    let mut h: u8 = 0;
    for mut v in key.bytes() {
        print!("{:08b} -> ", v);
        v = (v << 5) | (v >> 3);
        print!("{:08b} ({v})", v);
        h = h.wrapping_add(v as u8);
        println!(" | {h}");
    }
    h
}

pub fn bit_shift(value: &str) {
    for mut v in value.bytes() {
        print!("{:08b} ({}) -> ", v, v);
        v = (v << 5) | (v >> 3);
        println!("{:08b} ({v})", v);
    }
}

#[test]
fn has_code_test() {
    let v = hash_code("Peter");
    assert_eq!(v, 87317778);

    //let v = hash_code("This block overflows the value");
    //assert_eq!(v, 3862340559);

    let v = hash_code_2("Carl");
    assert_eq!(v, 111);
}

// Explores Rust's default hashing functions
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hasher<T: Hash>(key: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

pub fn default_hasher() {
    let input = "Peter";
    println!("Hash code for '{}': {}", input, hasher(&input));
}
