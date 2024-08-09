#![allow(dead_code)]

use std::task::Wake;

/**
 * This is a sandbox crate for chapter 1 of Data Structures and Algorithm Analysis in Java by Mark Allen Weiss
 */
pub fn recursion(n: i32) {
    // Defines base case
    if n >= 10 {
        // Recursive call to self
        recursion(n / 10);
    }
    // Prints the digit
    println!("{}", n % 10)
}

/** My (iterative) version of a binary search implementation;
 * Takes a sorted array and a key and returns either Some(index) or None */
pub fn binary_search(a: &[i32], key: i32) -> Option<i32> {
    // Sets initial position of the search boundaries
    let mut left = 0;
    let mut right = a.len() - 1;

    // Loops until the search boundaries overlap;
    // If the loop doesn't find the key, the function
    // returns None
    while left <= right {
        let mid = (left + right) / 2;
        if a[mid] == key {
            return Some(mid as i32);
        } else if a[mid] > key {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
        println!("Guess index: {}", &mid);
    }
    return None;
}

#[test]
pub fn binary_search_test() {
    // The target 73 exists at the 37th index
    let target = 73;
    let array: [i32; 39] = [
        1, 4, 5, 6, 10, 12, 16, 21, 23, 24, 25, 27, 31, 32, 33, 35, 37, 39, 40, 41, 42, 43, 45, 47,
        49, 50, 51, 52, 54, 56, 57, 60, 61, 67, 70, 71, 72, 73, 74,
    ];
    let result = match binary_search(&array, target) {
        Some(index) => index,
        None => 0,
    };
    assert_eq!(result, 37)
}

// Cheating with GPT
//pub fn unique_0(a: &Vec<i32>) -> bool {
//    for j in 0..a.len() {
//        for k in j + 1..a.len() {
//            if a[j] == a[k] {
//                println!("{} appears more than once", a[j]);
//                return false;
//            }
//        }
//    }
//    true
//}

// Checks a single array for uniqueness in O(n^2) time
pub fn unique_1(a: &Vec<i32>) -> bool {
    for (j, val) in a.iter().enumerate() {
        let start = (j + 1) as usize;
        if start <= a.len() {
            for k in &a[start ..] {
                if val == k {
                    println!("{val} appears more than once");
                    return false;
                }
            }
        }
    }
    return true;
}

// Checks an array for uniqueness in O(n * log(n)) time
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

// Same as 3 but more elegant (cheating with GPT)
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

pub fn unique_2(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    for j in a.iter() {
        for k in b.iter() {
            if *k == *j {
                return false;
            }
        }
    }
    return true;
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
    for j in 0..a.len() {
        total += a[j];
        avg[j] = total / (j as f32 + 1.0);
    }
    avg
}
