#![allow(dead_code)]

/**
 * This is a sandbox crate for Data Structures and Algorithm Analysis in Java by
 * Tamassia, Goodrich, and Goldwasser */

// Ch 4 - Asymptotic Analysis
/////////////////////////////

// My first stab comparing the elements of two vectors for uniqueness
/** Compares two vectors for unique elements in O(n * m) time */
pub fn unique_0(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    for j in a.iter() {
        for k in b.iter() {
            if *k == *j {
                return false;
            }
        }
    }
    return true;
}

// My first ham-fisted attempt at checking a single array for uniqueness
// Generally viewed as overly complicated an inefficient
/** Checks a single array for uniqueness in O(n^2) time */
pub fn unique_1(a: &Vec<i32>) -> bool {
    for (j, val) in a.iter().enumerate() {
        let start = (j + 1) as usize;
        if start <= a.len() {
            for k in &a[start..] {
                if val == k {
                    println!("{val} appears more than once");
                    return false;
                }
            }
        }
    }
    return true;
}

// Cheating to get a simpler, more elegant function of tgg::unique_1()
pub fn unique_2(a: &Vec<i32>) -> bool {
    for j in 0..a.len() {
        for k in j + 1..a.len() {
            if a[j] == a[k] {
                println!("{} appears more than once", a[j]);
                return false;
            }
        }
    }
    true
}

// A reimplementation with a simple tweak, technically incorrect as this can
// result in OOB panics
/** A reimplementation of tgg::unique_2() that checks an array for uniqueness in
 * O(n * log(n)) time */
pub fn unique_3(a: &Vec<i32>) -> bool {
    a.to_owned().sort();
    for j in 0..a.len() {
        if (j + 1) < a.len() && a[j] == a[j + 1] {
            println!("Found one! {}", a[j]);
            return false;
        }
    }
    true
}

// Another check (cheat) nets an even more elegant solution
pub fn unique_4(a: &Vec<i32>) -> bool {
    a.to_owned().sort();
    for j in 0..a.len() - 1 {
        if a[j] == a[j + 1] {
            println!("Found a duplicate: {}", a[j]);
            return false;
        }
    }
    true
}

/** Calculates a prefix average of an array in O(n^2) time */
pub fn prefix_average_0(a: &Vec<f32>) -> Vec<f32> {
    let mut avg: Vec<f32> = a.clone();
    // i is the index, and a[i] is the value at index i
    for j in 0..a.len() {
        let mut total: f32 = 0.0;
        for k in 0..=j as usize {
            total += a[k];
        }
        avg[j] = total / (j as f32 + 1.0);
        //println!("t: {}, i: {}", total, j)
    }
    avg
}

/** Calculates a prefix average of an array in O(n) time */
pub fn prefix_average_1(a: &Vec<f32>) -> Vec<f32> {
    let mut avg: Vec<f32> = a.clone();
    let mut total: f32 = 0.0;
    // i is the index, and a[i] is the value at index i
    for i in 0..a.len() {
        total += a[i];
        avg[i] = total / (i as f32 + 1.0);
    }
    avg
}
