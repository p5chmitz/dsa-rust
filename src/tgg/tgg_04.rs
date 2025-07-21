#![allow(dead_code)]

/*!
This is a sandbox crate for Data Structures and Algorithm Analysis in Java by
Tamassia, Goodrich, and Goldwasser

// Ch 4 - Asymptotic Analysis
/////////////////////////////
*/

// My first stab comparing the elements of two vectors for uniqueness
/** Compares two vectors for unique elements in O(n * m) time */
pub fn unique_0(a: &[i32], b: &[i32]) -> bool {
    for j in a.iter() {
        for k in b.iter() {
            if *k == *j {
                return false;
            }
        }
    }
    true
}

// My first ham-fisted attempt at checking a single array for uniqueness
// Generally viewed as overly complicated an inefficient
/** Checks a single array for uniqueness in O(n^2) time */
pub fn unique_1(a: &[i32]) -> bool {
    for (j, val) in a.iter().enumerate() {
        let start = j + 1;
        if start <= a.len() {
            for k in &a[start..] {
                if val == k {
                    println!("{val} appears more than once");
                    return false;
                }
            }
        }
    }
    true
}

// Cheating to get a simpler, more elegant function of tgg::unique_1()
pub fn unique_2(a: &[i32]) -> bool {
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
pub fn unique_3(a: &[i32]) -> bool {
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
pub fn unique_4(a: &[i32]) -> bool {
    a.to_owned().sort();
    for j in 0..a.len() - 1 {
        if a[j] == a[j + 1] {
            println!("Found a duplicate: {}", a[j]);
            return false;
        }
    }
    true
}

#[test]
fn uniqueness() {}

/** Calculates a prefix average of an array in O(n) time */
pub fn prefix_average_1(avg: &[f32]) -> Vec<f32> {
    let mut total: f32 = 0.0;
    let mut rtn = Vec::new();
    //for i in 0..avg.len() {
    //    total += avg[i];
    //    avg[i] = total / (i as f32 + 1.0);
    //}
    for (i, val) in avg.iter().enumerate() {
        total += *val;
        rtn.push(total / (i as f32 + 1.0));
    }
    rtn
}
#[test]
fn prefix_avg() {
    let v = &[1.0, 2.0, 3.0, 4.0];
    let avg = &[1.0, 1.5, 2.0, 2.5];
    assert_eq!(prefix_average_1(v), avg);
}
