#![allow(dead_code)]

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
